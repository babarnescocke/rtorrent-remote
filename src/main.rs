#![allow(non_snake_case)]
//#![feature(str_split_as_str)]
use crate::clistruct::cli_mod;
use crate::printing::printingFuncs;
use crate::torrentstructs::torrentStructs::{
    self, bytes_to_IEC_80000_13_string, RtorrentFileInfoStruct, RtorrentPeerStruct,
    RtorrentTorrentLSPrintStruct,
};
use crate::vechelp::hashvechelp;
use rtorrent::{multicall::d, multicall::f, multicall::p, Download, File};
use rtorrent_xmlrpc_bindings as rtorrent;
use std::error;
use std::io::{stdout, BufWriter, Write};
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
    arg_eater(&cli_mod::Cli::from_args())?;
    Ok(())
}

/// There is a significant amount of logic that needs to go into pulling the cli args apart. Some of it is merely functional, but some of it requires non-trivial understanding of what is actually being requested by the user.
/// In an earlier draft I kind of just logically threaded it out, such that functions were separated more across how a command would be passed in and moved through the program, however; this method reduces overall readability,
/// thus I have just gone with a series of if's, for now. I didn't do testing with transmission-remote before the project - just the man pages and my recollections of using it,
/// my recollection was that firing one command didn't effect or negate your ability run other commands - and that it was pretty ductile in taking commands: you could do all kinds of commands and in whatever order,
/// and it would return what you wanted - so I strove for that. The if statement structure here is pretty resilient and very readable - so its staying for the foreseeable future.
///
fn arg_eater(inputargs: &cli_mod::Cli) -> std::result::Result<(), Box<dyn error::Error>> {
    match &inputargs.addtorrent {
        Some(x) => {
            let handle = rtorrent::Server::new(&inputargs.rtorrenturl.clone().to_string());
            let x_clone = x.clone();
            // if the torrent we are trying to add has a host we are going to pass that string to rtorrent for rtorrent to pull.
            if Url::parse(&x_clone).unwrap().has_host() {
                handle.add_tor_started_exec(x.to_string())?;
            // else its a local file we are going to base64 encode it and send it as raw bytes.
            } else {
                println!("{}", x);
            }
        }
        None => {}
    }

    if inputargs.incompletedir.is_some() {
        //
        todo!();
    }
    if inputargs.debug {
        unimplemented!();
    }

    if inputargs.exitrtorrent {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-system-shutdown-normal
        if !inputargs.no_confirm {
            'userinput0: loop {
                // there is a reason for the verbosity of  "N to not proceed any further" and its because other ways of saying this produce a lexical ambiguity of whether we are exiting rtorrent-remote -or the rtorrent client
                println!("You have selected the option to exit the rtorrent server: {}. If this is correct please type Y and enter/return. Or N to not proceed any further", inputargs.rtorrenturl.clone().to_string());
                let userinput_string: String = read!("{}\n");
                if userinput_string.clone().eq("Y") {
                    break 'userinput0;
                } else if userinput_string.eq("N") {
                    std::process::exit(-1);
                }
            }
        }
        let handle = rtorrent::Server::new(&inputargs.rtorrenturl.clone().to_string());
        handle.exit_rtorrent()?;
    }
    // upon research -if and -f do the same thing in transmission-remote hence either will work here.
    if inputargs.files || inputargs.infofilebool {
        torrent_file_information_printer(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.infobool {
        torrent_time_up(inputargs.rtorrenturl.clone().to_string())?;
    }

    if inputargs.infopeerbool {
        torrent_peer_info(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.infopieces {
        print_torrent_bitfield(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }

    if inputargs.infotracker {
        todo!();
    }
    if inputargs.mark_files_download.len() > 0 || inputargs.mark_files_skip.len() > 0 {
        let priority: i64 = 0;
        // might seem a bit odd but these are virutally the same function because of how setting priority is done in rtorrent. Its a simple int, 0 is off, 1 is normal downloading and 2 is high priority.
        set_torrent_file_priorty(
            priority,
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.torrent.clone(),
            inputargs.mark_files_skip.clone(),
        )?;
    }
    if inputargs.sessioninfo {
        todo!();
    }
    if inputargs.sessionstats {
        let handle = rtorrent::Server::new(&inputargs.rtorrenturl.clone().to_string());
        let downtotal = handle.down_total()?;
        let uptotal = handle.up_total()?;
        let mut ratio: f64 = 0.0;
        if downtotal != 0 && uptotal != 0 {
            ratio = uptotal as f64 / downtotal as f64;
        }
        let seconds = 0;
        //the below prevents us from having to make syscall per line of output.
        let stdout = stdout();
        let stdoutlock = stdout.lock();
        let mut writer = BufWriter::new(stdoutlock);
        writer.write(
            format!("CURRENT SESSION\n Uploaded: {} \n Downloaded: {} \n Ratio: {} \n Duration: {} \n Hostname: {}\n", bytes_to_IEC_80000_13_string(uptotal), bytes_to_IEC_80000_13_string(downtotal), format!("{:.3}", ratio), seconds.to_string(), handle.hostname()?).as_bytes()
        )?;
        writer.flush()?;
    }
    if inputargs.reannounce {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-tracker-announce
        reannounce_torrents(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.list {
        list_torrents_end(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.no_temp_file.clone(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.labels.is_some() {
        todo!();
    }
    if inputargs.movepath.is_some() {
        todo!();
    }
    if inputargs.findpath.is_some() {
        todo!();
    }
    if inputargs.tracker.is_some() {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-tracker-insert
        todo!();
    }
    if inputargs.trackerrm.is_some() {
        todo!();
    }
    if inputargs.start {
        start_torrents(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.stop {
        stop_torrents(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.starttorpaused {
        todo!();
    }
    if inputargs.remove {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        remove_torrents(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
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
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;

        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        /*        remove_torrents(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;*/
    }

    if inputargs.verify {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-check-hash
        //check_torrents(
        //    inputargs.rtorrenturl.clone().to_string(),
        //    inputargs.tempdir.clone(),
        //    inputargs.torrent.clone(),
        //)?;
        #[macro_use]
        torrent_request_macro!(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            ".check_hash()"
        )?;
    }
    if inputargs.local_temp_timeout.is_some() {
        todo!();
    }
    Ok(())
}

// If I know how long rtorrent is up a lot of questions can be answered - however its a surprisingly inaccessible number to reach. For instance, my rtorrent doesn't report it as a method that I can ask for. Supposedly it is a stable part of the /proc/ pseudo-fs - but podman, at least, overwrites that time to be *now* whenever you query it. ps does have -etime,-etimes but ps is not as uniform across distributions as I might like.
pub fn torrent_time_up(rtorrenturl: String) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    println!("{}", handle.up_time_exec()?);
    Ok(())
}
/// a number of functions really are nearly the same - they only have different calls, eg. start_torrent and stop_torrent really are almost the exact same code - except the request to rtorrent is start/stop.

macro_rules! torrent_request_macro {
    ($rtorrenturl: expr, $tempdir: expr, &user_selected_torrent_indices: expr, $api: literal) => {

        pub fn (&self) -> std::result::Result<(), Box<dyn error::Error>> {
            let handle = rtorrent::Server::new(&$rtorrenturl);
            let vec_of_tor_hashs = to_vec_of_tor_hashes($tempdir.clone(), $rtorrenturl.clone())
            for i in user_selected_torrent_indices.into_iter() {
                Download::from_hash(&handle, &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?).$api?;
            }
            Ok(())
        }

}
}
/*torrent_request_macro!(
    start,
    start_tor,
    rtorrenturl,
    tempdir,
    &user_selected_torrent_indices,
    start()
);*/

pub fn reannounce_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        )
        .tracker_announce()?;
        println!("Successfully Started Re-announcing 1 Torrent");
    }
    Ok(())
}
pub fn print_torrent_bitfield(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        println!(
            "{}",
            Download::from_hash(
                &handle,
                &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?
            )
            .bitfield()?
        );
    }
    Ok(())
}
pub fn check_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        )
        .check_hash()?;
        println!("Successfully Started Checking 1 Torrent");
    }
    Ok(())
}
pub fn stop_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        )
        .stop()?;
        println!("Successfully Stopped 1 Torrent");
    }
    Ok(())
}
pub fn start_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        )
        .start()?;
        println!("Successfully Started 1 Torrent");
    }
    Ok(())
} // so I sat with this a bit -- the rtorrent API has some rough edges; and deleting files from the file system is complicated by the fact that there is
pub fn remove_and_delete_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        println!(
            "{}",
            Download::from_hash(
                &handle,
                &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?
            )
            .base_filename()?
        );
        //println!("Successfully Erased 1 Torrent");
    }
    Ok(())
}
pub fn remove_torrents(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        Download::from_hash(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
        )
        .erase()?;
        println!("Successfully Erased 1 Torrent");
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
        let torrent = Download::from_hash(
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
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;

    for i in user_selected_torrent_indices.into_iter() {
        //let dl = Download::from_hash(&handle, &vec_of_tor_hashs[*i as usize]);
        let mut index: i32 = 0;
        let mut vec_of_tor_file_info: Vec<RtorrentFileInfoStruct> = vec![];
        /*let stdout = std::io::stdout();
        let mut locked_stdout = stdout.lock();
        let mut writer = BufWriter::new(locked_stdout);
        */
        let iter = f::MultiBuilder::new(
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
fn torrent_peer_info(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let mut vec_of_tor_peers: Vec<RtorrentPeerStruct> = vec![];
    let vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in user_selected_torrent_indices.into_iter() {
        p::MultiBuilder::new(
            &handle,
            &hashvechelp::id_to_hash(vec_of_tor_hashs.clone(), i)?,
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
    rtorrenturl: String,
    no_tempfile_bool: bool,
    tempdir: String,
    indices_of_torrents: Vec<i32>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    // instantiate a bunch of stuff to get manipulated later
    let mut torrentList: Vec<RtorrentTorrentLSPrintStruct> = Vec::new();
    let mut vec_of_tor_hashs: Vec<String> = Vec::new();
    let mut path_to_before_rtorrent_remote_temp_file: Option<String> = None;
    // if we don't need a temporary file we can basically just skip ahead
    if no_tempfile_bool {
        let mut rtorrent_handler = rtorrent::Server::new(&rtorrenturl.clone());
        let mut index: i32 = 1;
        d::MultiBuilder::new(&mut rtorrent_handler, "default")
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
                vec_of_tor_hashs = hashvechelp::file_to_vec(x)?;
            }
            None => vec_of_tor_hashs.push(rtorrenturl.clone().to_string()),
        }

        let tempfile = hashvechelp::tempdir_to_tempfile(tempdir.clone(), rtorrenturl.to_string());
        // if tempfile is empty we will create one
        //if tempfile?.is_some() {}
        let mut rtorrent_handler = rtorrent::Server::new(&rtorrenturl.to_string());
        let mut index: i32 = 1;

        d::MultiBuilder::new(&mut rtorrent_handler, "default")
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
        hashvechelp::derive_vec_of_hashs_from_torvec(&mut vec_of_tor_hashs, &mut torrentList)?;
    }

    let print = spawn(move || {
        // Ideally I would like to setup print_torrent_ls to take any given slice of torrents to print - eg it could print everything or t1-10 or t1,4,6 etc. So I chose to use a slice here.
        //need to make a sorter so that torrentList vec is sorted by index number

        printingFuncs::print_torrent_ls(&torrentList[..]);
    });

    hashvechelp::vec_to_file(vec_of_tor_hashs, rtorrenturl.to_string(), tempdir.clone())?;
    hashvechelp::delete_old_vecfile(path_to_before_rtorrent_remote_temp_file)?;
    print.join().unwrap();
    Ok(())
}
