use xmlrpc::{Request, Value};
	

pub fn xmltester(rtorrenturl:&url::Url) {

	let ls_request = Request::new("d.multicall2").arg("").arg("main").arg("d.bytes_done=").arg("d.size_bytes=").arg("d.up.rate=").arg("d.down.rate=").arg("d.state=").arg("d.name=").arg("d.hash=").arg("d.ratio=");

	let request_result = ls_request.call_url(rtorrenturl.as_str()).unwrap();
    
 // should print out ID, % Dne, Have, ETA, UP, DOWN, Ratio, Status Name
    println!("ID, % Done, Have, ETA, UP, Down, Ratio, Status, Name");
    

// the below code finds the number of arrays rtorrent responded with and walks each array putting the values into a torrent struct, each one is sanitarily opened to ensure min panics. rtorrent's xmlrpc returns values alone and not pairs of values, eg. JSON, so this is a pretty ugly parser - but it works.

    for torrent_index_value in 0..request_result.as_array().unwrap().len() {
    	let torrent0 = TorrentInfo {
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
    		ratio: match Value::as_f64(&request_result[torrent_index_value][7]) {
    			None => 0.0,
    			Some(ref x) => *x,
    		}
    	};
println!("{} {} {} {} {} {} {} {} {}", torrent0.index_val, torrent0.percent_done(), torrent0.bytes_done, torrent0.seconds_left(), torrent0.up_rate, torrent0.down_rate, torrent0.ratio, torrent0.state, torrent0.name)
    }


}
struct TorrentInfo {
	index_val: i16, //this is an arbitrary value that we are assigning each torrent
    bytes_done: i64,
    size_bytes: i64,
    up_rate: i64,
    down_rate: i64,
    state: bool,
    name: String,
    hash: String,
    ratio: f64,
}

impl TorrentInfo {
	fn bytesleft(&self) -> i64 {
       self.size_bytes - self.bytes_done
	}

	fn seconds_left(&self) -> String {
		if self.down_rate > 0 {
		let seconds_left = self.bytesleft() / self.down_rate;
		return seconds_left.to_string();
	} else {
		return "Eternity".to_string();
	}

	}

	fn percent_done(&self) -> f64 {
		self.bytes_done as f64 * 100.0 / self.size_bytes as f64
	}

// #[derive(Debug)]
// enum state {
// 	Seeding,
// 	Error,
// 	Downloading,
// 	Checking,
// 	Done
// }
}