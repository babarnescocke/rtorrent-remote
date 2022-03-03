// collection of stuff to deal with hashmaps and going back and forth between the hash, that we need to manipulate torrents in rtorrent and the torrent ID, which is provided by this program.
pub mod hashhelp {

    use crc::{Crc, CRC_16_ISO_IEC_14443_3_A};
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::{read_dir, remove_file, File};
    use std::io::prelude::*;
    use std::time::SystemTime;

    pub fn id_to_hash(hashmap: HashMap<String, i32>, id: i32) -> Option<String> {
        for (k, v) in hashmap.iter() {
            if v == &id {
                return Some(k.clone());
            }
        }
        return None;
    }
    pub fn tempfile_finder(
        tempdir: String,
        rtorrenturl: String,
    ) -> std::result::Result<Option<String>, Box<dyn Error>> {
        for entry in read_dir(tempdir)? {
            let entry = entry?;
            let path = entry.path();
            if path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
                .contains(&crc16_checksum(rtorrenturl.clone()))
            {
                let val = path.into_os_string().into_string().unwrap();
                return Ok(Some(val));
            }
        }
        Ok(None)
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
    pub fn file_to_hashmap(
        path: String,
    ) -> std::result::Result<HashMap<String, i32>, Box<dyn Error>> {
        let file = &std::fs::read(path)?;
        Ok(bincode::deserialize(file).unwrap())
    }

    pub fn hashmap_to_file(
        hashmap: HashMap<String, i32>,
        rtorrenturl: String,
        tempdir: String,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let encoded: Vec<u8> = bincode::serialize(&hashmap)?;
        let mut file = File::create(tempdir + "/" + &new_tempfile_name(rtorrenturl)?)?;
        file.write(&encoded)?;
        Ok(())
    }
    pub fn unix_time_now() -> std::result::Result<String, Box<dyn Error>> {
        let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(n.as_secs().to_string()) /* {
                                        Ok(n) => Ok(n.as_secs().to_string()),
                                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                                    }*/
    }

    pub fn tempdir_to_tempfile(
        tempdir: String,
        rtorrenturl: String,
    ) -> std::result::Result<Option<String>, Box<dyn Error>> {
        for f in read_dir(tempdir)? {
            if f.as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
                .contains(&rtorrenturl)
            {
                return Ok(Some(f?.path().into_os_string().into_string().unwrap()));
            }
        }
        Ok(None)
    }
    /// just a simple string formatter to create a tempfile - I looked and there doesn't
    pub fn new_tempfile_name(
        rtorrenturl: String,
    ) -> std::result::Result<String, Box<dyn std::error::Error>> {
        Ok(String::from(format!(
            ".rtorrent-remote.{}.{}.dat",
            unix_time_now()?,
            crc16_checksum(rtorrenturl)
        )))
    }
    fn crc16_checksum(some_string: String) -> String {
        let crc16: Crc<u16> = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
        crc16.checksum(some_string.as_bytes()).to_string()
    }
}
