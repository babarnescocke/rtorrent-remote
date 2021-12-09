use crate::xmlrpchelper::TorrentInfo;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

// this function takes a vector of torrent infos, a hashmap previously written and compares hashes from the tempfile to the current vector of torrents. This attempts to preserve the index through each running of the application. Its inefficient as we need to walk the hashmap and the vector, but it gets us to where we need to be.
pub fn returnRemovedTorrents(liveTorVec: Vec<TorrentInfo>, fromTempTorHash: HashMap<i16,String>) -> Vec<TorrentInfo> {
	let mut returnTorVec :Vec<TorrentInfo> = vec![];
	let mut garbageIter: i16 = (fromTempTorHash.len()).try_into().expect("cannot find length of hashmaps");
	// the below variable is needed because we need keep track of possible index values.
	let mut garbageCountTwoElectricBugaloo = garbageIter + 1;
	//we walk the hashmap from the json file. If it is a match we can just add that torrentinfo to the vector we return, else we add it to the end. 
	for i in 0..garbageIter {
		if fromTempTorHash.get(&i).unwrap().eq(&liveTorVec[i as usize].hash) {
			let mut torInfo: TorrentInfo = liveTorVec[i as usize].clone();
			torInfo.index_val = i.into();
			returnTorVec.push(torInfo);
		} else {
			if i <= liveTorVec.len() as i16 {
			let mut torInfo: TorrentInfo = liveTorVec[i as usize].clone();
			torInfo.index_val = garbageCountTwoElectricBugaloo.into();
			garbageCountTwoElectricBugaloo+=1;
			returnTorVec.push(torInfo);
		}
		}
	}
	// the below covers if the current live view of the torrents is larger than the 
	if garbageIter < liveTorVec.len() as i16 {
		for i in garbageIter..liveTorVec.len() as i16 {
			let mut torInfo: TorrentInfo = liveTorVec[i as usize].clone();
			torInfo.index_val = garbageCountTwoElectricBugaloo.into();
			garbageCountTwoElectricBugaloo+=1;
			returnTorVec.push(torInfo);
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