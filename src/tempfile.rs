use serde::Deserialize;
use crate::xmlrpchelper::TorrentInfo;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime};

// rtorrent specifies torrents by their bencoded hash. This is a bit of a pain to deal with from a UI/UX perspective as identifying torrents by hash is visually and typographically challening. Transmission-remote, and I believe transmission's backend, just issue a runtime-long index of every torrent, indexed at 1. Such that if I want to remove the 100th torrent, transmission-remote -rad -t100, will remove & delete that torrent - but it will also mean reissuing that command is meaningless as the 100th torrent no longer exists. The 101st torrent doesn't move into that spot, at least until transmission is restarted. This is mostly what I want, to keep track of this, I am going to serialize and deserialize the torrent list in /tmp.



pub fn createTempFile(input: Vec<TorrentInfo>) {
    	let mut buffer = File::create(format!("{}.json",unixTime())).expect("Unable to create file");
	    for i in &input{
	    	let jsonLine: String = serde_json::to_string(i)?;
	    	buffer.write(jsonLine).expect("Unable to write data");
	    }
}

fn unixTime() -> u64 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(time) => {
    	return time.as_secs();
    }
	Err(_) => panic!("systemtime before UNIX_EPOCH"),

}
}