#![allow(non_snake_case)]
use crate::clistruct::cli_mod::Cli;
use crate::torrentstructs::torrentStructs;
use comfy_table::presets::NOTHING;
use comfy_table::*;
use rtorrent::multicall::d;
use rtorrent_xmlrpc_bindings as rtorrent;
use structopt::StructOpt;
use url::Url;

mod clistruct;
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
        if inputargs.no_temp_file {
            anarchic_index_rtorrent_torrent_list(inputargs.rtorrenturl.clone());
        }
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
fn anarchic_index_rtorrent_torrent_list(rtorrenturl: Url) {
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
