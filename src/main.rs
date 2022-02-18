#![allow(non_snake_case)]
use crate::clistruct::cli_mod::Cli;
use rtorrent::multicall::d;
use rtorrent_xmlrpc_bindings as rtorrent;
use structopt::StructOpt;

mod clistruct;
fn main() {
    let cli_input = &Cli::from_args();
    let rtorrent_handler = rtorrent::Server::new(&cli_input.rtorrenturl.to_string());
    d::MultiBuilder::new(&rtorrent_handler, "default")
        .call(d::NAME)
        .call(d::RATIO)
        .invoke()
        .into_iter()
        .for_each(|name| {
            println!("{:?}", name);
        });
}
