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
