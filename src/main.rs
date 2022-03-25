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
use rtorrent::{multicall::d, multicall::f, multicall::p, Download, Server};
use rtorrent_xmlrpc_bindings as rtorrent;
use std::error;
use std::io::{stdout, BufWriter, Read, Write};
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
        Err(err) => eprintln!("There was an issue parsing the commands passed: {}", err),
    }
    Ok(())
}

// There is a significant amount of logic that needs to go into pulling the cli args apart. Some of it is merely functional, but some of it requires non-trivial understanding of what is actually being requested by the user.
// In an earlier draft I kind of just logically threaded it out, such that functions were separated more across how a command would be passed in and moved through the program, however; this method reduces overall readability,
// thus I have just gone with a series of if's, for now. The if statement structure here is pretty resilient, you can manipulate multiple things per rtorrent-remote run, and very readable - so its staying for the foreseeable future.

/// takes all the
fn arg_eater(inputargs: &Cli) -> std::result::Result<(), Box<dyn error::Error>> {
    if inputargs.addtorrent.len() > 0 {
        add_torrent(inputargs.new_handle(), inputargs.addtorrent.clone())?;
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
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.infobool {
        print_torrent_info(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
        )?;
    }

    if inputargs.infopeerbool {
        torrent_peer_info(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.infopieces {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
            bitfield
        );
    }

    if inputargs.infotracker {
        print_torrent_trackers(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.mark_files_download.len() > 0 || inputargs.mark_files_skip.len() > 0 {
        let priority: i64 = 0;
        // might seem a bit odd but these are virtually the same function because of how setting priority is done in rtorrent. Its a simple int, 0 is off, 1 is normal downloading and 2 is high priority.
        set_torrent_file_priorty(
            priority,
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.torrent.clone(),
            inputargs.mark_files_skip.clone(),
        )?;
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
            inputargs.torrent.clone(),
            tracker_announce
        );
    }
    if inputargs.list {
        list_torrents_end(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.no_temp_file.clone(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.labels.len() > 0 {
        todo!();
    }
    if inputargs.bandwidth_high || inputargs.bandwidth_low || inputargs.bandwidth_normal {}
    if inputargs.priority_high.len() > 0 || inputargs.priority_normal.len() > 0 {}
    if inputargs.movepath.len() > 0 {
        todo!();
    }
    if inputargs.findpath.len() > 0 {
        todo!();
    }
    if inputargs.tracker.len() > 0 {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-tracker-insert
        todo!();
    }
    if inputargs.trackerrm.len() > 0 {
        todo!();
    }
    if inputargs.start {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
            start
        );
    }
    if inputargs.stop {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
            stop
        );
    }
    if inputargs.starttorpaused {
        add_torrent_paused()?;
    }
    if inputargs.remove {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
            erase
        );
    }
    if inputargs.removeAndDelete {
        if !inputargs.no_confirm {
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
        remove_and_delete_torrents(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
        )?;
    }

    if inputargs.verify {
        torrent_request_macro!(
            inputargs.new_handle(),
            inputargs.vec_of_tor_hashes()?,
            inputargs.torrent.clone(),
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
    rtorrenturl: String,
    addtorrent: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl);
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
            handle
                .add_tor_from_vec_u8_started(std::fs::read(Path::new(&clone).canonicalize()?)?)?;
        }
    };
    Ok(())
}

pub fn rtorrent_time_up(handle: Server) -> std::result::Result<i64, Box<dyn error::Error>> {
    Ok(hashvechelp::unix_time_now()? as i64 - handle.startup_time()?)
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

pub fn print_torrent_trackers() -> std::result::Result<(), Box<dyn error::Error>> {
    let stdout = stdout();
    let stdoutlock = stdout.lock();
    let mut w = BufWriter::new(stdoutlock);
    w.write(
        format!(
            "Tracker {}: {}\nActive in tier {}\n{}\nAsking for more peers in {} ({} seconds)\n",
            String::from("val"),
            String::from("val"),
            String::from("val"),
            String::from("val"),
            String::from("val"),
            String::from("val")
        )
        .as_bytes(),
    )?;
    w.flush()?;
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
        w.write(format!("\n Id: {}", f).as_bytes())?;
        w.write(format!("\n Name: {}", dl.name()?).as_bytes())?;
        w.write(format!("\n Hash: {}", hash_of_tor).as_bytes())?;
        w.write(format!("\n Magnet: {}", String::from("nbd")).as_bytes())?;
        w.write(format!("\n Labels: {}", String::from("nbd")).as_bytes())?;
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
                String::from("276.4 MB"),
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
        w.write(format!("\n Downloaded: {}", dl.down_total()?).as_bytes())?;
        w.write(format!("\n Uploaded: {}", dl.up_total()?).as_bytes())?;
        w.write(format!("\n Ratio: {}", String::from(".01")).as_bytes())?;
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
        w.write(
            format!(
                "\n Web Seeds: downloading from {} of {} web seeds",
                String::from("0"),
                String::from("1")
            )
            .as_bytes(),
        )?;
        w.write(b"\n\nHISTORY")?;
        w.write(format!("\n Date added: {}", String::from("date")).as_bytes())?;
        w.write(format!("\n Date finished: {}", String::from("date")).as_bytes())?;
        w.write(format!("\n Date started: {}", String::from("date")).as_bytes())?;
        w.write(format!("\n Latest activity: {}", String::from("date")).as_bytes())?;
        w.write(format!("\n Downloading Time: {}", String::from("date")).as_bytes())?;
        w.write(format!("\n Seeding Time: {}", String::from("date")).as_bytes())?;
        w.write(b"\n\nORIGINS")?;
        w.write(
            format!(
                "\n Date created: {}",
                String::from("Thu Mar 30 16:30:01 2017")
            )
            .as_bytes(),
        )?;
        w.write(format!("\n Public Torrent: {}", String::from("Yes")).as_bytes())?;
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
        )?;
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
    w.write(format!("\nCONFIG\n Configuration directory: {}\n Download directory: {}\n Listen port: {}\n Portforwarding: {}\n uTP enabled: {}\n Distributed hash table enabled: {} \n Local peer discovery enabled: {}\n Peer exchange allowed: {}\n Encryption: {}\n Maximum Memory Cache Size: {}\n", String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val"),String::from("val")).as_bytes())?;
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
/// so I sat with this a bit -- the rtorrent API has some rough edges; and deleting files from the file system is complicated by the fact that there is
pub fn remove_and_delete_torrents(
    handle: Server,
    vec_of_tor_hashes: Vec<String>,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    for i in user_selected_torrent_indices.into_iter() {
        println!(
            "{}",
            Download::from_hash(
                &handle,
                &hashvechelp::id_to_hash(vec_of_tor_hashes.clone(), i)?
            )
            .base_filename()?
        );
        //println!("Successfully Erased 1 Torrent");
    }
    Ok(())
}

pub fn set_torrent_file_priorty(
    priority: i64,
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
    user_selected_torrent_files: Vec<i64>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    let val = user_selected_torrent_indices.into_iter();
    if val.clone().len() > 1 {
        println!("More than 1 torrent was passed to rtorrent-remote, to manipulate multiple files, this functionality is not supported. Exiting." );
        std::process::exit(-1);
    }
    for i in val.into_iter() {
        let _torrent = Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        );
        for f in user_selected_torrent_files.clone().into_iter() {
            //    let file = File::new(torrent, f);
            //    file.set_priority(priority)?;
        }
    }
    Ok(())
}

// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-is-meta

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
fn to_vec_of_tor_hashes(
    tempdir: String,
    rtorrenturl: String,
) -> std::result::Result<Vec<String>, Box<dyn error::Error>> {
    match hashvechelp::tempfile_finder(tempdir.clone(), rtorrenturl.clone())? {
        Some(x) => Ok(hashvechelp::file_to_vec(x)?),
        None => Err(format!(
            "There is no tempfile in {}, run rtorrent-remote -l first",
            tempdir.clone()
        ))?,
    }
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

pub fn list_torrents_end(
    rtorrent_handler: Server,
    mut vec_of_tor_hashes: Vec<String>,
    rtorrenturl: String,
    no_tempfile_bool: bool,
    tempdir: String,
    indices_of_torrents: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    // instantiate a bunch of stuff to get manipulated later
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
            .invoke()
            .unwrap()
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
                    torrentList.push(torrentStructs::new_torrent_ls_print_maker(
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
                    ));
                    //buffer.write(&tempTor.to_printable_bytes()[..].concat());
                    //table.add_row(tempTor.to_vec());
                    index += 1;
                },
            );
    } else {
        match hashvechelp::tempfile_finder(tempdir.clone(), rtorrenturl.clone().to_string())? {
            Some(x) => {
                path_to_before_rtorrent_remote_temp_file = Some(x.clone());
                vec_of_tor_hashes = hashvechelp::file_to_vec(x)?;
            }
            None => vec_of_tor_hashes.push(rtorrenturl.clone().to_string()),
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
                    torrentList.push(torrentStructs::new_torrent_ls_print_maker(
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
                    ));

                    index += 1;
                },
            );
        // very simple way to keep everything in order w/r/t ordering index/hashes
        hashvechelp::derive_vec_of_hashs_from_torvec(&mut vec_of_tor_hashes, &mut torrentList)?;
    }

    let print = spawn(move || {
        // Ideally I would like to setup print_torrent_ls to take any given slice of torrents to print - eg it could print everything or t1-10 or t1,4,6 etc. So I chose to use a slice here.
        //need to make a sorter so that torrentList vec is sorted by index number
        if indices_of_torrents.is_empty() {
            printingFuncs::print_torrent_ls(torrentList);
        } else {
            printingFuncs::print_torrent_ls(
                torrentList
                    .into_iter()
                    .filter(|i| indices_of_torrents.contains(&i.id))
                    .collect(),
            );
        }
    });
    if !no_tempfile_bool {
        hashvechelp::vec_to_file(vec_of_tor_hashes, rtorrenturl.to_string(), tempdir.clone())?;
        hashvechelp::delete_old_vecfile(path_to_before_rtorrent_remote_temp_file)?;
    }
    print.join().unwrap();
    Ok(())
}
