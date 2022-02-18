#![allow(non_snake_case)]
use crate::clistruct::cli_mod::Cli;
use rtorrent::multicall::d;
use rtorrent_xmlrpc_bindings as rtorrent;
use structopt::StructOpt;

mod clistruct;
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
        // this isn't really ready - I just want easy testing
        let mut rtorrent_handler = rtorrent::Server::new(&inputargs.rtorrenturl.to_string());
        d::MultiBuilder::new(&mut rtorrent_handler, "default")
            .call(d::HASH)
            .call(d::DOWN_RATE)
            .call(d::UP_RATE)
            .call(d::NAME)
            .invoke()
            .unwrap()
            .into_iter()
            .for_each(|(HASH, DOWN_RATE, UP_RATE, NAME)| {
                println!("{}", format!(" down {}", DOWN_RATE))
            });
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
