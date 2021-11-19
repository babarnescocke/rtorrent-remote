use crate::xmlrpchelper::TorrentInfo;
use serde::Deserialize;
use serde_json;
use std::fs;
pub fn returnRemovedTorrents(liveTorVec: &Vec<TorrentInfo>, fromTempTorVec: &Vec<TorrentInfo>) -> Vec<TorrentInfo> {
	liveTorVec.to_vec()
}


pub fn returnDeserializedVec(filePath: String) -> &Vec<TorrentInfo> {
	let mut tempFileTorList = vec![];
	let path = filePath;
	let data = fs::read_to_string(&path).expect("unable to read from file");
	let res: TorrentInfo = serde_json::from_str(&data).expect("unable to read data from string");
	tempFileTorList.push(res);
	return tempFileTorList
}