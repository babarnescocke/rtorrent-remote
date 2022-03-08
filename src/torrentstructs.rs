// This is a set of functions and libraries that are used to derive torrent information, from rtorrent-xmlrpc responses and manipulate them.
// The structs are a bit weird in that a lot of what I need from rtorrent is only to derive some value, I don't actually want the value returned from rtorrent. So there is a wrapper fn - new_torrent_print_maker - which basically takes a full xmlrpc call response and generates what is needed to print transmission-remote -l.
// This program logic is to quickly drop values we don't need, if bools like "is_active" - which is amazingly unhelpful.

pub mod torrentStructs {
    use compound_duration::format_wdhms;
    pub fn new_torrent_ls_print_maker(
        id: i32,
        hash: Option<String>,
        down_rate: i64,
        up_rate: i64,
        name: String,
        ratio: f64,
        is_active: bool,
        left_bytes: i64,
        complete_bytes: i64,
        hashing: bool,
    ) -> RtorrentTorrentLSPrintStruct {
        RtorrentTorrentLSPrintStruct {
            id: id,
            hash: hash,
            done: done_stringer(complete_bytes.clone(), left_bytes.clone()),
            have: have_stringer(complete_bytes),
            eta: eta_maker(left_bytes.clone(), down_rate.clone()),
            down_rate: down_rate.clone().to_string(),
            up_rate: up_rate.clone().to_string(),
            ratio: format!("{:.1}", ratio),
            status: status_maker(is_active, up_rate, down_rate, left_bytes, hashing),
            name: name,
        }
    }

    // This is a struct that builds a torrent from information that rtorrent provides
    pub struct RtorrentTorrentLSPrintStruct {
        // need to have ID, Done%, Have (bytes have), ETA, Up rate, Down Rate, Ratio, Status, Name
        pub id: i32,
        pub hash: Option<String>,
        pub done: String,
        pub have: String,
        pub eta: String,
        pub down_rate: String,
        pub up_rate: String,
        pub ratio: String,
        pub status: String,
        pub name: String,
    }
    impl RtorrentTorrentLSPrintStruct {
        pub fn to_vec(&self) -> Vec<String> {
            return vec![
                self.id.to_string().clone(),
                self.done.clone(),
                self.have.clone(),
                self.eta.clone(),
                self.up_rate.clone(),
                self.down_rate.clone(),
                self.ratio.clone(),
                self.status.clone(),
                self.name.clone(),
            ];
        }
    }

    //
    fn eta_maker(bytes_left: i64, down_rate: i64) -> String {
        // this is from compound_duration. I started coding it, it wasn't hard but it was just so tedious, so found that crate.
        if bytes_left == 0 {
            return String::from("Done");
        }
        if down_rate == 0 {
            return String::from("N/A");
        }
        format_wdhms(bytes_left / down_rate)
    }

    fn done_stringer(complete_bytes: i64, left_bytes: i64) -> String {
        if left_bytes == 0 {
            return String::from("100%");
        } else {
            let percent = complete_bytes / (complete_bytes + left_bytes);
            return percent.to_string() + "%";
        }
    }

    fn have_stringer(complete_bytes: i64) -> String {
        let possible_powers = vec![
            (1024_i64, String::from(" KiB")),
            (1024_i64.pow(2), String::from(" MiB")),
            (1024_i64.pow(3), String::from(" GiB")),
            (1024_i64.pow(4), String::from(" TiB")),
        ];
        if complete_bytes < 1024 {
            return complete_bytes.to_string() + " b";
        }
        for pp in possible_powers.into_iter() {
            if (complete_bytes / pp.0) < 1024 {
                return (complete_bytes / pp.0).to_string() + &pp.1;
            }
        }
        return String::from("unknown");
    }

    pub fn status_maker(
        is_active: bool,
        up_rate: i64,
        down_rate: i64,
        left_bytes: i64,
        is_hashing: bool,
    ) -> String {
        // per https://github.com/transmission/transmission/blob/main/utils/remote.cc#L812 - valid codes are Queued, Finished, Stopped, Verifying, Up & Down, Uploading, Seeding, Idle, Unknown
        //seems like a greater sieve to start with torrents not active

        if !is_active {
            if left_bytes == 0 {
                return String::from("Finished");
            } else {
                // there is a condition "paused" in the rtorrent docs, but this seems like a very immaterial semantic point
                return String::from("Stopped");
            }
        // if active
        //if hashing we don't need to check for anything
        } else if is_hashing {
            return String::from("Verifying");
        } else {
            // if there are still bytes left and its active
            if left_bytes > 0 {
                if up_rate > 0 && down_rate > 0 {
                    return String::from("Up & Down");
                } else if up_rate > 0 && down_rate == 0 {
                    return String::from("Uploading");
                } else if up_rate == 0 && down_rate > 0 {
                    return String::from("Downloading");
                } else {
                    return String::from("Idle");
                }
            } else if left_bytes == 0 {
                // this isn't technically accurate as, some, maybe all, trackers consider "seeding" an incomplete torrent to be uploading a torrent you currently haven't finished downloading
                if up_rate > 0 {
                    return String::from("Seeding");
                } else {
                    return String::from("Idle");
                }
            } else {
                return String::from("Unknown");
            }
        }
    }
}
