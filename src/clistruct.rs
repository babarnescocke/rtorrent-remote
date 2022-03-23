/// module to handle structopt struct and parser
pub mod cli_mod {
    use std::error;
    //use std::str::FromStr;
    use std::vec::Vec;
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
        #[structopt(short = "a", long = "add", default_value = "")]
        pub addtorrent: String,

        /// Local Debug
        // Print Debug information
        #[structopt(short, long)]
        pub debug: bool,

        /// Exit Rtorrent
        // Close Rtorrent to close down
        #[structopt(long = "exit")]
        pub exitrtorrent: bool,

        /// Get a file list for the current torrent(s)
        // List the current torrent(s) files.
        #[structopt(short = "f", long = "files")]
        pub files: bool,

        /// Show details of the current torrent(s)
        #[structopt(long = "info")]
        pub infobool: bool,

        /// List the specified torrent's files.
        #[structopt(long = "info-files")]
        pub infofilebool: bool,

        /// List the specified torrent's peers
        #[structopt(long = "info-peers")]
        pub infopeerbool: bool,

        /// List the current torrent(s)' pieces.
        #[structopt(long = "info-pieces")]
        pub infopieces: bool,

        /// List the current torrent(s) trackers.
        #[structopt(long = "info-trackers")]
        pub infotracker: bool,

        ///Tell rtorrent to download files
        #[structopt(long = "get", short = "g", use_delimiter = true)]
        pub mark_files_download: Vec<i64>,

        /// Tell rtorrent to download files
        #[structopt(long = "no-get", short = "G", use_delimiter = true)]
        pub mark_files_skip: Vec<i64>,

        /// List session information from the server
        #[structopt(long = "session-info")]
        pub sessioninfo: bool,

        /// List statistical information from the server
        #[structopt(long = "session-stats")]
        pub sessionstats: bool,

        /// Reannounce the current torrent(s).
        #[structopt(long = "reannounce")]
        pub reannounce: bool,

        /// List Torrents
        #[structopt(short = "l", long = "list")]
        pub list: bool,

        /// Labels
        // set the current torrent(s)' labels
        #[structopt(short = "L", long = "labels", default_value = "")]
        pub labels: String,

        /// Give this torrent first chance at available bandwidth
        #[structopt(long = "Bh", long = "bandwidth-high")]
        pub bandwidth_high: bool,

        /// Give this torrent the bandwidth left over by high priority torrents
        #[structopt(long = "Bn", long = "bandwidth-normal")]
        pub bandwidth_normal: bool,

        /// Give this torrent the bandwidth left over by high and normal priority torrents
        #[structopt(long = "Bl", long = "bandwidth-low")]
        pub bandwidth_low: bool,

        /// Try to download the specified file(s) first. all marks all of the torrent's files as normal priority, file-index sets a single file's priority as normal, and files sets multiple files' priorities as normal, such as "-pn1,3-5" to normalize files #1, #3, #4, and #5.
        #[structopt(long = "ph", long = "priority-high", use_delimiter = true)]
        pub priority_high: Vec<i64>,

        /// Try to download the specified files normally.
        #[structopt(long = "pn", long = "priority-normal", use_delimiter = true)]
        pub priority_normal: Vec<i64>,

        /// Set the maximum number of peers. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
        #[structopt(long = "pr", long = "peers", default_value = "0")]
        pub peers: i64,

        /// Move the current torrents' data from their current locations to the specified directory.
        #[structopt(long = "move", default_value = "")]
        pub movepath: String,

        /// No-Confirm For rtorrent-remote operations
        // Don't ask for confirmation on certain commands, deleting torrents, exiting rtorrent etc.
        #[structopt(long = "no-confirm")]
        pub no_confirm: bool,

        /// Tell Transmission where to look for the current torrents' data.
        #[structopt(long = "find",default_value = "")]
        pub findpath: String,

        /// Host - the URL of rtorrent
        #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse), env = "RTORRENT_REMOTE_URL")]
        /////// https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main
        pub rtorrenturl: Url,

        /// Add a tracker to a torrent
        #[structopt(long = "tracker-add", default_value= "")]
        pub tracker: String,

        /// Remove a tracker from a torrent
        #[structopt(long = "tracker-remove", default_value = "")]
        pub trackerrm: String,

        /// Start Torrent(s)
        //Start the current torrents
        #[structopt(short = "s", long = "start")]
        pub start: bool,

        /// Stop Torrent(s)
        // stop the current torrent(s)
        #[structopt(short = "S", long = "stop")]
        pub stop: bool,

        /// Add torrent paused to rtorrent, URL, Magnet Link or Filename.
        #[structopt(long = "start-paused")]
        pub starttorpaused: bool,

        /// Remove Torrent
        #[structopt(long = "remove")]
        pub remove: bool,

        ///Remove and Delete Torrent
        //Remove and Delete Torrent
        #[structopt(long = "remove-and-delete", long = "rad")]
        pub removeAndDelete: bool,

        /// Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents. To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
        #[structopt(short = "t", long = "torrent", use_delimiter = true)]
        pub torrent: Vec<i32>,

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
        #[structopt(long = "local-temp-timeout",default_value = "0")]
        pub local_temp_timeout: i64,
    }

    // here is a list of commands I think could be implemented:
    /*

    -as --alt-speed
        Use the alternate Limits.
    -AS --no-alt-speed
        Don't use the alternate Limits.
    -asd --alt-speed-downlimit limit
        Limit the alternate download speed to limit kilobytes per second.
    -asu --alt-speed-uplimit limit
        Limit the alternate upload speed to limit kilobytes per second.
    -asc --alt-speed-scheduler
        Use the scheduled on/off times.
    -ASC --no-alt-speed-scheduler
        Don't use the scheduled on/off days and times.
    --alt-speed-time-begin time
        Time to start using the alt speed limits (in hhmm).
    --alt-speed-time-end time
        Time to stop using the alt speed limits (in hhmm).
    --alt-speed-days days
        Set the number of days on which to enable the speed scheduler, using a list such as "2,4-6".
    --torrent-done-script filename
        Specify a file to run each time a torrent finishes
    --no-torrent-done-script
        Don't run any script when a torrent finishes

    -d --downlimit limit
        Limit the maximum download speed to limit kB/s. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
    -D --no-downlimit
        Disable download speed limits. If current torrent(s) are

    -u --uplimit limit
        Limit the maximum upload speed to limit kB/s. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
    -U --no-uplimit
        Disable upload speed limits.

    --utp
        Enable uTP for peer connections.
    --no-utp
        Disable uTP for peer connections. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.

    -er --encryption-required
        Encrypt all peer connections.
    -ep --encryption-preferred
        Prefer encrypted peer connections.
    -et --encryption-tolerated
        Prefer unencrypted peer connections.

    -gsr --global-seedratio ratio
        All torrents, unless overridden by a per-torrent setting, should seed until a specific ratio
    -GSR --no-global-seedratio
        All torrents, unless overridden by a per-torrent setting, should seed regardless of ratio
        -m --portmap
        Enable portmapping via NAT-PMP or UPnP
    -M --no-portmap
        Disable portmapping

    -x --pex
        Enable peer exchange (PEX).
    -X --no-pex
        Disable peer exchange (PEX).
    -y --lds
        Enable local peer discovery (LPD).
    -Y --no-lds
        Disable local peer discovery (LPD).
        */

    /// the following are unimplementable flags from transmission-remote
    /*

    -c --incomplete-dir dir
        When adding new torrents, store their contents in directory until the torrent is done.
    -C --no-incomplete-dir
        Don't store incomplete torrents in a different directory.

    -er --encryption-required
        Encrypt all peer connections.
    -ep --encryption-preferred
        Prefer encrypted peer connections.
    -et --encryption-tolerated
        Prefer unencrypted peer connections.
    -pl --priority-low all | file-index | files
        Try to download the specified files last
        */
    

    pub trait FromStr: Sized {
        fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>>;
    }
    impl FromStr for Vec<i32> {
        ///a parser that takes input like "1 2"; "1,2"; "1-2"; "1;2"; "2-1" etc and produces vec[1,2];
        /// it needs to work for the --torrent; --get and --no-get flags.

        fn from_str(s: &str) -> Result<Vec<i32>, Box<dyn error::Error>> {
            let mut retVec: Vec<i32> = Vec::new();
            if s.len() == 0 {
                Err("Nothing provided to be parsed")?
            } else if is_string_numeric(&s.to_string()) {
                retVec.push(s.parse::<i32>()?);
            } else if s.contains("-") {
                let mut temp_vec = Vec::new();
                for l in s.split("-").into_iter() {
                    temp_vec.push(l)
                }
                if temp_vec.len() != 2 {
                    Err("Presented a range that cannot be parsed")?
                }
                temp_vec.sort();
                let stop = temp_vec.pop().unwrap().parse::<i32>()?;
                let start = temp_vec.pop().unwrap().parse::<i32>()?;
                for q in start..stop {
                    retVec.push(q);
                }
            } else {
                let v: Vec<&str> = s.split(&[';', ',', ' '][..]).collect();
                for y in v.into_iter() {
                    retVec.push(y.parse::<i32>()?);
                }
            }

            retVec.sort_unstable();
            retVec.dedup();
            Ok(retVec)
        }
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
