/// collection of stuff to deal with vectors and going back and forth between the hash, that we need to manipulate torrents in rtorrent, and the torrent ID, which is provided by this program.
pub mod hashvechelp {

    use crate::torrentstructs::torrentStructs::RtorrentTorrentLSPrintStruct;
    use crc::{Crc, CRC_16_ISO_IEC_14443_3_A};
    use std::error::Error;
    use std::fs::{read, read_dir, remove_file, write};
    use std::io::Cursor;
    use std::time::SystemTime;
    use zstd::{decode_all, encode_all};

    ///there is probably a more elegant solution here, but there is a non-trivial chance that we will parse a user request to be index out of bounds. And so I would like to catch it especially to know its the most obvious index out of bounds.
    pub fn id_to_hash(vec: Vec<String>, id: i32) -> Result<String, Box<dyn Error>> {
        if (id as usize) <= vec.len() - 1 {
            Ok(vec[id as usize].clone())
        } else {
            Err(format!("Requested id: {} which is out of range", id))?
        }
    }
    /// walks a specific directory and looks for a file with the specific checksum from our rtorrenturl. There is a logical issue here where there is a non-trivial chance of someone using the same rtorrent server instance, but slightly different URLs, eg: http://localhost:80/RPC2 http://localhost/RPC2, which will create two independent files.
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

    /// Deletes some path
    pub fn delete_old_vecfile(
        path_to_before_rtorrent_remote_temp_file: Option<String>,
    ) -> std::io::Result<()> {
        if path_to_before_rtorrent_remote_temp_file.is_some() {
            remove_file(path_to_before_rtorrent_remote_temp_file.unwrap())?;
        }
        Ok(())
    }
    /// serializes and compresses a vec of torrent hashes.
    pub fn vec_to_zstd_file(
        vector: Vec<String>,
        rtorrenturl: String,
        tempdir: String,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let zstded = encode_all(Cursor::new(bincode::serialize(&vector)?), 12)?;
        write(tempdir + "/" + &new_tempfile_name(rtorrenturl)?, zstded)?;
        Ok(())
    }

    /// takes a tempfile path, deserializes, uncompresses the vec of hashes stored within.
    pub fn zstd_file_to_vec(path: String) -> std::result::Result<Vec<String>, Box<dyn Error>> {
        let unzstded = decode_all(Cursor::new(&read(path)?))?;
        Ok(bincode::deserialize(&unzstded[..])?)
    }

    /// a function so that we can know when such a file was created.
    pub fn unix_time_now() -> std::result::Result<u64, Box<dyn Error>> {
        let n = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(n.as_secs())
    }

    /// just a simple string formatter to create a tempfile name using time and rtorrenturl.
    pub fn new_tempfile_name(rtorrenturl: String) -> std::result::Result<String, Box<dyn Error>> {
        Ok(String::from(format!(
            ".rtorrent-remote.{}.{}.dat",
            unix_time_now()?.to_string(),
            crc16_checksum(rtorrenturl)
        )))
    }

    /// creates a checksum so that we don't have to store a URL or URL with username:password in plain text.
    fn crc16_checksum(some_string: String) -> String {
        let crc16: Crc<u16> = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
        crc16.checksum(some_string.as_bytes()).to_string()
    }

    /// a function that compares a vector of hashes and a vector of specialized structs to see if we need adjust the order of the list from rtorrent. If the hash isn't in our vector of torrent hashes - we push that hash onto the end.
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
