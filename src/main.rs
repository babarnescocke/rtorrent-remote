#![allow(non_snake_case)]
//#![feature(str_split_as_str)]
use crate::clistruct::cli_mod::Cli;
use crate::printing::printingFuncs;
use crate::torrentstructs::torrentStructs::{
    self, bytes_to_IEC_80000_13_string, RtorrentFileInfoStruct, RtorrentPeerStruct,
    RtorrentTorrentLSPrintStruct,
};
use crate::vechelp::hashvechelp;
use compound_duration::format_wdhms;
use rtorrent::{multicall::d, multicall::f, multicall::p, multicall::t, Download, File, Server};
use rtorrent_xmlrpc_bindings as rtorrent;
use std::error;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;
use std::thread::spawn;
use structopt::StructOpt;
use text_io::read;
use url::Url;

// trying to move stuff out of main() so things are kind of separated out. argeater() can probably be more sophisticated - my goal was to move arg eater to a separate file entirely - but because of rust's hierarchy rules that's not going to happen.
// There isn't enough error handling - there is propagation. I was kind of ok with the panics - but as I move forward I see problems with it. Previously, I thought this program kind of just one-shots anyway so the panic isn't so bad -however it coredumps everytime it does, doesn't print to stderr etc. That's a point that will need to be majorly overhauled.

///StructOpt Struct and String to Vec<i32>
mod clistruct;
/// some specialized printing functions
mod printing;
/// variety of structs that help processing this info
mod torrentstructs;
/// creates and maintains a 1 indexed list of torrent hashes
mod vechelp;

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    // Take in args from struct opt
    match clistruct::cli_mod::Cli::from_args_safe() {
        Ok(r) => arg_eater(&r)?,
        Err(err) => eprintln!("{}", err),
        // structopts errors are nicely formatted and verbose - so no need for further formatting.
    }
    Ok(())
}

// There is a significant amount of logic that needs to go into pulling the cli args apart. Some of it is merely functional, but some of it requires non-trivial understanding of what is actually being requested by the user.
// In an earlier draft I kind of just logically threaded it out, such that functions were separated more across how a command would be passed in and moved through the program, however; this method reduces overall readability,
// thus I have just gone with a series of if's, for now. The if statement structure here is pretty resilient, you can manipulate multiple things per rtorrent-remote run, and very readable - so its staying for the foreseeable future.

fn arg_eater(inputargs: &Cli) -> std::result::Result<(), Box<dyn error::Error>> {
    if inputargs.addtorrent.is_some() {
        add_torrent(
            inputargs.new_handle(),
            // by definition if inputargs.addtorrent.is_some() it is unwrap-able
            inputargs.addtorrent.as_ref().unwrap().clone(),
        )?;
    }

    if inputargs.exitrtorrent {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-system-shutdown-normal
        exit_rtorrent(
            inputargs.new_handle(),
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.no_confirm.clone(),
        )?;
    }
    // upon research -if and -f do the same thing in transmission-remote hence either will work here.
    if inputargs.files || inputargs.infofilebool {
        torrent_file_information_printer(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
        )?;
    }
    if inputargs.infobool {
        print_torrent_info(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
        )?;
    }

    if inputargs.infopeerbool {
        torrent_peer_info(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
        )?;
    }
    if inputargs.infopieces {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            bitfield
        );
    }

    if inputargs.infotracker {
        print_torrent_trackers(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
        )?;
    }
    if inputargs.mark_files_download.is_some() || inputargs.mark_files_skip.is_some() {
        match &inputargs.mark_files_download {
            Some(_) => select_files_for_dl(
                inputargs.new_handle(),
                inputargs.vec_of_tor_hashes()?,
                inputargs.torrent_string_to_veci32()?,
                inputargs.get_string_to_veci32()?,
            )?,
            None => {}
        }
        match &inputargs.mark_files_skip {
            Some(_) => select_files_for_dl(
                inputargs.new_handle(),
                inputargs.vec_of_tor_hashes()?,
                inputargs.torrent_string_to_veci32()?,
                inputargs.no_get_string_to_veci32()?,
            )?,
            None => {}
        }
    }
    if inputargs.sessioninfo {
        print_session_info(inputargs.new_handle())?;
    }
    if inputargs.sessionstats {
        session_stats(inputargs.new_handle())?;
    }
    if inputargs.reannounce {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-tracker-announce
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            tracker_announce
        );
    }
    if inputargs.list() {
        match inputargs.torrent_string_to_veci32() {
            Ok(x) => list_torrents_end(
                inputargs.new_handle(),
                inputargs.no_temp_file.clone(),
                inputargs.tempdir(),
                Some(x),
                inputargs.rtorrent_time_query,
                inputargs.local_temp_timeout,
            )?,
            Err(_) => list_torrents_end(
                inputargs.new_handle(),
                inputargs.no_temp_file.clone(),
                inputargs.tempdir(),
                None,
                inputargs.rtorrent_time_query,
                inputargs.local_temp_timeout,
            )?,
        }
    }
    if inputargs.labels.is_some() {
        for f in inputargs.torrent_string_to_veci32()? {
            //println!("{:#?}", f);
        }
    }
    if inputargs.bandwidth_high || inputargs.bandwidth_low || inputargs.bandwidth_normal {
        let mut priority = 1;
        let mut sanity_bool = true; // we check if user has given us something silly.
                                    // I am making these separate to catch possible erroneous input
        if inputargs.bandwidth_high {
            priority = 3;
            sanity_bool = false;
        }
        if inputargs.bandwidth_normal {
            priority = 2;
            if !sanity_bool {
                Err("You entered too many bandwidth options - 1 at a time please.")?
            }
            sanity_bool = false;
        }
        if inputargs.bandwidth_low {
            priority = 1;
            if !sanity_bool {
                Err("You entered too many bandwidth options - 1 at a time please.")?
            }
            sanity_bool = false;
        }
        if !sanity_bool {
            set_torrent_priority(
                inputargs.new_handle(),
                inputargs.vec_of_tor_hashes()?,
                inputargs.torrent_string_to_veci32()?,
                priority,
            )?;
        }
    }
    if inputargs.priority_normal.is_some() || inputargs.priority_high.is_some() {
        let priority = 0;
        set_torrent_file_priorty(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            priority,
            inputargs.torrent_string_to_veci32()?,
            inputargs.get_string_to_veci32()?,
        )?;
    }
    if inputargs.movepath.is_some() || inputargs.findpath.is_some() {
        if inputargs.movepath.is_some() && inputargs.findpath.is_some() {
            Err("passed both move and find flags - this is not supported")?;
            std::process::exit(-1);
        } else {
            if inputargs.movepath.is_some() {
                torrent_set_macro!(
                    inputargs.new_handle(),
                    inputargs.vec_of_tor_hashes()?,
                    inputargs.torrent_string_to_veci32()?,
                    set_directory,
                    &inputargs.movepath.as_ref().unwrap()
                );
                torrent_request_macro!(
                    inputargs.new_handle(),
                    inputargs.vec_of_tor_hashes()?,
                    inputargs.torrent_string_to_veci32()?,
                    check_hash
                );
            }
            if inputargs.findpath.is_some() {
                if inputargs.movepath.is_some() {
                    torrent_set_macro!(
                        inputargs.new_handle(),
                        inputargs.vec_of_tor_hashes()?,
                        inputargs.torrent_string_to_veci32()?,
                        set_directory,
                        &inputargs.findpath.as_ref().unwrap()
                    );
                    torrent_request_macro!(
                        inputargs.new_handle(),
                        inputargs.vec_of_tor_hashes()?,
                        inputargs.torrent_string_to_veci32()?,
                        check_hash
                    );
                }
            }
        }
    }
    if inputargs.tracker.is_some() {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-tracker-insert
        todo!();
    }
    if inputargs.trackerrm.is_some() {
        todo!();
    }
    if inputargs.start {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            start
        );
    }
    if inputargs.stop {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            stop
        );
    }
    if inputargs.starttorpaused.is_some() {
        add_torrent_paused(
            inputargs.new_handle(),
            inputargs.starttorpaused.as_ref().unwrap().clone(),
        )?;
    }
    if inputargs.remove {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        // just a stylistic choice but because this doesn't delete any of the underlying torrent data, I don't ask for confirmation.
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            erase
        );
    }
    if inputargs.removeAndDelete {
        remove_and_delete_torrents(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            inputargs.no_confirm.clone(),
        )?;
    }

    if inputargs.verify {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent_string_to_veci32()?,
            check_hash
        );
    }

    Ok(())
}

/// a number of functions really are nearly the same - they only have different calls, eg. start_torrent and stop_torrent really are almost the exact same code - except the request to rtorrent is start/stop.
#[macro_export]
macro_rules! torrent_request_macro {
    ( $handle:expr, $vec_of_tor_hashes:expr, $userselectedtorrentindices:expr, $apicall:ident) => {
        for i in $userselectedtorrentindices.into_iter() {
            Download::from_hash(
                &$handle,
                &hashvechelp::id_to_hash($vec_of_tor_hashes.clone(), i)?,
            )
            .$apicall()?;
        }
    };
}
#[macro_export]
macro_rules! torrent_set_macro {
    ( $handle:expr, $vec_of_tor_hashes:expr, $userselectedtorrentindices:expr, $apicall:ident, $val_to_pass:expr) => {
        for i in $userselectedtorrentindices.into_iter() {
            Download::from_hash(
                &$handle,
                &hashvechelp::id_to_hash($vec_of_tor_hashes.clone(), i)?,
            )
            .$apicall($val_to_pass)?;
        }
    };
}
pub fn select_files_for_dl(
    rs: Server,
    vec_of_tor_hashes: Vec<String>,
    torrent_indices: Vec<i32>,
    get_file_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for t in torrent_indices.into_iter() {
        let dl = Download::from_hash(&rs, &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), t)?);
        for f in get_file_indices.clone().iter() {
            File::from_id(dl.clone(), (*f).into()).set_priority(1)?;
        }
    }
    Ok(())
}

pub fn select_files_for_skip(
    rs: Server,
    vec_of_tor_hashes: Vec<String>,
    torrent_indices: Vec<i32>,
    skip_file_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for t in torrent_indices.into_iter() {
        let dl = Download::from_hash(&rs, &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), t)?);
        for f in skip_file_indices.clone().iter() {
            File::from_id(dl.clone(), (*f).into()).set_priority(0)?;
        }
    }
    Ok(())
}

pub fn set_torrent_priority(
    rs: Server,
    vec_of_tor_hashes: Vec<String>,
    userselectedtorrentindices: Vec<i32>,
    priority: i64,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for t in userselectedtorrentindices.into_iter() {
        Download::from_hash(&rs, &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), t)?)
            .priority_set(priority)?
    }
    Ok(())
}

pub fn add_torrent(
    rs: Server,
    addtorrent: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let x_clone = addtorrent.clone();
    // if the torrent we are trying to add has a host we are going to pass that string to rtorrent for rtorrent to pull.
    match Url::parse(&x_clone) {
        Ok(x_url) => {
            if x_url.has_host() {
                rs.add_tor_started(addtorrent.clone())?;
            }
        }
        Err(_) => {
            let clone = addtorrent.clone();
            rs.add_tor_from_vec_u8_started(std::fs::read(Path::new(&clone).canonicalize()?)?)?;
        }
    };
    Ok(())
}

pub fn add_torrent_paused(
    handle: Server,
    addtorrent: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let x_clone = addtorrent.clone();
    // if the torrent we are trying to add has a host we are going to pass that string to rtorrent for rtorrent to pull.
    match Url::parse(&x_clone) {
        Ok(x_url) => {
            if x_url.has_host() {
                handle.add_tor_paused(addtorrent.clone())?;
            }
        }
        Err(_) => {
            let clone = addtorrent.clone();
            handle.add_tor_from_vec_u8_paused(std::fs::read(Path::new(&clone).canonicalize()?)?)?;
        }
    };
    Ok(())
}

pub fn session_stats(handle: Server) -> std::result::Result<(), Box<dyn error::Error>> {
    let downtotal = handle.down_total()?;
    let uptotal = handle.up_total()?;
    let mut ratio: f64 = 0.0;
    if downtotal != 0 && uptotal != 0 {
        ratio = uptotal as f64 / downtotal as f64;
    }
    let seconds_since_rtorrent_start =
        hashvechelp::unix_time_now()? as i64 - handle.startup_time()?;
    let stdout = stdout();
    let stdoutlock = stdout.lock();
    let mut writer = BufWriter::new(stdoutlock);
    writer.write(
        format!(
            "CURRENT SESSION\n Uploaded: {} \n Downloaded: {} \n Ratio: {} \n Session Time: {} Sec ({})\n Hostname: {}\n",
            bytes_to_IEC_80000_13_string(uptotal),
            bytes_to_IEC_80000_13_string(downtotal),
            format!("{:.3}", ratio),
            seconds_since_rtorrent_start.clone(),
            format_wdhms(seconds_since_rtorrent_start),
            handle.hostname()?
        )
        .as_bytes(),
    )?;
    writer.flush()?;
    Ok(())
}

pub fn print_torrent_trackers(
    rs: Server,
    vec_of_tor_hashes: Vec<String>,
    torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for f in torrent_indices.into_iter() {
        let stdout = stdout();
        let stdoutlock = stdout.lock();
        let mut w = BufWriter::new(stdoutlock);
        t::MultiBuilder::new(&rs, &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), f)?)
            .call(t::URL)
            .call(t::ACTIVTY_TIME_NEXT)
            .call(t::LATEST_SUM_PEERS)
            .invoke()?
            .into_iter()
            .for_each(|(URL, ACTIVTY_TIME_NEXT, LATEST_SUM_PEERS)| {
                let activity_time_next =
                    ACTIVTY_TIME_NEXT - hashvechelp::unix_time_now().unwrap() as i64;
                w.write(
                    format!(
                "Tracker {}\nActive in tier {}\n{}\nAsking for more peers in {} ({} seconds)\n",
                URL,
                String::from("val"),
                format!("{} Peers", LATEST_SUM_PEERS),
                format_wdhms(activity_time_next.clone()),
                activity_time_next
            )
                    .as_bytes(),
                )
                .unwrap();
            });

        w.flush()?;
    }
    Ok(())
}

pub fn print_torrent_info(
    rtorrent_handler: Server,
    vec_of_hashes: Vec<String>,
    torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for f in torrent_indices.into_iter() {
        let hash_of_tor = hashvechelp::id_to_hash(vec_of_hashes.clone(), f.clone())?;
        let clone_of_hash_tor = hash_of_tor.clone();
        let dl = Download::from_hash(&rtorrent_handler, &clone_of_hash_tor);
        let stdout = stdout();
        let stdoutlock = stdout.lock();
        let mut w = BufWriter::new(stdoutlock);
        //
        w.write(b"NAME")?;
        w.write(
            format!(
                "\n Id: {}\n Name: {}\n Hash: {}",
                f,
                dl.name()?,
                hash_of_tor
            )
            .as_bytes(),
        )?;
        w.write(b"\n\nTRANSFER")?;
        w.write(format!("\n State: {}", String::from("Idle")).as_bytes())?;
        w.write(format!("\n Location: {}", dl.base_path()?).as_bytes())?;
        w.write(format!("\n Percent Done: {}", String::from("100%")).as_bytes())?;
        w.write(
            format!(
                "\n ETA: {} ({} Seconds)",
                String::from("whatver"),
                String::from("whatver")
            )
            .as_bytes(),
        )?;
        w.write(format!("\n Download Speed: {} Kbp/s", dl.down_rate()?).as_bytes())?;
        w.write(format!("\n Upload Speed: {}", dl.up_rate()?).as_bytes())?;
        w.write(
            format!(
                "\n Have: {} ({} verified)",
                dl.completed_bytes()?,
                String::from("276.4 MB")
            )
            .as_bytes(),
        )?;
        w.write(format!("\n Availability: {}", String::from("100%")).as_bytes())?;
        w.write(
            format!(
                "\n Total size: {} ({} wanted)",
                String::from("100"),
                String::from("100")
            )
            .as_bytes(),
        )?;
        let dl_total = dl.down_total()?;
        let up_total = dl.up_total()?;
        w.write(format!("\n Downloaded: {}", dl_total).as_bytes())?;
        w.write(format!("\n Uploaded: {}", up_total).as_bytes())?;
        w.write(format!("\n Ratio: {}", dl_total / up_total).as_bytes())?;
        w.write(format!("\n Corrupt DL: {}", String::from(".")).as_bytes())?;
        w.write(
            format!(
                "\n Peers: connected to {}, uploading to {}, downloading from {}",
                String::from("0"),
                String::from("0"),
                String::from("0")
            )
            .as_bytes(),
        )?;

        w.write(b"\n\nHISTORY")?;
        w.write(format!("\n Date added: {}", dl.load_date()?).as_bytes())?;
        //       w.write(format!("\n Date finished: {}", String::from("date")).as_bytes())?;
        //       w.write(format!("\n Date started: {}", String::from("date")).as_bytes())?;
        //       w.write(format!("\n Latest activity: {}", String::from("date")).as_bytes())?;
        //       w.write(format!("\n Downloading Time: {}", String::from("date")).as_bytes())?;
        //       w.write(format!("\n Seeding Time: {}", String::from("date")).as_bytes())?;
        w.write(b"\n\nORIGINS")?;
        w.write(format!("\n Date created: {}", dl.creation_date()?).as_bytes())?;
        /*        w.write(format!("\n Public Torrent: {}", String::from("Yes")).as_bytes())?;
        w.write(
            format!(
                "\n Comment: {}",
                String::from("WebTorrent <https://webtorrent.io>")
            )
            .as_bytes(),
        )?;
        w.write(
            format!(
                "\n Creator: {}",
                String::from("WebTorrent <https://webtorrent.io>")
            )
            .as_bytes(),
        )?;*/
        w.write(format!("\n Piece Count: {}", String::from("1055")).as_bytes())?;
        w.write(format!("\n Piece Size: {}", dl.chunk_size()?).as_bytes())?;
        w.write(b"\n\nLIMITS & BANDWIDTH")?;
        w.write(format!("\n Download Limit: {}", String::from("Unlimited")).as_bytes())?;
        w.write(format!("\n Upload Limit: {}", String::from("Unlimited")).as_bytes())?;
        w.write(format!("\n Ratio Limit: {}", String::from("Unlimited")).as_bytes())?;
        w.write(format!("\n Honor's Session Limits: {}", String::from("stuff")).as_bytes())?;
        w.write(format!("\n Peer Limit: {}", String::from("")).as_bytes())?;
        w.write(format!("\n Bandwidth Priority: {} \n", String::from("someVal")).as_bytes())?;

        w.flush()?;
    }
    Ok(())
}
pub fn print_session_info(server: Server) -> std::result::Result<(), Box<dyn error::Error>> {
    let stdout = stdout();
    let stdoutlock = stdout.lock();
    let mut w = BufWriter::new(stdoutlock);
    w.write(format!("VERSION\n rtorrent API Version: {}\n rtorrent Client Version: {}\n libtorrent Version: {}\n",server.api_version()?,server.client_version()?,server.library_version()?).as_bytes())?;
    w.write(format!("\nCONFIG\n Configuration directory: {}\n Download directory: {}\n Listen ports: {}\n Portforwarding: {}\n uTP enabled: {}\n Distributed hash table enabled: {} \n Local peer discovery enabled: {}\n Peer exchange allowed: {}\n Encryption: {}\n Maximum Memory Cache Size: {}\n", String::from("val"),String::from("val"),server.port()?,String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val")).as_bytes())?;
    w.write(format!("\nLIMITS\n Peer limit: {}\n Default speed ratio limit: {}\n Upload speed limit: {} (Disabled limit {}; Disabled turtle limit: {})\n Download speed limit: {} (Disabled limit {}; Disabled turtle limit: {})", String::from("bal"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val")).as_bytes())?;
    w.write(
        format!(
            "\n\nMISC\n Autostart added torrents: {}\n Delete automatically added torrents: {}\n",
            String::from("yes"),
            String::from("val")
        )
        .as_bytes(),
    )?;
    w.flush()?;
    Ok(())
}
pub fn exit_rtorrent(
    handle: Server,
    url: String,
    no_confirm: bool,
) -> std::result::Result<(), Box<dyn error::Error>> {
    if !no_confirm {
        'userinput0: loop {
            // there is a reason for the verbosity of  "N to not proceed any further" and its because other ways of saying this produce a lexical ambiguity of whether we are exiting rtorrent-remote -or the rtorrent client
            println!("You have selected the option to exit the rtorrent server: {}. If this is correct, please type Y and enter/return. Or N to not proceed any further", url);
            let userinput_string: String = read!("{}\n");
            if userinput_string.clone().eq("Y") {
                break 'userinput0;
            } else if userinput_string.eq("N") {
                std::process::exit(-1);
            }
        }
    }
    handle.exit_rtorrent()?;
    Ok(())
}
// This 3 xmlrpc calls, wrapped into one function. (1) We get the base_filename which will be the fullpath for individual files or base directory for multi-file torrent's, eg: /downloads/ArchLinux_x86_64_11.01.2001.iso or /downloads/CentOS-DVD1/; (2) We ask rtorrent to erase the information from its session, this at least is the .torrent file in its session folder; but depending on rtorrent's settings it may delete the .torrent file in your watch dir. (3) We ask rtorrent to delete the file at the path we captured in step 1, rtorrent runs this in the background - so that if you try to delete lots of files, this doesn't block another call.
pub fn remove_and_delete_torrents(
    handle: Server,
    vec_of_tor_hashes: Vec<String>,
    user_selected_torrent_indices: Vec<i32>,
    no_confirm: bool,
) -> std::result::Result<(), Box<dyn error::Error>> {
    if !no_confirm {
        'userinput1: loop {
            println!("You have selected the option to remove a torrent from rtorrent and delete it from the file system. If this is correct please type Y and enter/return. Or N to not proceed any further");
            let userinput_string: String = read!("{}\n");
            if userinput_string.clone().eq("Y") {
                break 'userinput1;
            } else if userinput_string.eq("N") {
                std::process::exit(-1);
            }
        }
    }
    for i in user_selected_torrent_indices.into_iter() {
        let handle_clone = handle.clone();
        let dl = Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), i)?,
        );
        let remote_file_path = dl.base_filename()?;
        if !remote_file_path.eq("*") || !remote_file_path.eq("/") {
            dl.erase()?;
            handle_clone.delete_path_exec(remote_file_path)?;
        } else {
            Err(format!("Error, the path, {}, you are attempting to delete could be harmful to the underlying system. The torrent hasn't been deleted ", remote_file_path))?
        }
    }
    Ok(())
}

pub fn set_torrent_file_priorty(
    rs: Server,
    vec_of_tor_hashes: Vec<String>,
    priority: i64,
    user_selected_torrent_indices: Vec<i32>,
    user_selected_torrent_files: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    if user_selected_torrent_indices.len() > 1 {
        Err("changing files in multiple torrent's is not safe. Exiting")?;
        std::process::exit(-1);
    }
    for ti in user_selected_torrent_indices.into_iter() {
        let dl = Download::from_hash(
            &rs,
            &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), ti)?,
        );
        for f in user_selected_torrent_files.clone().into_iter() {
            File::from_id(dl.clone(), f.into()).set_priority(priority.clone())?;
        }
    }

    Ok(())
}

fn torrent_file_information_printer(
    handle: Server,
    vec_of_tor_hashs: Vec<String>,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for i in user_selected_torrent_indices.into_iter() {
        //let dl = Download::from_hash(&handle, &vec_of_tor_hashs[*i as usize]);
        let mut index: i32 = 0;
        let mut vec_of_tor_file_info: Vec<RtorrentFileInfoStruct> = vec![];
        /*let stdout = std::io::stdout();
        let mut locked_stdout = stdout.lock();
        let mut writer = BufWriter::new(locked_stdout);
        */
        f::MultiBuilder::new(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
            None,
        )
        .call(f::COMPLETED_CHUNKS)
        .call(f::SIZE_CHUNKS)
        .call(f::PRIORITY)
        .call(f::SIZE_BYTES)
        .call(f::PATH)
        .invoke()?
        .into_iter()
        .for_each(
            |(COMPLETED_CHUNKS, SIZE_CHUNKS, PRIORITY, SIZE_BYTES, PATH)| {
                let temp_Tor_File_Info = torrentStructs::new_file_info_struct_maker(
                    index,
                    COMPLETED_CHUNKS,
                    SIZE_CHUNKS,
                    PRIORITY,
                    SIZE_BYTES,
                    PATH,
                );
                vec_of_tor_file_info.push(temp_Tor_File_Info);
                index += 1;
            },
        );

        printingFuncs::print_torrent_files(
            Download::from_hash(
                &handle,
                &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
            )
            .name()?,
            &vec_of_tor_file_info[..],
        );
    }
    Ok(())
}

//fn file_to_base64_vec_u8(path: String) -> Result<Vec<u8>, Box<dyn error::Error>> {
//    let f = &std::fs::read(path)?;
//    Ok(encode_config(f, STANDARD_NO_PAD).as_bytes().to_vec())
//}
fn torrent_peer_info(
    handle: Server,
    vec_of_tor_hashes: Vec<String>,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let mut vec_of_tor_peers: Vec<RtorrentPeerStruct> = vec![];

    for i in user_selected_torrent_indices.into_iter() {
        p::MultiBuilder::new(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), i)?,
        )
        .call(p::ADDRESS)
        .call(p::IS_ENCRYPTED)
        .call(p::COMPLETED_PERCENT)
        .call(p::DOWN_RATE)
        .call(p::UP_RATE)
        .call(p::CLIENT_VERSION)
        .invoke()?
        .into_iter()
        .for_each(
            |(ADDRESS, IS_ENCRYPTED, COMPLETED_PERCENT, DOWN_RATE, UP_RATE, CLIENT_VERSION)| {
                let temp_peer_info = torrentStructs::new_peer_struct_maker(
                    ADDRESS,
                    IS_ENCRYPTED,
                    COMPLETED_PERCENT,
                    DOWN_RATE,
                    UP_RATE,
                    CLIENT_VERSION,
                );
                vec_of_tor_peers.push(temp_peer_info);
            },
        );

        printingFuncs::print_torrent_peers(&vec_of_tor_peers);
    }
    Ok(())
}
pub fn hash_multicall_to_tmp(
    rs: Server,
    tempdir: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let mut vec_of_tor_hashes: Vec<String> = vec![];
    vec_of_tor_hashes.push(rs.get_endpoint());
    d::MultiBuilder::new(&rs, "default")
        .call(d::HASH)
        .invoke()?
        .into_iter()
        .for_each(|(HASH,)| {
            vec_of_tor_hashes.push(HASH);
        });
    hashvechelp::vec_to_zstd_file(vec_of_tor_hashes, rs.get_endpoint(), tempdir.clone())?;
    Ok(())
}
// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-is-meta

pub fn list_torrents_end(
    rtorrent_handler: Server,
    no_tempfile_bool: bool,
    tempdir: String,
    indices_of_torrents: Option<Vec<i32>>,
    rtime: bool,
    temp_timeout: Option<i64>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    // instantiate a bunch of stuff to get manipulated later
    let mut vec_of_tor_hashes: Vec<String> = vec![];
    let mut torrentList: Vec<RtorrentTorrentLSPrintStruct> = Vec::new();
    let mut path_to_before_rtorrent_remote_temp_file: Option<String> = None;
    // if we don't need a temporary file we can basically just skip ahead
    if no_tempfile_bool {
        let mut index: i32 = 1;
        d::MultiBuilder::new(&rtorrent_handler, "default")
            .call(d::DOWN_RATE)
            .call(d::UP_RATE)
            .call(d::NAME)
            .call(d::RATIO)
            .call(d::IS_ACTIVE)
            .call(d::LEFT_BYTES)
            .call(d::COMPLETED_BYTES)
            .call(d::IS_HASH_CHECKING)
            .invoke()?
            .into_iter()
            .for_each(
                |(
                    DOWN_RATE,
                    UP_RATE,
                    NAME,
                    RATIO,
                    IS_ACTIVE,
                    LEFT_BYTES,
                    COMPLETED_BYTES,
                    HASHING,
                )| {
                    // need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
                    torrentList.push(
                        torrentStructs::RtorrentTorrentLSPrintStruct::new_from_multicall(
                            index,
                            None,
                            DOWN_RATE,
                            UP_RATE,
                            NAME,
                            RATIO,
                            IS_ACTIVE,
                            LEFT_BYTES,
                            COMPLETED_BYTES,
                            HASHING,
                        ),
                    );
                    //buffer.write(&tempTor.to_printable_bytes()[..].concat());
                    //table.add_row(tempTor.to_vec());
                    index += 1;
                },
            );
    } else {
        match hashvechelp::tempfile_finder(
            tempdir.clone(),
            rtorrent_handler.get_endpoint().to_string(),
        )? {
            Some(x) => {
                path_to_before_rtorrent_remote_temp_file = Some(x.clone());
                vec_of_tor_hashes = hashvechelp::zstd_file_to_vec(x)?;
            }
            None => vec_of_tor_hashes.push(rtorrent_handler.get_endpoint()),
        }

        let mut index: i32 = 1;

        d::MultiBuilder::new(&rtorrent_handler, "default")
            .call(d::HASH)
            .call(d::DOWN_RATE)
            .call(d::UP_RATE)
            .call(d::NAME)
            .call(d::RATIO)
            .call(d::IS_ACTIVE)
            .call(d::LEFT_BYTES)
            .call(d::COMPLETED_BYTES)
            .call(d::IS_HASH_CHECKING)
            .invoke()?
            .into_iter()
            .for_each(
                |(
                    HASH,
                    DOWN_RATE,
                    UP_RATE,
                    NAME,
                    RATIO,
                    IS_ACTIVE,
                    LEFT_BYTES,
                    COMPLETED_BYTES,
                    HASHING,
                )| {
                    // need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
                    torrentList.push(
                        torrentStructs::RtorrentTorrentLSPrintStruct::new_from_multicall(
                            index,
                            Some(HASH),
                            DOWN_RATE,
                            UP_RATE,
                            NAME,
                            RATIO,
                            IS_ACTIVE,
                            LEFT_BYTES,
                            COMPLETED_BYTES,
                            HASHING,
                        ),
                    );

                    index += 1;
                },
            );
        // very simple way to keep everything in order w/r/t ordering index/hashes
        hashvechelp::derive_vec_of_hashes_from_torvec(&mut vec_of_tor_hashes, &mut torrentList)?;
    }

    let print = spawn(move || {
        // this decides if we are printing some subset of our list or the whole list, if --torrents were passed, we compare that against the indices in torrentList - printing only the matches.
        match indices_of_torrents {
            Some(x) => printingFuncs::print_torrent_ls(
                torrentList
                    .into_iter()
                    .filter(|i| x.contains(&i.id))
                    .collect(),
            ),
            None => printingFuncs::print_torrent_ls(torrentList),
        }
    });
    if !no_tempfile_bool {
        hashvechelp::vec_to_zstd_file(
            vec_of_tor_hashes,
            rtorrent_handler.get_endpoint(),
            tempdir.clone(),
        )?;
        hashvechelp::delete_file(path_to_before_rtorrent_remote_temp_file)?;
    }
    print.join().unwrap();
    Ok(())
}
