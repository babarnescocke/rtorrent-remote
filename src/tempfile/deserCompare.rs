use crate::xmlrpchelper::TorrentInfo;
use serde::Deserialize;
use serde_json;

pub fn returnRemovedTorrents(liveTorVec: &Vec<TorrentInfo>, fromTempTorVec: &Vec<TorrentInfo>) -> Vec<TorrentInfo> {
	liveTorVec.to_vec()
}


pub fn returnDeserializedVec(filePath: String) -> *Vec<TorrentInfo> {
	let mut tempFileTorList = Vec![];
	let path = filePath;
	let data = fs::read_to_string(&path).expect("unable to read from file");
	let res: serde_json::Value = serde_json::from_str(&data).expect("unable to read data from string");
}