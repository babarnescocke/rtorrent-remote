pub mod cli_mod {
    use std::error;
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
        pub addtorrent: Option<Vec<String>>,

        /// Incomplete Directory
        // Where to store new torrents until they are complete
        #[structopt(short = "c", long = "incomplete-dir")]
        pub incompletedir: Option<String>,

        /// No Incomplete Directory
        // Don't store incomplete torrents in a different location
        #[structopt(short = "C", long = "no-incomplete-dir")]
        pub incompletedirbool: bool,

        /// Debug
        // Print Debug information
        #[structopt(short, long)]
        pub debug: bool,

        /// Exit
        // Tell rtorrent to close down
        #[structopt(long = "exit")]
        pub exitrtorrent: bool,

        /// Files
        // List the current torrent(s) files.
        #[structopt(short = "f", long = "files")]
        pub files: bool,

        /// Info
        // Show the current torrent(s) details
        #[structopt(long = "info")]
        pub infobool: bool,

        /// Info Files
        // List the current torrent(s) files.
        #[structopt(long = "info-files")]
        pub infofilebool: bool,

        /// Info Peers
        // List the current torrent(s)' peers.
        #[structopt(long = "info-peers")]
        pub infopeerbool: bool,

        /// Info Pieces
        // List the current torrent(s)' pieces.
        #[structopt(long = "info-pieces")]
        pub infopieces: bool,

        /// Info Trackers
        // List the current torrent(s) trackers.
        #[structopt(long = "info-trackers")]
        pub infotracker: bool,

        /// Mark Files for Download
        // Tell rtorrent to download files
        #[structopt(long = "get", short = "g")]
        pub mark_files_download: Option<Vec<i64>>,

        /// Mark Files for Download
        // Tell rtorrent to download files
        #[structopt(long = "no-get", short = "G")]
        pub mark_files_skip: Option<Vec<i64>>,

        /// Session Info
        // show the session's detail
        #[structopt(long = "session-info")]
        pub sessioninfo: bool,

        /// Session Stats
        // Show the session's statistics
        #[structopt(long = "session-stats")]
        pub sessionstats: bool,

        /// Re-Announce Torrent
        // Re-announce torrent to trackers
        #[structopt(long = "reannounce")]
        pub reannounce: bool,

        /// List Torrents
        // List torrents
        #[structopt(short = "l", long = "list")]
        pub list: bool,

        /// Labels
        // set the current torrent(s)' labels
        #[structopt(short = "L", long = "labels")]
        pub labels: Option<Option<String>>,

        /// Move
        // Move Current torrent's data to a new folder
        #[structopt(long = "move")]
        pub movepath: Option<Option<String>>,

        /// No-Confirm
        // Don't ask for confirmation on certain commands, deleting torrents, exiting rtorrent etc.
        #[structopt(long = "force")]
        pub no_confirm: bool,

        /// Find
        // Tell Transmission where to find a torrent's data.
        #[structopt(long = "find")]
        pub findpath: Option<Option<String>>,

        // Host
        // the URL of rtorrent
        #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse), env = "RTORRENT_REMOTE_URL")]
        /////// https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main
        pub rtorrenturl: Url,

        /// Tracker-Add
        // Add tracker to current torrent(s)
        #[structopt(long = "tracker-add")]
        pub tracker: Option<String>,

        /// Tracker-Remove
        // Remove Tracker from current torrent(s)'
        #[structopt(long = "tracker-remove")]
        pub trackerrm: Option<String>,

        /// Start Torrent(s)
        //Start the current torrents
        #[structopt(short = "s", long = "start")]
        pub start: bool,

        /// Stop Torrent(s)
        // stop the current torrent(s)
        #[structopt(short = "S", long = "stop")]
        pub stop: bool,

        /// Start Paused
        // Start added torrents paused
        #[structopt(long = "start-paused")]
        pub starttorpaused: bool,

        /// Remove Torrent
        #[structopt(long = "remove")]
        pub remove: bool,

        ///Remove and Delete Torrent
        //Remove and Delete Torrent
        #[structopt(long = "remove-and-delete", long = "rad")]
        pub removeAndDelete: bool,

        /// Torrent
        // Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents; and specific torrents can be chosen by id or hash.  To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
        #[structopt(short = "t", long = "torrent")]
        pub torrent: Vec<String>,

        /// Verify Torrent
        #[structopt(long = "verify", short = "V")]
        pub verify: bool,

        /// Set Temp directory
        #[structopt(
            long = "tempdir",
            default_value = "/tmp/",
            env = "RTORRENT_REMOTE_TEMPDIR"
        )]
        pub tempdir: String,

        /// No Temp File
        #[structopt(long = "nt", long = "no-temp-file")]
        pub no_temp_file: bool,

        /// Local Temp Timeout
        // Local tempfile timeout in seconds
        #[structopt(long = "local-temp-timeout")]
        pub local_temp_timeout: Option<u64>,
    }

    pub fn parse_torrents(
        torrent_input_from_user: Vec<String>,
    ) -> Result<Vec<i32>, Box<dyn error::Error>> {
        let mut retVec: Vec<i32> = Vec::new();
        for f in torrent_input_from_user.iter() {
            if is_string_numeric(f) {
                retVec.push(f.parse::<i32>().unwrap());
            } else if f.contains("-") {
            }
        }
        retVec.sort_by(|a, b| a.cmp(b));
        Ok(retVec)
    }
    fn is_string_numeric(string_to_check: &String) -> bool {
        for c in string_to_check.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
}
