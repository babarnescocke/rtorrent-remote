pub mod printingFuncs {
    use crate::torrentstructs::torrentStructs::{
        bytes_to_IEC_80000_13_string, RtorrentFileInfoStruct, RtorrentPeerStruct,
        RtorrentTorrentLSPrintStruct,
    };
    use comfy_table::presets::NOTHING;
    use comfy_table::*;
    use std::error::Error;

    // this function takes the name of a torrent and a slice of file info structs and prints them out in a nice table.
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

    // This function takes a vector of peer structs and outputs a table of peers.
    pub fn print_torrent_peers(slice_of_torrent_peer_infos: &Vec<RtorrentPeerStruct>) {
        let mut table = Table::new();
        table.load_preset(NOTHING).set_header(vec![
            "Address",
            "Encrypted",
            "Done",
            "Down",
            "Up",
            "Client",
        ]);
        for peer in slice_of_torrent_peer_infos.into_iter() {
            table.add_row(peer.to_vec_of_strings());
        }
        println!("{}", table);
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

    pub fn print_torrent_ls(slice_of_torrent_structs: Vec<RtorrentTorrentLSPrintStruct>) {
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
