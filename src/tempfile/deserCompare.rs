use crate::xmlrpchelper::TorrentInfo;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

// this function takes two vectors of torrentinfos and changes the index of the "live torrent list" to the older one.
pub fn returnRemovedTorrents(liveTorVec: Vec<TorrentInfo>, fromTempTorHash: HashMap<i16,String>) -> Vec<TorrentInfo> {
	let mut returnTorVec :Vec<TorrentInfo> = vec![];
	let mut garbageIter = fromTempTorHash.len() - 1;
	//we walk the vector that the server has returned and determine if this is known to our 
	for i in 0..liveTorVec.len() {
		if liveTorVec[(i as usize)].hash.eq(fromTempTorHash.get(&(i as i16)).expect("Couldn't retrieve val")) {
			returnTorVec.insert(i, liveTorVec[i as usize].clone());
		} 
	}
	returnTorVec
}



pub fn returnDeserializedHashMap(filePath: String) -> HashMap<i16,String> {
	let mut tempHashMap: HashMap<i16, String> = HashMap::new();
	let path = filePath;
	let reader = BufReader::new(File::open(path).expect("unable to read tempfile path"));
	serde_json::from_reader(reader).expect("json reader failed")

}