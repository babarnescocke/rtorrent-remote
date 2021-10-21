use structopt::StructOpt;
//use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "rtorrent-remote", about = "a transmission-remote-like client for rtorrent")]
struct Cli {

  /// Add Torrent
  // Add torrent by filename or magnet URL
  #[structopt(short = "a", long = "add")]
  addtorrent: Option<String>,

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
  // the URL of rtorrent
  #[structopt(short, default_value = "http://localhost:8080/rpc2")] // https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main 
  rtorrenturl: String,

  /// Tracker-Add
  // Add tracker to current torrent(s)'
  #[structopt(long = "tracker-add")]
  tracker: Option<String>,       

  /// Tracker-Remove
  // Remove Tracker from current torrent(s)'
  #[structopt(long = "tracker-remove")]
  trackerid: Option<String>,

  /// Start Torrent(s)
  //Start the current torrents
  #[structopt(short = "s", long = "start")]
  start: Option<bool>,

  /// Stop torrent(s)
  // stop the current torrent(s)
  #[structopt(short = "S", long = "stop")]
  stop: Option<bool>,

  /// Start paused
  // Start added torrents paused
  #[structopt(long = "start-paused")]
  starttorpaused: Option<bool>,

  /// Start added torrents unpaused
  // start added torrents unpaused
  #[structopt(long = "no-start-paused")]
  starttorunpaused: Option<bool>,

  /// torrent
  // Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents; and specific torrents can be chosen by id or hash.  To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
  #[structopt(short = "t", long = "torrent")]
  torrent: Option<String>,

  /// Enable UTP
  #[structopt(long = "utp")]
  utp: Option<bool>,

  /// Disable UTP
  #[structopt(long = "no-utp")]
  noutp: Option<bool>,

  /// Verify Current Torrent(s)
  #[structopt(long = "verify", short = "V")]
  verify: Option<bool>,


}
fn main() {
  let cli = Cli::from_args();
  println!("{:?}", cli);
}
