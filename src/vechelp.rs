/// collection of stuff to deal with vectors and going back and forth between the hash, that we need to manipulate torrents in rtorrent, and the torrent ID, which is provided by this program.
pub mod hashvechelp {

    use crate::torrentstructs::torrentStructs::RtorrentTorrentLSPrintStruct;
    use crc::{Crc, CRC_16_ISO_IEC_14443_3_A};
    use std::error::Error;
    use std::fs::{read_dir, remove_file, File};
    use std::io::prelude::*;
    use std::time::SystemTime;

    //there is probably a more elegant solution here, but there is a non-trivial chance that we will parse a user request to be index out of bounds. And so I would like to catch it especially to know its the most obvious index out of bounds.
    pub fn id_to_hash(vec: Vec<String>, id: i32) -> Result<String, Box<dyn Error>> {
        if (id as usize) <= vec.len() - 1 {
            Ok(vec[id as usize].clone())
        } else {
            Err(format!("Requested id: {} which is out of range", id))?
        }
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
    pub fn delete_old_vecfile(
        path_to_before_rtorrent_remote_temp_file: Option<String>,
    ) -> std::io::Result<()> {
        if path_to_before_rtorrent_remote_temp_file.is_some() {
            remove_file(path_to_before_rtorrent_remote_temp_file.unwrap())?;
        }
        Ok(())
    }
    // this is a simple function that takes a path, and returns a vec
    pub fn file_to_vec(path: String) -> std::result::Result<Vec<String>, Box<dyn Error>> {
        let file = &std::fs::read(path)?;
        Ok(bincode::deserialize(file).unwrap())
    }
    fn path_to_unix_time_of_file_creation(path: String) -> i64 {
        let string_split: Vec<&str> = path.split('.').collect();
        string_split[string_split.len() - 2].parse::<i64>().unwrap()
    }

    pub fn vec_to_file(
        vector: Vec<String>,
        rtorrenturl: String,
        tempdir: String,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let encoded: Vec<u8> = bincode::serialize(&vector)?;
        let mut file = File::create(tempdir + "/" + &new_tempfile_name(rtorrenturl)?)?;
        file.write(&encoded)?;
        Ok(())
    }
    pub fn unix_time_now() -> std::result::Result<u64, Box<dyn Error>> {
        let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(n.as_secs())
    }

    /// just a simple string formatter to create a tempfile - I looked and there doesn't
    pub fn new_tempfile_name(rtorrenturl: String) -> std::result::Result<String, Box<dyn Error>> {
        Ok(String::from(format!(
            ".rtorrent-remote.{}.{}.dat",
            unix_time_now()?.to_string(),
            crc16_checksum(rtorrenturl)
        )))
    }
    fn crc16_checksum(some_string: String) -> String {
        let crc16: Crc<u16> = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
        crc16.checksum(some_string.as_bytes()).to_string()
    }
    pub fn derive_vec_of_hashs_from_torvec(
        vector_of_tor_hashes: &mut Vec<String>,
        torvec: &mut Vec<RtorrentTorrentLSPrintStruct>,
    ) -> std::result::Result<(), Box<dyn Error>> {
        for f in torvec.iter_mut() {
            let hash_unwrapper = f.hash.clone();
            match hash_unwrapper {
                Some(x) => {
                    if vector_of_tor_hashes.contains(&x.clone()) {
                        f.id = vector_of_tor_hashes.iter().position(|i| *i == x).unwrap() as i32;
                    } else {
                        vector_of_tor_hashes.push(x);
                        f.id = (vector_of_tor_hashes.len() - 1) as i32;
                    }
                }
                None => return Err("cannot derive hash from torrent as hash is missing")?,
            }
        }
        Ok(())
    }
}
