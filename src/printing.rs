pub mod printingFuncs {
    use crate::torrentstructs::torrentStructs::{
        bytes_to_IEC_80000_13_string, RtorrentFileInfoStruct, RtorrentTorrentLSPrintStruct,
    };
    use comfy_table::presets::NOTHING;
    use comfy_table::*;
    use std::error::Error;
    // this function needs to have the following structure while printing:
    /*	Big Buck Bunny (3 files):
      #  Done Priority Get      Size  Name
      0: 100% Normal   Yes   0.14 kB  Big Buck Bunny/Big Buck Bunny.en.srt
      1: 100% Normal   Yes  276.1 MB  Big Buck Bunny/Big Buck Bunny.mp4
      2: 100% Normal   Yes  310.4 kB  Big Buck Bunny/poster.jpg
    */

    pub fn print_torrent_files(
        name_of_torrent: String,
        slice_of_torrent_file_infos: &[RtorrentFileInfoStruct],
    ) {
        if slice_of_torrent_file_infos.len() > 1 {
            println!(
                "{} ({} files):",
                name_of_torrent,
                slice_of_torrent_file_infos.len()
            );
        } else {
            println!("{} (1 file):", name_of_torrent);
        }

        let mut table = Table::new();
        table
            .load_preset(NOTHING)
            .set_header(vec!["#", "Done", "Priority", "Get", "Size", "Name"]);
        for fileInfo in slice_of_torrent_file_infos.into_iter() {
            table.add_row(fileInfo.to_vec_of_strings());
        }
        println!("{}", table);
    }

    // this function needs to have the following structure while printing:
    /*
    Address                       Flags         Done  Down    Up      Client
    IP ADDR                       T?EH          0.0      0.0     0.0  Deluge 2.0.3.54
    */
    pub fn print_torrent_peers() -> Result<(), Box<dyn Error>> {
        todo!();
    }
    // this function needs to have the following structure while printing:

    /*
        NAME
      Id: 1
      Name: Big Buck Bunny
      Hash: dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c
      Magnet: magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&dn=Big%20Buck%20Bunny&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F
      Labels:

    TRANSFER
      State: Idle
      Location: /var/lib/transmission/Downloads
      Percent Done: 100%
      ETA: 0 seconds (0 seconds)
      Download Speed: 0 kB/s
      Upload Speed: 0 kB/s
      Have: 276.4 MB (276.4 MB verified)
      Availability: 100%
      Total size: 276.4 MB (276.4 MB wanted)
      Downloaded: 281.1 MB
      Uploaded: 42.32 MB
      Ratio: 0.1
      Corrupt DL: 262.1 kB
      Peers: connected to 0, uploading to 0, downloading from 0
      Web Seeds: downloading from 0 of 1 web seeds

    HISTORY
      Date added:
      Date finished:
      Date started:
      Latest activity:
      Downloading Time:
      Seeding Time:     41 minutes (2468 seconds)

    ORIGINS
      Date created: Thu Mar 30 16:30:01 2017
      Public torrent: Yes
      Comment: WebTorrent <https://webtorrent.io>
      Creator: WebTorrent <https://webtorrent.io>
      Piece Count: 1055
      Piece Size: 256.0 KiB

    LIMITS & BANDWIDTH
      Download Limit: Unlimited
      Upload Limit: Unlimited
      Ratio Limit: Default
      Honors Session Limits: Yes
      Peer limit: 50
      Bandwidth Priority: Normal
        */
    pub fn print_torrent_info() -> Result<(), Box<dyn Error>> {
        todo!();
    }

    // the below function should have something like this as a response:
    /*

    Tracker 0: udp://tracker.leechers-paradise.org:6969
    Active in tier 0
    Got an error "Connection failed" 3 minutes, 59 seconds (239 seconds) ago
    Asking for more peers in 1 hour, 56 minutes (7016 seconds)
    Got a scrape error "Connection failed" 23 minutes (1424 seconds) ago
    Asking for peer counts in 37 minutes (2223 seconds)

    Tracker 1: udp://tracker.coppersurfer.tk:6969
    Active in tier 1
    Got an error "Connection failed" 23 minutes (1429 seconds) ago
    Asking for more peers in 36 minutes (2184 seconds)
    Got a scrape error "Connection failed" 2 minutes, 27 seconds (147 seconds) ago
    Asking for peer counts in 1 hour, 58 minutes (7113 seconds)

    Tracker 2: udp://tracker.opentrackr.org:1337
    Active in tier 2
    Got a list of 37 peers 12 minutes (734 seconds) ago
    Asking for more peers in 15 minutes (955 seconds)
    Tracker had 35 seeders and 2 leechers 12 minutes (737 seconds) ago
    Asking for peer counts in 17 minutes (1063 seconds)

    Tracker 3: udp://explodie.org:6969
    Active in tier 3
    Got a list of 29 peers 7 minutes (423 seconds) ago
    Asking for more peers in 27 minutes (1677 seconds)
    Tracker had 28 seeders and 2 leechers 12 minutes (737 seconds) ago
    Asking for peer counts in 17 minutes (1063 seconds)

    Tracker 4: udp://tracker.empire-js.us:1337
    Active in tier 4
    Got an error "Connection failed" 22 minutes (1336 seconds) ago
    Asking for more peers in 38 minutes (2304 seconds)
    Got a scrape error "Connection failed" 2 minutes, 32 seconds (152 seconds) ago
    Asking for peer counts in 1 hour, 57 minutes (7063 seconds)


      */
    pub fn print_torrent_trackers() -> Result<(), Box<dyn Error>> {
        todo!();
    }

    // the following should have something like:

    /*
       11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111
     11111111 11111111 11111111 1111111
    */
    pub fn print_torrent_pieces() -> Result<(), Box<dyn Error>> {
        todo!();
    }
    // below function's output should look like this:
    /*
    VERSION
      Daemon version: 3.00 (bb6b5a062e)
      RPC version: 16
      RPC minimum version: 1

    CONFIG
      Configuration directory: /var/lib/transmission/.config/transmission-daemon
      Download directory: /var/lib/transmission/Downloads
      Listenport: 51413
      Portforwarding enabled: Yes
      uTP enabled: Yes
      Distributed hash table enabled: Yes
      Local peer discovery enabled: No
      Peer exchange allowed: Yes
      Encryption: preferred
      Maximum memory cache size: 4.00 MiB

    LIMITS
      Peer limit: 200
      Default seed ratio limit: Unlimited
      Upload speed limit: Unlimited (Disabled limit: 100 kB/s; Disabled turtle limit: 50 kB/s)
      Download speed limit: Unlimited (Disabled limit: 100 kB/s; Disabled turtle limit: 50 kB/s)

    MISC
      Autostart added torrents: Yes
      Delete automatically added torrents: No

        */
    pub fn print_session_info() -> Result<(), Box<dyn Error>> {
        todo!();
    }

    //the output for the below function should look something like this:
    /*
     CURRENT SESSION
      Uploaded:   42.84 MB
      Downloaded: 414.2 MB
      Ratio:      0.1
      Duration:   2 hours, 43 minutes (9829 seconds)

    TOTAL
      Started 1 times
      Uploaded:   42.84 MB
      Downloaded: 414.2 MB
      Ratio:      0.1
      Duration:   2 hours, 43 minutes (9829 seconds)
    Unknown option: localhost


        */
    pub fn print_session_stats() -> Result<(), Box<dyn Error>> {
        todo!();
    }

    pub fn print_torrent_ls(slice_of_torrent_structs: &[RtorrentTorrentLSPrintStruct]) {
        //slice_of_torrent_structs.sort_by_key(|t| t.id.clone());
        let mut table = Table::new();
        let mut sum_bytes = 0;
        let mut sum_up = 0;
        let mut sum_down = 0;
        table.load_preset(NOTHING).set_header(vec![
            "ID", "Done", "Have", "ETA", "Up", "Down", "Ratio", "Status", "Name",
        ]);
        for tempTor in slice_of_torrent_structs.into_iter() {
            table.add_row(tempTor.to_vec_of_strings());
            sum_bytes += tempTor.raw_bytes_have;
            sum_up += tempTor.raw_up;
            sum_down += tempTor.raw_down;
        }
        table.add_row([
            "Sum:",
            "",
            &bytes_to_IEC_80000_13_string(sum_bytes),
            "",
            &sum_up.to_string(),
            &sum_down.to_string(),
            "",
            "",
            "",
        ]);
        println!("{}", table);
    }
}
