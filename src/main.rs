#![allow(non_snake_case)]
use crate::clistruct::cli_mod::Cli;
use rtorrent_xmlrpc_bindings::{self, Result};
use structopt::StructOpt;
use url::Url;

mod clistruct;
fn main() {
    let cli_input = Cli::from_args();
    println!("{:?}", cli_input);
}
