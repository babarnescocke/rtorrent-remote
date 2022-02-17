use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rtorrent-remote",
    about = "a transmission-remote-like client for rtorrent"
)]
pub struct Cli {
    /// Add Torrent
    // Add torrent by filename, URL or Magnet URL
    #[structopt(short = "a", long = "add")]
    addtorrent: Option<Vec<String>>,

    /// Incomplete Directory
    // Where to store new torrents until they are complete
    #[structopt(short = "c", long = "incomplete-dir")]
    incompletedir: Option<String>,

    /// No Incomplete Directory
    // Don't store incomplete torrents in a different location
    #[structopt(short = "C", long = "no-incomplete-dir")]
    incompletedirbool: bool,

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
    exitrtorrent: bool,

    /// Files
    // List the current torrent(s) files.
    #[structopt(short = "f", long = "files")]
    files: bool,

    /// Info
    // Show the current torrent(s) details
    #[structopt(long = "info")]
    infobool: bool,

    /// Info Files
    // List the current torrent(s) files.
    #[structopt(long = "info-files")]
    infofilebool: bool,

    /// Info Peers
    // List the current torrent(s)' peers.
    #[structopt(long = "info-peers")]
    infopeerbool: bool,

    /// Info pieces
    // List the current torrent(s)' pieces.
    #[structopt(long = "info-pieces")]
    infopieces: bool,

    /// Info trackers
    // List the current torrent(s) trackers.
    #[structopt(long = "info-trackers")]
    infotracker: bool,

    /// Session info
    // show the session's detail
    #[structopt(long = "session-info")]
    sessioninfo: bool,

    /// Session stats
    // Show the session's statistics
    #[structopt(long = "session-stats")]
    sessionstats: bool,

    /// List Torrents
    // List torrents
    #[structopt(short = "l", long = "list")]
    list: bool,

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
    #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse))]
    /////// https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main
    rtorrenturl: Url,

    /// Tracker-Add
    // Add tracker to current torrent(s)
    #[structopt(long = "tracker-add")]
    tracker: Option<String>,

    /// Tracker-Remove
    // Remove Tracker from current torrent(s)'
    #[structopt(long = "tracker-remove")]
    trackerid: Option<String>,

    /// Start Torrent(s)
    //Start the current torrents
    #[structopt(short = "s", long = "start")]
    start: bool,

    /// Stop torrent(s)
    // stop the current torrent(s)
    #[structopt(short = "S", long = "stop")]
    stop: bool,

    /// Start paused
    // Start added torrents paused
    #[structopt(long = "start-paused")]
    starttorpaused: bool,

    #[structopt(long = "remove")]
    remove: bool,

    #[structopt(long = "remove-and-delete", long = "rad")]
    removeAndDelete: bool,
    /// Start added torrents unpaused
    // start added torrents unpaused
    #[structopt(long = "no-start-paused")]
    starttorunpaused: bool,

    /// torrent
    // Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents; and specific torrents can be chosen by id or hash.  To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
    #[structopt(short = "t", long = "torrent")]
    torrent: Option<Vec<String>>,

    /// Enable UTP
    #[structopt(long = "utp")]
    utp: bool,

    /// Disable UTP
    #[structopt(long = "no-utp")]
    noutp: bool,

    /// Verify Current Torrent(s)
    #[structopt(long = "verify", short = "V")]
    verify: bool,

    #[structopt(long = "tempdir", default_value = "/tmp/")]
    tempdir: String,
}
