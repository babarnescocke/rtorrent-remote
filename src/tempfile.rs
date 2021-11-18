use serde::Deserialize;
use crate::xmlrpchelper::TorrentInfo;
use std::fs::{File};
use std::time::{SystemTime};
use std::io::Write;
use walkdir::WalkDir;
//use std::DirEntry::path;

// rtorrent specifies torrents by their bencoded hash. This is a bit of a pain to deal with from a UI/UX perspective as identifying torrents by hash is visually and typographically challening.
//Transmission-remote, and I believe transmission's backend, just issue a runtime-long index of every torrent, indexed at 1. Such that if I want to remove the 100th torrent, transmission-remote -rad -t100, will remove & delete that torrent
// - but it will also mean reissuing that command is meaningless as the 100th torrent no longer exists. The 101st torrent doesn't move into that spot, at least until transmission is restarted.
// This is mostly what I want, to keep track of this, I am going to serialize and deserialize the torrent list in /tmp.

pub fn tempFileDoer(inputDir: String,input: &Vec<TorrentInfo>) -> &Vec<TorrentInfo> {
	if previousRtorrentRemoteJSONS(inputDir.clone()).chars().count() > 0 {
		println!("deserialize data and compare");
		returnRemovedTorrents
		return input
	} else {
		println!("no former tempfile found!");
		createTempFile(input, inputDir);
		return input
	}
} 


fn createTempFile(input: &Vec<TorrentInfo>, inputDir: String) {
	let timeSecUnixEpoch = unixTime();
    	let mut file = File::create(format!("{}{}.rtorrentremote.json", inputDir,timeSecUnixEpoch)).expect("Unable to create file");
	    for i in input{
	    	let jsonLine: String = serde_json::to_string(i).expect("unable to unwrap");
	    	file.write(&jsonLine.as_bytes()).expect("Unable to write data");
	    }
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
fn previousRtorrentRemoteJSONS(inputDir: String) -> String {
	for entry in WalkDir::new(inputDir).max_depth(1) {
		if entry.as_ref().expect("cannot access file in dir").file_name().to_string_lossy().contains("rtorrentremote.json") {
			return entry.expect("failure to return file").file_name().to_string_lossy().to_string()
	}
}
	"".to_string()

}