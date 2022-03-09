#![allow(non_snake_case)]
use crate::clistruct::cli_mod;
use crate::printing::printingFuncs;
use crate::torrentstructs::torrentStructs::{
    self, RtorrentFileInfoStruct, RtorrentPeerStruct, RtorrentTorrentLSPrintStruct,
};
use crate::vechelp::hashvechelp;

use rtorrent::{multicall::d, multicall::f, multicall::p, Download, Error, Result};
use rtorrent_xmlrpc_bindings as rtorrent;
use std::error;
//use std::io::{BufWriter, Write};
use std::thread::spawn;
use structopt::StructOpt;
use url::Url;

mod clistruct;
mod printing;
mod torrentstructs;
mod vechelp;
fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    // Take in args from struct opt
    let cli_input = &cli_mod::Cli::from_args();
    arg_eater(&cli_input)?;
    Ok(())
}

fn arg_eater(inputargs: &cli_mod::Cli) -> std::result::Result<(), Box<dyn error::Error>> {
    if inputargs.addtorrent.is_some() {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-load-start
        todo!();
    }
    if inputargs.incompletedir.is_some() {
        //
        todo!();
    }
    if inputargs.debug {
        unimplemented!();
    }
    if inputargs.cachesize.is_some() {
        todo!();
    }
    if inputargs.exitrtorrent {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-system-shutdown-normal
        todo!();
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
        todo!();
    }

    if inputargs.infopeerbool {
        torrent_peer_info(
            inputargs.rtorrenturl.clone().to_string(),
            inputargs.tempdir.clone(),
            inputargs.torrent.clone(),
        )?;
    }
    if inputargs.infopieces {
        todo!();
    }
    if inputargs.infotracker {
        todo!();
    }
    if inputargs.no_confirm {
        todo!();
    }
    if inputargs.sessioninfo {
        todo!();
    }
    if inputargs.sessionstats {
        todo!();
    }
    if inputargs.reannounce {
        todo!();
    }
    if inputargs.list {
        list_torrents(
            &inputargs.rtorrenturl.clone(),
            inputargs.no_temp_file.clone(),
            inputargs.tempdir.clone(),
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
        /* //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-start
        match hashvechelp::tempfile_finder(
            inputargs.tempdir.clone(),
            inputargs.rtorrenturl.clone().to_string(),
        )? {
            Some(x) => {
                if inputargs.torrent.is_some() {
                    let id: i32 = inputargs.torrent.clone().unwrap()[0].parse()?;
                    let hash: String =
                        hashhelp::id_to_hash(hashvechelp::file_to_vec(x)?, id).unwrap();
                    let mut rtorrent_handler =
                        rtorrent::Server::new(&inputargs.rtorrenturl.clone().to_string());
                    println!("{}", hash);
                }
            }
            None => {
                eprintln!("cannot find tempfile to extract hash from, {} didn't have a tempfile. Try running with -l first.", inputargs.tempdir.clone());
            }
        }*/
    }
    if inputargs.stop {
        //https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-stop
        todo!();
    }
    if inputargs.starttorpaused {
        todo!();
    }
    if inputargs.remove {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        todo!();
    }
    if inputargs.removeAndDelete {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-erase
        todo!();
    }
    if inputargs.starttorunpaused {
        todo!();
    }
    if inputargs.torrent.len() > 0 {
        //todo!();
    }
    if inputargs.utp {
        todo!();
    }
    if inputargs.noutp {
        todo!();
    }
    if inputargs.verify {
        // https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-check-hash
        todo!();
    }
    if inputargs.local_temp_timeout.is_some() {
        todo!();
    }
    Ok(())
}
pub fn list_torrents(
    rtorrenturl: &Url,
    no_tempfile_bool: bool,
    tempdir: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    // instantiate a bunch of stuff to get manipulated later
    let mut torrentList: Vec<RtorrentTorrentLSPrintStruct> = Vec::new();
    let mut vec_of_tor_hashs: Vec<String> = Vec::new();
    let mut path_to_before_rtorrent_remote_temp_file: Option<String> = None;
    if no_tempfile_bool {
        anarchic_index_rtorrent_torrent_list(rtorrenturl.clone(), &mut torrentList);
    } else {
        match hashvechelp::tempfile_finder(tempdir.clone(), rtorrenturl.clone().to_string())? {
            Some(x) => {
                path_to_before_rtorrent_remote_temp_file = Some(x.clone());
                vec_of_tor_hashs = hashvechelp::file_to_vec(x)?;
            }
            None => vec_of_tor_hashs.push(rtorrenturl.clone().to_string()),
        }

        index_rtorrent_torrent_list(rtorrenturl.clone(), &mut torrentList, tempdir.clone())?;
    }
    // very simple way to keep everything in order w/r/t ordering index/hashes
    hashvechelp::derive_vec_of_hashs_from_torvec(&mut vec_of_tor_hashs, &mut torrentList)?;

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
// this accurately recreates transmision-remote's -l command - but the ordering isn't saved - and cannot be considered consistent across multiple calls. E.g. If you delete -t1 this list will all get moved up by 1 - which is not the desired behavior. But it bypasses a lot of application logic to run it like this, so I thought it was worth having the option.
fn anarchic_index_rtorrent_torrent_list(
    rtorrenturl: Url,
    torvec: &mut Vec<RtorrentTorrentLSPrintStruct>,
) {
    // this isn't really ready - I just want easy testing
    // this is the more straight forward version of the
    let mut rtorrent_handler = rtorrent::Server::new(&rtorrenturl.to_string());
    let mut index: i32 = 1;
    //let mut table = Table::new();
    //table.load_preset(NOTHING).set_header(vec![
    //    "ID", "Done", "Have", "ETA", "Up", "Down", "Ratio", "Status", "Name",
    //]);
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
            |(DOWN_RATE, UP_RATE, NAME, RATIO, IS_ACTIVE, LEFT_BYTES, COMPLETED_BYTES, HASHING)| {
                // need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
                let tempTor = torrentStructs::new_torrent_ls_print_maker(
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
                );
                //buffer.write(&tempTor.to_printable_bytes()[..].concat());
                //table.add_row(tempTor.to_vec());
                index += 1;
            },
        );
    //println!("{table}");
}

// this function prints the torrent list - but at the same time keeps the index the same from run to run. It does this by creating a file, located in the directory inputargs.tempdir, with a hashmap to keep track.
fn index_rtorrent_torrent_list(
    rtorrenturl: Url,
    vector_of_torrents: &mut Vec<RtorrentTorrentLSPrintStruct>,
    tempdir: String,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let tempfile = hashvechelp::tempdir_to_tempfile(tempdir, rtorrenturl.clone().to_string());
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
                let tempTor = torrentStructs::new_torrent_ls_print_maker(
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
                );
                vector_of_torrents.push(tempTor);
                index += 1;
            },
        );
    Ok(())
}
// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//// I haven't checked yet, I think there may be an edge case for magnet links yet to be initialized as torrents. Magnet links are meta file -and you basically download the torrent file from peers - and so if you call torrent ls on rtorrent while this is happening - I think there is a chance you may get teh hash of the metafile and not the hash of the eventual torrent.
//https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-d-is-meta

fn torrent_file_information_printer(
    rtorrenturl: String,
    tempdir: String,
    user_selected_torrent_indices: Vec<String>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let mut vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;

    for i in cli_mod::parse_torrents(user_selected_torrent_indices)?.iter() {
        //let dl = Download::from_hash(&handle, &vec_of_tor_hashs[*i as usize]);
        let mut index: i32 = 0;
        let mut vec_of_tor_file_info: Vec<RtorrentFileInfoStruct> = vec![];
        /*let stdout = std::io::stdout();
        let mut locked_stdout = stdout.lock();
        let mut writer = BufWriter::new(locked_stdout);
        */
        let iter = f::MultiBuilder::new(&handle, &vec_of_tor_hashs[*i as usize], None)
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
            Download::from_hash(&handle, &vec_of_tor_hashs[*i as usize]).name()?,
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
    user_selected_torrent_indices: Vec<String>,
) -> std::result::Result<(), Box<dyn error::Error>> {
    let handle = rtorrent::Server::new(&rtorrenturl.clone());
    let mut vec_of_tor_peers: Vec<RtorrentPeerStruct> = vec![];
    let mut vec_of_tor_hashs = to_vec_of_tor_hashes(tempdir.clone(), rtorrenturl.clone())?;
    for i in cli_mod::parse_torrents(user_selected_torrent_indices)?.into_iter() {
        p::MultiBuilder::new(&handle, &vec_of_tor_hashs[i as usize])
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
