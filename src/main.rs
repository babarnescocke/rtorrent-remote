use structopt::StructOpt;
//use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "rtorrent-remote", about = "a transmission-remote-like client for rtorrent")]
struct Cli {
  /// Help
  #[structopt(short, long)]
  help: bool,

  /// Add Torrent
  // Add torrent by filename or magnet URL
  #[structopt(short = "a", long = "add")]
  torrent: Option<String>,

  /// Incomplete Directory
  // Where to store new torrents until they are complete
  #[structopt(short = "-c", long = "incomplete-dir")]
  incompletedir: Option<String>,

  /// No Incomplete Directory
  // Don't store incomplete torrents in a different location
  #[structopt(short = "C", long = "no-incomplete-dir")]
  incompletedirbool: Option<bool>,

  /// Debug
  // Print Debug information
  #[structopt(short, long)]
  debug: bool,

  //

}
fn main() {
  let cli = Cli::from_args();
  println!("{:?}", cli);
}
