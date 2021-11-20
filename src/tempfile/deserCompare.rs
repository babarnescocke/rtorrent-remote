use crate::xmlrpchelper::TorrentInfo;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

// this function takes two vectors of torrentinfos and changes the index of the "live torrent list" to the older one.
pub fn returnRemovedTorrents(liveTorVec: &Vec<TorrentInfo>, fromTempTorHash: &HashMap<i16,String>) -> Vec<TorrentInfo> {
	let mut currentTorList: Vec<TorrentInfo> = vec![];
	for i in liveTorVec {
		if i.index_val == 1 {
			println!("we're here");
		}
	}
	currentTorList
}


pub fn returnDeserializedHashMap(filePath: String) -> HashMap<i16,String> {
	let mut tempHashMap: HashMap<i16, String> = HashMap::new();
	let path = filePath;
	let reader = BufReader::new(File::open(path).expect("unable to read tempfile path"));
	serde_json::from_reader(reader).expect("json reader failed")

}