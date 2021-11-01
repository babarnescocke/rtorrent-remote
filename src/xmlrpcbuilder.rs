use xmlrpc::Request;


pub fn xmltester(rtorrenturl:&url::Url) {
	let ls_request = Request::new("d.multicall2").arg("").arg("main").arg("d.bytes_done=").arg("d.size_bytes=").arg("d.up.rate=").arg("d.down.rate=").arg("d.state=").arg("d.name=");

	let request_result = ls_request.call_url(rtorrenturl.as_str()).unwrap();
    
 // should print out ID, % Dne, Have, ETA, UP, DOWN, Ratio, Status Name
    println!("ID, % Done, Have, ETA, UP, Down, Ratio, Status, Name");
    
}

// fn donepercent(donebytes:i64, bytes:i64) -> f64 {
// 	return donebytes as f64/ bytes as f64;
// }

