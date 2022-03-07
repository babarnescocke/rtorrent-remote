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

        // Cache
        // Set the maximum size of the sessions memory cache. Reset if rtorrent is restarted or closed.
        #[structopt(short = "e", long = "cache")]
        pub cachesize: Option<i32>,

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

        /// Session Info
        // show the session's detail
        #[structopt(long = "session-info")]
        pub sessioninfo: bool,

        /// Session Stats
        // Show the session's statistics
        #[structopt(long = "session-stats")]
        pub sessionstats: bool,

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

        /// Find
        // Tell Transmission where to find a torrent's data.
        #[structopt(long = "find")]
        pub findpath: Option<Option<String>>,

        // Host
        // the URL of rtorrent
        #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse))]
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

        /// Start Added torrents unpaused
        // start added torrents unpaused
        #[structopt(long = "no-start-paused")]
        pub starttorunpaused: bool,

        /// Torrent
        // Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents; and specific torrents can be chosen by id or hash.  To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
        #[structopt(short = "t", long = "torrent")]
        pub torrent: Option<Vec<String>>,

        /// Enable UTP
        #[structopt(long = "utp")]
        pub utp: bool,

        /// Disable UTP
        #[structopt(long = "no-utp")]
        pub noutp: bool,

        /// Verify Current Torrent(s)
        #[structopt(long = "verify", short = "V")]
        pub verify: bool,

        /// Set Temp directory
        #[structopt(long = "tempdir", default_value = "/tmp/")]
        pub tempdir: String,

        /// No Temp File
        #[structopt(long = "nt", long = "no-temp-file")]
        pub no_temp_file: bool,
    }

    pub fn parse_torrents(
        torrent_input_from_user: Option<Vec<String>>,
    ) -> std::io::Result<Vec<i32>, Box<dyn error::Error>> {
        match torrent_input_from_user {
            Some(x) => {
                let mut retVec: Vec<i32> = Vec::new();
                for f in x.iter_mut()
                {
                    if f.is_numeric() {
                        retVec.push(f)
                    }
                }
                Ok(retVec)
            },
            None => Err("No list of torrents provided, no strings were provided to the -t or --torrent flag")?
        }
    }
}
