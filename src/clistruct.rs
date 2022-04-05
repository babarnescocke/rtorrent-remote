/// module to handle structopt struct and parser. Chose structopt not just because we have a lot of input to handle, but we do, but because StructOpt automatically generates help, version and hopefully shell-completions and man pages.
pub mod cli_mod {
    use crate::vechelp::hashvechelp::{to_vec_of_tor_hashes, unix_time_now};
    use std::error;
    //use std::str::FromStr;
    use rtorrent_xmlrpc_bindings::Server;
    use std::vec::Vec;
    use structopt::StructOpt;
    use url::Url;

    /// Struct of Cli Args
    #[derive(Debug, StructOpt)]
    #[structopt(
        name = "rtorrent-remote",
        about = "a transmission-remote-like client for rtorrent"
    )]
    pub struct Cli {
        /// Add Torrent
        // Add torrent by filename, URL or Magnet URL
        #[structopt(short = "a", long = "add")]
        pub addtorrent: Option<String>,

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
        #[structopt(short = "f", long = "files", requires = "torrent")]
        pub files: bool,

        /// Show details of the current torrent(s)
        #[structopt(long = "info", requires = "torrent")]
        pub infobool: bool,

        /// List the specified torrent's files.
        #[structopt(long = "info-files", requires = "torrent")]
        pub infofilebool: bool,

        /// List the specified torrent's peers
        #[structopt(long = "info-peers", requires = "torrent")]
        pub infopeerbool: bool,

        /// List the current torrent(s)' pieces.
        #[structopt(long = "info-pieces", requires = "torrent")]
        pub infopieces: bool,

        /// List the current torrent(s) trackers.
        #[structopt(long = "info-trackers", requires = "torrent")]
        pub infotracker: bool,

        ///Tell rtorrent to download files
        #[structopt(long = "get", short = "g", requires = "torrent")]
        pub mark_files_download: Option<String>,

        /// Tell rtorrent to download files
        #[structopt(long = "no-get", short = "G", requires = "torrent")]
        pub mark_files_skip: Option<String>,

        /// List session information from the server
        #[structopt(long = "session-info")]
        pub sessioninfo: bool,

        /// List statistical information from the server
        #[structopt(long = "session-stats")]
        pub sessionstats: bool,

        /// Reannounce the current torrent(s).
        #[structopt(long = "reannounce", requires = "torrent")]
        pub reannounce: bool,

        /// List Torrents
        #[structopt(short = "l", long = "list")]
        pub list: bool,

        /// Labels
        // set the current torrent(s)' labels
        #[structopt(short = "L", long = "labels", requires = "torrent")]
        pub labels: Option<String>,

        /// Give this torrent first chance at available bandwidth
        #[structopt(
            long = "Bh",
            long = "bandwidth-high",
            requires = "torrent",
            conflicts_with = "bandwidth-normal"
        )]
        pub bandwidth_high: bool,

        /// Give this torrent the bandwidth left over by high priority torrents
        #[structopt(
            long = "Bn",
            long = "bandwidth-normal",
            requires = "torrent",
            conflicts_with = "bandwidth-low"
        )]
        pub bandwidth_normal: bool,

        /// Give this torrent the bandwidth left over by high and normal priority torrents
        #[structopt(
            long = "Bl",
            long = "bandwidth-low",
            requires = "torrent",
            conflicts_with = "bandwidth-high"
        )]
        pub bandwidth_low: bool,

        /// Try to download the specified file(s) first. all marks all of the torrent's files as normal priority, file-index sets a single file's priority as normal, and files sets multiple files' priorities as normal, such as "-pn=1,3-5" to normalize files #1, #3, #4, and #5.
        #[structopt(long = "ph", long = "priority-high", requires = "torrent")]
        pub priority_high: Option<String>,

        /// Try to download the specified files normally.
        #[structopt(long = "pn", long = "priority-normal", requires = "torrent")]
        pub priority_normal: Option<String>,

        /// Set the maximum number of peers. If current torrent(s) are selected this operates on them. Otherwise, it changes the global setting.
        #[structopt(long = "pr", long = "peers", requires = "torrent")]
        pub peers: Option<i64>,

        /// Move the current torrents' data from their current locations to the specified directory.
        #[structopt(long = "move", requires = "torrent")]
        pub movepath: Option<String>,

        /// No-Confirm For rtorrent-remote operations
        // Don't ask for confirmation on certain commands, deleting torrents, exiting rtorrent etc.
        #[structopt(long = "no-confirm")]
        pub no_confirm: bool,

        /// Tell Transmission where to look for the current torrents' data.
        #[structopt(long = "find", requires = "torrent")]
        pub findpath: Option<String>,

        /// Host - the URL of rtorrent
        #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse), env = "RTORRENT_REMOTE_URL")]
        /////// https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main
        pub rtorrenturl: Url,

        /// Add a tracker to a torrent
        #[structopt(long = "tracker-add", requires = "torrent")]
        pub tracker: Option<String>,

        /// Remove a tracker from a torrent
        #[structopt(long = "tracker-remove", requires = "torrent")]
        pub trackerrm: Option<String>,

        /// Start Torrent(s)
        //Start the current torrents
        #[structopt(short = "s", long = "start", requires = "torrent")]
        pub start: bool,

        /// Stop Torrent(s)
        // stop the current torrent(s)
        #[structopt(short = "S", long = "stop", requires = "torrent")]
        pub stop: bool,

        /// Add torrent paused to rtorrent, URL, Magnet Link or Filename.
        #[structopt(long = "add-paused")]
        pub starttorpaused: Option<String>,

        /// Remove Torrent
        #[structopt(long = "remove", requires = "torrent")]
        pub remove: bool,

        ///Remove and Delete Torrent
        //Remove and Delete Torrent
        #[structopt(long = "remove-and-delete", long = "rad", requires = "torrent")]
        pub removeAndDelete: bool,

        /// Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents. To set more than one current torrent, join their ids together in a list, such as "-t=2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
        #[structopt(short = "t", long = "torrent")]
        pub torrent: Option<String>,

        /// Verify Torrent
        #[structopt(long = "verify", short = "V", requires = "torrent")]
        pub verify: bool,

        /// Set Temp directory
        #[structopt(
            long = "tempdir",
            env = "RTORRENT_REMOTE_TEMPDIR",
            conflicts_with = "no_temp_file"
        )]
        pub tempdir: Option<String>,

        /// No Temp File
        #[structopt(long = "nt", long = "no-temp-file")]
        pub no_temp_file: bool,

        /// Local Temp Timeout
        // Local tempfile timeout in seconds
        #[structopt(long = "local-temp-timeout")]
        pub local_temp_timeout: Option<u64>,

        /// Use rtorrent time for tempfile - results in multiple queries per run. Will run noticeably slower with higher latency networks.
        // Queries the uptime of the rtorrent server to verify tempfile information
        #[structopt(long = "query-rtorrent-time", conflicts_with = "local-temp-timeout")]
        pub rtorrent_time_query: bool,
    }

    impl Cli {
        /// Convenience function produces an rtorrent server handle, ServerInner, for xmlrpc requests. Not related to the struct, but very convenient.
        pub fn new_handle(&self) -> Server {
            Server::new(self.rtorrenturl.as_ref())
        }
        /// Convenience function that
        pub fn vec_of_tor_hashes(&self) -> std::result::Result<Vec<String>, Box<dyn error::Error>> {
            to_vec_of_tor_hashes(
                self.tempdir(),
                self.new_handle(),
                self.rtorrent_time_query,
                self.local_temp_timeout,
                self.no_confirm,
            )
        }
        /// returns true if -l, --list has been passed, or if no other flag has been passed.
        pub fn list(&self) -> bool {
            if self.list || self.no_action_selected() {
                true
            } else {
                false
            }
        }
        pub fn torrent_string_to_veci32(&self) -> Result<Vec<i32>, Box<dyn error::Error>> {
            match &self.torrent {
                Some(x) => return Ok(string_to_veci32(x)?),
                None => return Err("--torrent, -t, flag was not given adequate input")?,
            }
        }
        pub fn priority_high_string_to_veci32(&self) -> Result<Vec<i32>, Box<dyn error::Error>> {
            match &self.priority_high {
                Some(x) => return Ok(string_to_veci32(x)?),
                None => return Err("--priority-high, --ph, flag was not given adequate input")?,
            }
        }
        pub fn priority_normal_string_to_veci32(&self) -> Result<Vec<i32>, Box<dyn error::Error>> {
            match &self.priority_normal {
                Some(x) => return Ok(string_to_veci32(x)?),
                None => return Err("--priority-normal, --pn, flag was not given adequate input")?,
            }
        }
        pub fn get_string_to_veci32(&self) -> Result<Vec<i32>, Box<dyn error::Error>> {
            match &self.mark_files_download {
                Some(x) => return Ok(string_to_veci32(x)?),
                None => return Err("--get, -g, flag was not given adequate input")?,
            }
        }
        pub fn no_get_string_to_veci32(&self) -> Result<Vec<i32>, Box<dyn error::Error>> {
            match &self.mark_files_skip {
                Some(x) => return Ok(string_to_veci32(x)?),
                None => return Err("--no-get, -G, flag was not given adequate input")?,
            }
        }
        pub fn tempdir(&self) -> String {
            match &self.tempdir {
                Some(x) => x.to_string(),
                None => String::from("/tmp/"),
            }
        }
        pub fn no_action_selected(&self) -> bool {
            if self.addtorrent.is_some()
                || self.labels.is_some()
                || self.mark_files_download.is_some()
                || self.mark_files_skip.is_some()
                || self.priority_high.is_some()
                || self.priority_normal.is_some()
                || self.peers.is_some()
                || self.movepath.is_some()
                || self.findpath.is_some()
                || self.tracker.is_some()
                || self.trackerrm.is_some()
                || self.starttorpaused.is_some()
                || self.debug
                || self.exitrtorrent
                || self.files
                || self.infobool
                || self.infofilebool
                || self.infopeerbool
                || self.infopieces
                || self.infotracker
                || self.sessioninfo
                || self.sessionstats
                || self.reannounce
                || self.list
                || self.bandwidth_high
                || self.bandwidth_normal
                || self.bandwidth_low
                || self.start
                || self.stop
                || self.remove
                || self.removeAndDelete
                || self.verify
            {
                return false;
            }
            true
        }
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

    ///a parser that takes input like "1 2"; "1,2"; "1-2"; "1;2"; "2-1"; "2,2-1" etc and produces vec[1,2];
    /// it needs to work for the --torrent; --get and --no-get flags. Can probably be abstracted to be some <T>, baby steps.
    pub fn string_to_veci32(s: &String) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        // produce the vec we need to send back
        let mut retVec: Vec<i32> = Vec::new();
        // if the string is only numeric, no letters/non-numeric symbols we can just add that to our return vec, we are done.
        if is_string_numeric(&s.to_string()) {
            retVec.push(s.parse::<i32>()?);
        // those simple cases out of the way, we separate by dividing characters and then evaluate each delimited substring on its own. If a given substring contains a '-' we split on it traversing the digits between those two numbers.
        } else {
            let v: Vec<&str> = s.split(&[';', ',', ' '][..]).collect();
            // walk substrings
            for y in v.into_iter() {
                // if the substring doesn't have a '-' we can attempt to delimit it and push it on to our vec.
                if !y.contains('-') {
                    retVec.push(y.parse::<i32>()?);
                } else {
                    // we instantiate a temporary vector to walk later
                    let mut temp_vec = Vec::new();
                    //split our substring by '-' and iterate over that.
                    for l in y.split('-').into_iter() {
                        // push on to temporary vector
                        temp_vec.push(l);
                    }
                    // possibly you could make a parser that does this and takes odd pairings or ranges to the limits of some number, eg "1-2-3", "-3" both being a vector [1,2,3] but this seems illogical, so if we get input like that we error out.
                    if temp_vec.len() != 2 {
                        Err("Presented a range that cannot be parsed")?
                    }
                    // now that we know we have two values to traverse, we simply need to make sure that they are in order, parse them, and traverse them pushing each to our retvec as we go.
                    temp_vec.sort();
                    let stop = temp_vec.pop().unwrap().parse::<i32>()?;
                    let start = temp_vec.pop().unwrap().parse::<i32>()?;
                    for q in start..stop + 1 {
                        retVec.push(q);
                    }
                }
            }
        }
        retVec.sort_unstable();
        retVec.dedup();
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
    /// Provides the rtorrent instance's current uptime in seconds.
    pub fn rtorrent_time_up(handle: Server) -> std::result::Result<i64, Box<dyn error::Error>> {
        Ok(unix_time_now()? as i64 - handle.startup_time()?)
    }
}
#[cfg(test)]

mod tests {
    use super::cli_mod;
    #[test]
    fn string_to_vec_test_two_at_a_time() {
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1,2")).unwrap(),
            [1, 2]
        );
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1-2")).unwrap(),
            [1, 2]
        );
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("2-1")).unwrap(),
            [1, 2]
        );
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1;2")).unwrap(),
            [1, 2]
        );
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1 2")).unwrap(),
            [1, 2]
        );
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1 2,2")).unwrap(),
            [1, 2]
        );
    }
    #[test]
    fn string_to_vec_test_the_whole_shebang() {
        assert_eq!(
            cli_mod::string_to_veci32(&String::from("1,2;3,4-5 4-8")).unwrap(),
            [1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
}
