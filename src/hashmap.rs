// collection of stuff to deal with hashmaps and going back and forth between the hash, that we need to manipulate torrents in rtorrent and the torrent ID, which is provided by this program.
pub mod hashhelp {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::collections::HashMap;
    pub fn id_to_hash(id: i64, tempfile: String) -> String {
    	let f = File::open(tempfile).unwrap();
    	let mut reader = BufReader::new(f);
    	let mut line = String::new();
    	reader.read_line(&mut line).unwrap();
    }

    fn hashmap_from_file(tempfile) -> HashMap {
    	
    }
}
