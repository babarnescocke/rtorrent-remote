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
            id,
            hash,
            done: done_stringer(complete_bytes.clone(), left_bytes.clone()),
            have: bytes_to_IEC_80000_13_string(complete_bytes),
            eta: eta_maker(left_bytes.clone(), down_rate.clone()),
            down_rate: down_rate.clone().to_string(),
            up_rate: up_rate.clone().to_string(),
            ratio: format!("{:.1}", ratio),
            status: status_maker(
                is_active,
                up_rate.clone(),
                down_rate.clone(),
                left_bytes,
                hashing,
            ),
             name,
            raw_bytes_have: complete_bytes,
            raw_up: up_rate,
            raw_down: down_rate,
        }
    }

    // This is a struct that builds a torrent from information that rtorrent provides
    #[derive(Debug)]
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
        pub raw_bytes_have: i64,
        pub raw_up: i64,
        pub raw_down: i64,
    }
    impl RtorrentTorrentLSPrintStruct {
        pub fn to_vec_of_strings(&self) -> Vec<String> {
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

    /// Takes a number in bytes and returns a string of time left, or N/A an indeterminate division or Done.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let bytes_left = 10;
    /// let down_rate = 10;
    /// assert_eq!(eta_maker(bytes_left, down_rate), 1s);
    /// ```
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

    /// Takes a number of completed bytes and bytes left to download and returns the percent we have completed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// assert_eq!(done_stringer(10, 0), 100%);
    /// ```
    fn done_stringer(complete_bytes: i64, left_bytes: i64) -> String {
        if left_bytes == 0 {
            return String::from("100%");
        } else {
            let percent = complete_bytes / (complete_bytes + left_bytes);
            return percent.to_string() + "%";
        }
    }
    /// Simple function that walks powers of 1024 and returns the given IEC 8000-13 compliant string with KiB, GiB, TiB, PiB suffix.
    pub fn bytes_to_IEC_80000_13_string(bytes: i64) -> String {
        let possible_powers = vec![
            (1024_i64, String::from(" KiB")),
            (1024_i64.pow(2), String::from(" MiB")),
            (1024_i64.pow(3), String::from(" GiB")),
            (1024_i64.pow(4), String::from(" TiB")),
            (1024_i64.pow(5), String::from(" PiB")),
        ];
        if bytes < 1024 {
            return bytes.to_string() + " B";
        }
        for pp in possible_powers.into_iter() {
            if (bytes / pp.0) < 1024 {
                return (bytes / pp.0).to_string() + &pp.1;
            }
        }
        return String::from("unknown");
    }
    /// Untangles the mess from rtorrent to get the appropriate string for status - such as - "Seeding", "Downloading" etc. I have checked pretty thoroughly, this is basically my best option.
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

    //this function takes f.multicall outputs (https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#f-commands) and makes an RtorrentFileInfoStruct for passing to printer
    pub fn new_file_info_struct_maker(
        number: i32,
        number_of_completed_chunks: i64,
        number_of_total_chunks: i64,
        priority_from_rtorrent: i64,
        size_bytes: i64,
        path: String,
    ) -> RtorrentFileInfoStruct {
        let priority_get_tuple = priorty_num_to_tuple_priority_and_get(priority_from_rtorrent);
        RtorrentFileInfoStruct {
             number,
            done: done_stringer(
                number_of_completed_chunks,
                number_of_completed_chunks - number_of_total_chunks,
            ),
            priority: priority_get_tuple.0,
            get: priority_get_tuple.1,
            size: bytes_to_IEC_80000_13_string(size_bytes),
             path,
        }
    }
    // the different priority levels for files according to rtorrent docs - https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html#term-f-priority
    //this func takes the input from rtorrent xmlrpc and returns a tuple of both "priority" and "get".
    fn priorty_num_to_tuple_priority_and_get(input_val: i64) -> (String, String) {
        if input_val == 0 {
            (String::from("Don't Download"), String::from("No"))
        } else if input_val == 1 {
            (String::from("Normal"), String::from("Yes"))
        } else if input_val == 2 {
            (String::from("High"), String::from("Yes"))
        } else {
            (
                String::from("Unknown/Invalid Input"),
                String::from("Unknown/Invalid Input"),
            )
        }
    }

    #[derive(Debug)]
    pub struct RtorrentFileInfoStruct {
        pub number: i32,
        pub done: String,
        pub priority: String,
        pub get: String,
        pub size: String,
        pub path: String,
    }

    impl RtorrentFileInfoStruct {
        pub fn to_vec_of_strings(&self) -> Vec<String> {
            vec![
                self.number.clone().to_string() + ":",
                self.done.clone(),
                self.priority.clone(),
                self.get.clone(),
                self.size.clone(),
                self.path.clone(),
            ]
        }
    }
    #[derive(Debug)]
    pub struct RtorrentPeerStruct {
        pub ip_addr: String,
        pub encrypted: bool,
        pub done: String,
        pub down: String,
        pub up: String,
        pub client: String,
    }
    impl RtorrentPeerStruct {
        pub fn to_vec_of_strings(&self) -> Vec<String> {
            vec![
                self.ip_addr.clone(),
                self.encrypted.clone().to_string(),
                self.done.clone(),
                self.down.clone(),
                self.up.clone(),
                self.client.clone(),
            ]
        }
    }
    pub fn new_peer_struct_maker(
        ip: String,
        encrypted: bool,
        done: i64,
        down: i64,
        up: i64,
        client: String,
    ) -> RtorrentPeerStruct {
        RtorrentPeerStruct {
            ip_addr: ip,
            encrypted,
            done: done.to_string(),
            down: down.to_string(),
            up: up.to_string(),
            client,
        }
    }
    #[derive(Debug)]
    pub struct RtorrentDeepTorrentInfo {
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
        pub raw_bytes_have: i64,
        pub raw_up: i64,
        pub raw_down: i64,
    }
}
