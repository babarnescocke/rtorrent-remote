#![allow(non_snake_case)]
use crate::clistruct::cli_mod::Cli;
use crate::hashmap::hashhelp;
use crate::torrentstructs::torrentStructs::{self, RtorrentTorrentPrint};
use comfy_table::presets::NOTHING;
use comfy_table::*;
use rtorrent::multicall::d;
use rtorrent_xmlrpc_bindings as rtorrent;
use std::collections::HashMap;

use std::thread::spawn;
use structopt::StructOpt;
use url::Url;

mod clistruct;
mod hashmap;
mod torrentstructs;
fn main() {
    // Take in args from struct opt
    let cli_input = &Cli::from_args();
    arg_eater(&cli_input);
}

fn arg_eater(inputargs: &Cli) {
    if inputargs.addtorrent.is_some() {
        todo!();
    }
    if inputargs.incompletedir.is_some() {
        todo!();
    }
    if inputargs.debug {
        unimplemented!();
    }
    if inputargs.cachesize.is_some() {
        todo!();
    }
    if inputargs.exitrtorrent {
        todo!();
    }
    if inputargs.files {
        todo!();
    }
    if inputargs.infobool {
        todo!();
    }
    if inputargs.infofilebool {
        todo!();
    }
    if inputargs.infopeerbool {
        todo!();
    }
    if inputargs.infopieces {
        todo!();
    }
    if inputargs.infotracker {
        todo!();
    }
    if inputargs.sessioninfo {
        todo!();
    }
    if inputargs.sessionstats {
        todo!();
    }
    if inputargs.list {
        let mut torrentList: Vec<RtorrentTorrentPrint> = Vec::new();
        let mut hashmap: HashMap<String, i32> = HashMap::new();
        let mut path_to_before_rtorrent_remote_temp_file: Option<String> = None;
        if inputargs.no_temp_file {
            anarchic_index_rtorrent_torrent_list(inputargs.rtorrenturl.clone(), &mut torrentList);
        } else {
            match hashhelp::tempfile_finder(
                inputargs.tempdir.clone(),
                inputargs.rtorrenturl.clone().to_string(),
            ) {
                Some(x) => {
                    path_to_before_rtorrent_remote_temp_file = Some(x.clone());
                    hashmap = hashhelp::file_to_hashmap(x);
                }
                None => {
                    hashmap = HashMap::new();
                }
            }

            //let mut hashmap: HashMap<String, i32> = HashMap::new();
            index_rtorrent_torrent_list(
                inputargs.rtorrenturl.clone(),
                &mut torrentList,
                inputargs.tempdir.clone(),
            );
        }
        // very simple way to keep everything in order w/r/t ordering index/hashes
        derive_hashmap_from_torvec(&mut hashmap, &mut torrentList);

        let print = spawn(move || {
            // Ideally I would like to setup torrent_ls_printer to take any given slice of torrents to print - eg it could print everything or t1-10 or t1,4,6 etc. So I chose to use a slice here.
            //need to make a sorter so that torrentList vec is sorted by index number

            torrent_ls_printer(&torrentList[..]);
        });

        hashhelp::hashmap_to_file(
            hashmap,
            inputargs.rtorrenturl.to_string(),
            inputargs.tempdir.clone(),
        );
        hashhelp::delete_old_hashmap(path_to_before_rtorrent_remote_temp_file).unwrap();
        print.join().unwrap();
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
        todo!();
    }
    if inputargs.trackerid.is_some() {
        todo!();
    }
    if inputargs.start {
        todo!();
    }
    if inputargs.stop {
        todo!();
    }
    if inputargs.starttorpaused {
        todo!();
    }
    if inputargs.remove {
        todo!();
    }
    if inputargs.removeAndDelete {
        todo!();
    }
    if inputargs.starttorunpaused {
        todo!();
    }
    if inputargs.torrent.is_some() {
        todo!();
    }
    if inputargs.utp {
        todo!();
    }
    if inputargs.noutp {
        todo!();
    }
    if inputargs.verify {
        todo!();
    }
}

/// this accurately recreates transmision-remote's -l command - but the ordering isn't saved - and cannot be considered consistent across multiple calls. E.g. If you delete -t1 this list will all get moved up by 1 - which is not the desired behavior. But it bypasses a lot of application logic to run it like this, so I thought it was worth having the option.
fn anarchic_index_rtorrent_torrent_list(rtorrenturl: Url, torvec: &mut Vec<RtorrentTorrentPrint>) {
    // this isn't really ready - I just want easy testing
    /// this is the more straight forward version of the
    let mut rtorrent_handler = rtorrent::Server::new(&rtorrenturl.to_string());
    let mut index: i32 = 1;
    let mut table = Table::new();
    table.load_preset(NOTHING).set_header(vec![
        "ID", "Done", "Have", "ETA", "Up", "Down", "Ratio", "Status", "Name",
    ]);
    d::MultiBuilder::new(&mut rtorrent_handler, "default")
        .call(d::DOWN_RATE)
        .call(d::UP_RATE)
        .call(d::NAME)
        .call(d::RATIO)
        .call(d::IS_ACTIVE)
        .call(d::LEFT_BYTES)
        .call(d::COMPLETED_BYTES)
        .invoke()
        .unwrap()
        .into_iter()
        .for_each(
            |(DOWN_RATE, UP_RATE, NAME, RATIO, IS_ACTIVE, LEFT_BYTES, COMPLETED_BYTES)| {
                /// need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
                let tempTor = torrentStructs::new_torrent_print_maker(
                    index,
                    None,
                    DOWN_RATE,
                    UP_RATE,
                    NAME,
                    RATIO,
                    IS_ACTIVE,
                    LEFT_BYTES,
                    COMPLETED_BYTES,
                );
                //buffer.write(&tempTor.to_printable_bytes()[..].concat());
                table.add_row(tempTor.to_vec());
                index += 1;
            },
        );
    println!("{table}");
}

// this function prints the torrent list - but at the same time keeps the index the same from run to run. It does this by creating a file, located in the directory inputargs.tempdir, with a hashmap to keep track.
fn index_rtorrent_torrent_list(
    rtorrenturl: Url,
    vector_of_torrents: &mut Vec<RtorrentTorrentPrint>,
    tempdir: String,
) {
    let mut tempfile = hashhelp::tempdir_to_tempfile(tempdir, rtorrenturl.clone().to_string());
    // if tempfile is empty we will create one
    if tempfile.is_some() {}
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
        .invoke()
        .unwrap()
        .into_iter()
        .for_each(
            |(HASH, DOWN_RATE, UP_RATE, NAME, RATIO, IS_ACTIVE, LEFT_BYTES, COMPLETED_BYTES)| {
                /// need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
                let tempTor = torrentStructs::new_torrent_print_maker(
                    index,
                    Some(HASH),
                    DOWN_RATE,
                    UP_RATE,
                    NAME,
                    RATIO,
                    IS_ACTIVE,
                    LEFT_BYTES,
                    COMPLETED_BYTES,
                );
                vector_of_torrents.push(tempTor);
                index += 1;
            },
        );
}

fn torrent_ls_printer(slice_of_torrent_structs: &[RtorrentTorrentPrint]) {
    let mut table = Table::new();
    table.load_preset(NOTHING).set_header(vec![
        "ID", "Done", "Have", "ETA", "Up", "Down", "Ratio", "Status", "Name",
    ]);
    for tempTor in slice_of_torrent_structs.into_iter() {
        table.add_row(tempTor.to_vec());
    }
    println!("{}", table);
}
/// a function that just walks torrentlist and if the torrent is in the hashmap we just keep walking - else we add that item.
pub fn derive_hashmap_from_torvec(
    hashmap: &mut HashMap<String, i32>,
    torvec: &mut Vec<RtorrentTorrentPrint>,
) {
    for f in torvec.iter_mut() {
        match f.hash.clone() {
            Some(y) => {
                if !hashmap.contains_key(&y) {
                    let hashmap_return = hashmap.insert(y, f.id);
                    if hashmap_return.is_some() {
                        todo!();
                    }
                } else {
                    f.id = hashmap.get(&y).unwrap().clone();
                }
            }
            None => {
                println!("uh-oh- torrentvec has no hash");
            }
        }
    }
}
