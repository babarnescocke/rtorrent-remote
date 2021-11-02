use xmlrpc::{Request, Value};
	

pub fn xmltester(rtorrenturl:&url::Url) {

	let ls_request = Request::new("d.multicall2").arg("").arg("main").arg("d.bytes_done=").arg("d.size_bytes=").arg("d.up.rate=").arg("d.down.rate=").arg("d.state=").arg("d.name=").arg("d.hash=");

	let request_result = ls_request.call_url(rtorrenturl.as_str()).unwrap();
    
 // should print out ID, % Dne, Have, ETA, UP, DOWN, Ratio, Status Name
    println!("ID, % Done, Have, ETA, UP, Down, Ratio, Status, Name");
    
  //  println!("The useful size of resutlt is {}", std::mem::size_of_val(&request_result));

    // println!("This is the 1st Item in the array, the total number of bytes done {:?}", request_result[0][0]);
    // println!("This is the 2nd Item in the array, the total size in bytes {:?}", request_result[0][1]); 
    // println!("This is the 3rd Item in the array, the up rate {:#?}", request_result[0][2].as_i64());
    // println!("This is the 4th Item in the array, the down rate {:?}", request_result[0][3]);
    // println!("This is the 5th Item in the array, the state {:?}", request_result[0][4]); 
    // println!("This is the 6th Item in the array, the name {:?}", request_result[0][5]);
    // println!("This is the 7th Item in the array, the hash {}", 
    // 	match Value::as_str(&request_result[0][6]) {
    // 		None => "No hash",
    // 		Some(ref x) => x,
    // 	}); 

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
    		Name: match Value::as_str(&request_result[torrent_index_value][5]) {
    			None => "Torrent with No Name".to_string(),
    			Some(ref x) => x.to_string(),
    		},
    		Hash: match Value::as_str(&request_result[torrent_index_value][6]) {
    			None => "torrent with no hash".to_string(),
    			Some(ref x) => x.to_string(),
    		}
    	};
println!("{}", torrent0.Name)
    }


}
struct TorrentInfo {
	index_val: i16, //this is an arbitrary value that we are assigning each torrent
    bytes_done: i64,
    size_bytes: i64,
    up_rate: i64,
    down_rate: i64,
    state: bool,
    Name: String,
    Hash: String,
}