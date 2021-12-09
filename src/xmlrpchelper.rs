use xmlrpc::{Request, Value};
use crate::printer;
use num::pow;
use serde::{Serialize, Deserialize};
use crate::tempfile;
use std::collections::HashMap;
use self::tempfile::deserCompare;

pub fn xmlLister(rtorrenturl:&url::Url) {
    let mut torList = vec![]; 
	let ls_request = Request::new("d.multicall2").arg("").arg("main").arg("d.bytes_done=").arg("d.size_bytes=").arg("d.up.rate=").arg("d.down.rate=").arg("d.state=").arg("d.name=").arg("d.hash=").arg("d.ratio=").arg("d.is_hash_checking=").arg("d.is_open=").arg("d.is_active=").arg("d.down.total=").arg("d.up.total=");

	let request_result = ls_request.call_url(rtorrenturl.as_str()).unwrap();
    

// the below code finds the number of arrays rtorrent responded with and walks each array putting the values into a torrent struct, each one is sanitarily opened to ensure min panics. rtorrent's xmlrpc returns values alone and not pairs of values, eg. JSON, so this is a pretty ugly parser - but it works.

    for torrent_index_value in 0..request_result.as_array().unwrap().len() {
    	let torrent = TorrentInfo {
    		index_val: torrent_index_value as i16,
    		bytes_done: match Value::as_i64(&request_result[torrent_index_value][0]) {
    			None => 0,
    			Some(x) => x,
    		},
    		size_bytes: match Value::as_i64(&request_result[torrent_index_value][1]) {
    			None => 0,
    			Some(x) => x,
    		},
    		up_rate: match Value::as_i64(&request_result[torrent_index_value][2]) {
    			None => 0,
    			Some(x) => x,
    		},
    		down_rate: match Value::as_i64(&request_result[torrent_index_value][3]) {
    			None => 0,
    			Some(x) => x,
    		},
    		state: match Value::as_bool(&request_result[torrent_index_value][4]) {
    			None => false,
    			Some(x) => x,
    		},
    		name: match Value::as_str(&request_result[torrent_index_value][5]) {
    			None => "Torrent with No Name".to_string(),
    			Some(ref x) => x.to_string(),
    		},
    		hash: match Value::as_str(&request_result[torrent_index_value][6]) {
    			None => "torrent with no hash".to_string(),
    			Some(ref x) => x.to_string(),
    		},
    		ratio: match Value::as_f64(&request_result[torrent_index_value][7]) { // this is useless
    			None => 0.0,
    			Some(ref x) => *x,
    		},
    		isHashing: match Value::as_bool(&request_result[torrent_index_value][8]) {
    			None => false,
    			Some(ref x) => *x,
    		},
    		isOpen: match Value::as_bool(&request_result[torrent_index_value][9]) {
    			None => false,
    			Some(ref x) => *x,
    		},
    		isActive: match Value::as_bool(&request_result[torrent_index_value][10]) {
    			None => false,
    			Some(ref x) => *x,
    		},
    		totalDown: match Value::as_i64(&request_result[torrent_index_value][11]) {
    			None => 0,
    			Some(ref x) => *x,
    		},
    		totalUp: match Value::as_i64(&request_result[torrent_index_value][12]){
    			None => 0,
    			Some(ref x) => *x,
    		}
    		};
     torList.push(torrent); 


}
       printer::lsprinter(tempfile::tempFileDoer("/tmp/".to_string(), &torList, rtorrenturl));
       //printer::lsprinter(&torList);  


}

pub fn deleteTorrent(numTorrent: i16, inputDir: String, rtorrenturl: &url::Url){
	let pathToJson = tempfile::previousRtorrentRemoteJSONS(inputDir, &rtorrenturl.to_string());
	if pathToJson.chars().count() > 0 {
	let deleteReq = Request::new("d.erase").arg(hashFromIndex(numTorrent, &tempfile::deserCompare::returnDeserializedHashMap(pathToJson)));
	let deleteReqReq = deleteReq.call_url(rtorrenturl.as_str()).unwrap();
} else {
	println!("Error, cannot delete entry as no list of torrents exists, please run --list on the current url before deleting anything.");
}
}

pub fn hashFromIndex(numTorrent: i16, hashMap: &HashMap<i16, String>) -> String {
	return hashMap.get(&numTorrent).expect("Cannot find relevant value for that key").to_string();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TorrentInfo {
	pub index_val: i16, //this is an arbitrary value that we are assigning each torrent
    pub bytes_done: i64,
    pub size_bytes: i64,
    pub up_rate: i64,
    pub down_rate: i64,
    pub state: bool,
    pub name: String,
    pub hash: String,
    pub ratio: f64,
    pub isHashing: bool,
    pub isOpen: bool,
    pub isActive: bool,
    pub totalDown: i64,
    pub totalUp: i64,
}


	// I want percents to print .0 after everything but 100 - this is just a hacky solution.
	pub fn percenter(inputPercent: &f64) -> String {
		if *inputPercent == 100.0 {
			inputPercent.to_string()
		} else {
			format!("{:.1}", inputPercent)
		}
	}
		fn whatPower(base: &i64, toUnsquare: i64) -> i64 {
	if base > &0 {
	for i in 1..8 { // this will iterate over possible powers of 1024^0, 
       if base / pow(toUnsquare, i) < 1 {
    	 let retval :i64 = (i - 1).try_into().unwrap();
    	 return retval;
       }
	}
	}
	//return "input needs to be >0 ".to_string();
	0
	}

	fn iec_Bytes(input: &i64) -> String {
	   let power = whatPower(input, 1024);
	   let roundedDividedDown :i64 = input / pow(1024, power as usize);
	   let mut retVal :String = format!("{:.3}", roundedDividedDown);
	   match power {
		 0 => retVal.push_str(" B"),
		 1 => retVal.push_str(" KiB"),
		 2 => retVal.push_str(" MiB"),
		 3 => retVal.push_str(" GiB"),
		 4 => retVal.push_str(" TiB"),
		 5 => retVal.push_str(" PiB"),
		 6 => retVal.push_str(" EiB"),
		 7 => retVal.push_str(" ZiB"),
		 _ => retVal.push_str("nope")
	}
	return retVal;
	}

impl TorrentInfo {
	pub fn index_val_mut(&mut self) -> &mut i16{
		&mut self.index_val

	}



	pub fn bytesleft(&self) -> i64 {
       self.size_bytes - self.bytes_done
	}

	pub fn seconds_left(&self) -> String {
		if self.down_rate > 0 {
		let seconds_left = self.bytesleft() / self.down_rate;
		return seconds_left.to_string();
	} else {
		return "0".to_string();
	}

	}

	pub fn getRatio(&self) -> f64 {
		if self.totalDown == 0 {
			return 0.0
		} else {
        self.totalDown as f64 / self.totalUp as f64
		}
	   }
	pub fn getUpBytesStr(&self) -> String {
		self::iec_Bytes(&self.up_rate) + "/s"
	}
	pub fn getDownBytesStr(&self) -> String {
		self::iec_Bytes(&self.down_rate) + "/s"
	}
	pub fn getBytesDoneStr(&self) -> String {
		self::iec_Bytes(&self.bytes_done)
	}
	   // function takes an int and gives back the appropriate IEC level name and rounded value. e.g. 1024 in, 1 KiB out.
      // There are a couple crates that do this but I fell asleep with this solution in my head, so here we go. We are sieving out values as we go. If the value is > 1024 we ask if it is bigger than 1024^2, if it is bigger than 1024^3 etc until we reach the appropriate level, and then it is hardcoded, Kibibyte, Megibytes etc.


	fn percent_done(&self) -> f64 {
		self.bytes_done as f64 * 100.0 / self.size_bytes as f64
	}

	pub fn percent_print(&self) -> String {
		if self.percent_done() == 100.0 {
			self.percent_done().to_string()
		} else {
			let retVal = format!("{:.1}", self.percent_done());
			retVal.to_string()
		}
	}
    // kind of insane but there is no out of the box "status" string for xmlrpc. So I am implementing a function that returns the status.
	pub fn status(&self) -> &str{
		if self.bytes_done == self.size_bytes && !self.state {
				return &"Seeding";
		} else if self.isHashing {
			return &"Hashing"
		} else if self.isOpen && !self.isActive {
			return &"Paused"
		} else if !self.isOpen && !self.state {
			if self.bytes_done == self.size_bytes {
				return &"Done"
			} else {
			return &"Stopped"
		    } 
       } else if self.bytes_done != self.size_bytes {
       	return &"Downloading"

       } else {
			return &"Error";
		}

	}



}

pub fn addTorrentFromURL(urlString: &String, rtorrentURL: &url::Url) {
    let add_request = Request::new("load.verbose").arg("").arg(urlString.to_string());
	let request_result = add_request.call_url(rtorrentURL.as_str()).unwrap();
}