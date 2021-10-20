use structopt::StructOpt;
//use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "rtorrent-remote", about = "a transmission-remote-like client for rtorrent")]
struct Cli {

  /// Add Torrent
  // Add torrent by filename or magnet URL
  #[structopt(short = "a", long = "add")]
  torrent: Option<String>,

  /// Incomplete Directory
  // Where to store new torrents until they are complete
  #[structopt(short = "c", long = "incomplete-dir")]
  incompletedir: Option<String>,

  /// No Incomplete Directory
  // Don't store incomplete torrents in a different location
  #[structopt(short = "C", long = "no-incomplete-dir")]
  incompletedirbool: Option<bool>,

  /// Debug
  // Print Debug information
  #[structopt(short, long)]
  debug: bool,

  // Cache
  // Set the maximum size of the sessions memory cache. Reset if rtorrent is restarted or closed.
  #[structopt(short = "e", long = "cache")]
  cachesize: Option<i32>,

  /// Exit
  // Tell rtorrent to close down
  #[structopt(long = "exit")]
  exitrtorrent: Option<bool>,

  /// Files
  // List the current torrent(s) files.
  #[structopt(short = "f", long = "files")]
  files: Option<bool>,

  /// Info
  // Show the current torrent(s) details
  #[structopt(long = "info")]
  infobool: Option<bool>,

  /// Info Files
  // List the current torrent(s) files.
  #[structopt(long = "info-files")]
  infofilebool: Option<bool>,

  /// Info Peers
  // List the current torrent(s)' peers.
  #[structopt( long = "info-peers")]
  infopeerbool: Option<bool>,

  /// Info pieces
  // List the current torrent(s)' pieces.
  #[structopt(long = "info-pieces")]
  infopieces: Option<bool>,

  /// Info trackers
  // List the current torrent(s) trackers.
  #[structopt(long = "info-trackers")]
  infotracker: Option<bool>,

  /// Session info
  // show the session's detail
  #[structopt(long = "session-info")]
  sessioninfo: Option<bool>,

  /// Session stats
  // Show the session's statistics
  #[structopt(long = "session-stats")]
  sessionstats: Option<bool>,

  /// List Torrents
  // List torrents
  #[structopt(short = "l", long = "list")]
  list: Option<bool>,

  /// Labels
  // set the current torrent(s)' labels
  #[structopt(short = "L", long = "labels")]
  labels: Option<Option<String>>,

  /// Move
  // Move Current torrent's data to a new folder
  #[structopt(long = "move")]
  movepath: Option<Option<String>>,

  /// Find
  // Tell Transmission where to find a torrent's data.
  #[structopt(long = "find")]
  findpath: Option<Option<String>>,

  // Host
  #[structopt(default)]
}
fn main() {
  let cli = Cli::from_args();
  println!("{:?}", cli);
}
