// collection of stuff to deal with hashmaps and going back and forth between the hash, that we need to manipulate torrents in rtorrent and the torrent ID, which is provided by this program.
pub mod hashhelp {

    use crc::{Crc, CRC_16_ISO_IEC_14443_3_A};
    use std::collections::HashMap;
    use std::fs::{read_dir, remove_file, File};
    use std::io::prelude::*;
    use std::io::ErrorKind;
    use std::time::SystemTime;
    pub fn id_to_hash(id: i64, tempfile: String) -> String {
        let f = File::open(tempfile).unwrap();

        let mut line = String::new();
        //reader.read_line(&mut line).unwrap();
        todo!();
    }
    pub fn tempfile_finder(tempdir: String, rtorrenturl: String) -> Option<String> {
        for entry in read_dir(tempdir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
                .contains(&crc16_checksum(rtorrenturl.clone()))
            {
                return Some(path.into_os_string().into_string().unwrap());
            }
        }
        return None;
    }
    pub fn delete_old_hashmap(
        path_to_before_rtorrent_remote_temp_file: Option<String>,
    ) -> std::io::Result<()> {
        if path_to_before_rtorrent_remote_temp_file.is_some() {
            remove_file(path_to_before_rtorrent_remote_temp_file.unwrap())?;
        }
        Ok(())
    }
    // this is a simple function that takes a path, and returns a hashmap
    pub fn file_to_hashmap(path: String) -> HashMap<String, i32> {
        let mut file = &std::fs::read(path).unwrap();
        bincode::deserialize(file).unwrap()
    }
    pub fn hashmap_to_file(hashmap: HashMap<String, i32>, rtorrenturl: String, tempdir: String) {
        let encoded: Vec<u8> = bincode::serialize(&hashmap).unwrap();
        let mut file = File::create(tempdir + "/" + &new_tempfile_name(rtorrenturl)).unwrap();
        file.write(&encoded).unwrap();
        drop(file);
    }
    pub fn unix_time_now() -> String {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => return n.as_secs().to_string(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    pub fn tempdir_to_tempfile(tempdir: String, rtorrenturl: String) -> Option<String> {
        for f in read_dir(tempdir).unwrap() {
            if f.as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
                .contains(&rtorrenturl)
            {
                return Some(f.unwrap().path().into_os_string().into_string().unwrap());
            }
        }
        None
    }
    /// just a simple string formatter to create a tempfile - I looked and there doesn't
    pub fn new_tempfile_name(rtorrenturl: String) -> String {
        String::from(format!(
            ".rtorrent-remote.{}.{}.dat",
            unix_time_now(),
            crc16_checksum(rtorrenturl)
        ))
    }
    fn crc16_checksum(some_string: String) -> String {
        let crc16: Crc<u16> = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
        crc16.checksum(some_string.as_bytes()).to_string()
    }
}
