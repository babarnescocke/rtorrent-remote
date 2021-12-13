use crate::xmlrpchelper::TorrentInfo;
use std::fs::{File};
use std::time::{SystemTime};
use walkdir::WalkDir;
use std::collections::HashMap;
pub mod deserCompare;
#[allow(non_snake_case)]

//use std::DirEntry::path;

// rtorrent specifies torrents by their bencoded hash. This is a bit of a pain to deal with from a UI/UX perspective as identifying torrents by hash is visually and typographically challening.
//Transmission-remote, and I believe transmission's backend, just issue a runtime-long index of every torrent, indexed at 1. Such that if I want to remove the 100th torrent, transmission-remote -rad -t100, will remove & delete that torrent
// - but it will also mean reissuing that command is meaningless as the 100th torrent no longer exists. The 101st torrent doesn't move into that spot, at least until transmission is restarted.
// This is mostly what I want, to keep track of this, I am going to serialize and deserialize the torrent list in /tmp.

pub fn tempFileDoer(inputDir: String,input: &Vec<TorrentInfo>, rtorrentURL: &url::Url) -> Vec<TorrentInfo> {
	let onlyAlphanumericRtorrentURL: String = rtorrentURL.to_string().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
	let tempFileName = previousRtorrentRemoteJSONS(inputDir.clone(),&onlyAlphanumericRtorrentURL);
	if tempFileName.chars().count() > 0 {
		return writeNewJSONDeleteOldJSONFile(deserCompare::returnRemovedTorrents(input.to_vec(), deserCompare::returnDeserializedHashMap(tempFileName.clone())), inputDir, onlyAlphanumericRtorrentURL.clone(), tempFileName);
	} else {
		createTempFile(vecTorrentInfoToIndexHashKVP(input), inputDir, onlyAlphanumericRtorrentURL.clone());
		return input.to_vec()
	}
} 


fn writeNewJSONDeleteOldJSONFile(inputVecTorInfo: Vec<TorrentInfo>, inputDir: String, rtorrentURL: String, pathToOldJson: String) -> Vec<TorrentInfo> {
	createTempFile(vecTorrentInfoToIndexHashKVP(&inputVecTorInfo), inputDir, rtorrentURL);
	std::fs::remove_file(pathToOldJson);
	inputVecTorInfo
}

fn vecTorrentInfoToIndexHashKVP(input: &Vec<TorrentInfo>) -> HashMap<i16, String>  {
	let mut torrentIndexHashmap: HashMap<i16, String> = HashMap::new();
	for i in input {
		torrentIndexHashmap.insert(i.index_val, i.hash.clone());
	}
	torrentIndexHashmap
}
fn createTempFile(input: HashMap<i16, String>, inputDir: String, rtorrentURL: String) {
	let timeSecUnixEpoch = unixTime();
	serde_json::to_writer(&File::create(format!("{}{}{}.rtorrentremote.json",inputDir,timeSecUnixEpoch,rtorrentURL)).expect("error writing to files"), &input);

}
// surprisingly this is actually a rather compact way to get Unix Time in seconds.
fn unixTime() -> u64 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(time) => {
    	return time.as_secs();
    }
	Err(_) => panic!("systemtime before UNIX_EPOCH"),

}
}


// function to walk /tmp and return string of previous saved json if there has been a previous running of rtorrent remote, returns empty string if none found.
pub fn previousRtorrentRemoteJSONS(inputDir: String, rtorrentURL: &String) -> String {
	for entry in WalkDir::new(inputDir).max_depth(1) {
		let mut containsString: String = rtorrentURL.clone();
		containsString.push_str(".rtorrentremote.json");
		let mut potFilePath: String = entry.as_ref().expect("cannot access file in dir").file_name().to_string_lossy().to_string();
		if potFilePath.contains(&containsString) {
		//at this point we have a temp file we are pretty sure we created, however we don't know if it is still valid, this simply checks if the file was created in the last hour
			let tempFileVecString: Vec<&str> = potFilePath.split(".").collect();
			let timeOfTempFileVecString = tempFileVecString[0].to_string().replace(rtorrentURL, "");
			let timeOfTempFileInt = timeOfTempFileVecString.parse::<u64>().unwrap().clone();
			if (unixTime() - 3600) > timeOfTempFileInt {
				println!("your temp file is too old, please rerun --list");
				return "".to_string()
			} 

			return entry.expect("failure to return file").path().to_string_lossy().to_string()
	}
}
	"".to_string()

}