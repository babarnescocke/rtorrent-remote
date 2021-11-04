//#[macro_use] extern crate prettytable;
use prettytable::{Table, row, cell, format};
use crate::xmlrpchelper::TorrentInfo;
use num::pow;



// this is just a simple function that takes a reference to a vector of torrentInfos, and then prints them. The table, function here is probably overkill, but I want it to reliably print rows and columns with even spacing - so this was least resistence stuff.

pub fn lsprinter(torrents: &Vec<TorrentInfo>) {
	let mut table = Table::new();
    // this sets the format to be w/o lines
	table.set_format(*format::consts::FORMAT_CLEAN);
    // headers
	table.set_titles(row!["ID", "% Done", "Have", "ETA", "Up", "Down", "Ratio", "Status", "Name"]);
	// walks the vector of torrentinfos and adds each line too 
for t in torrents.iter() {
	table.add_row(row![t.index_val,
		t.percent_done(),
		t.bytes_done,
		t.seconds_left(),
		t.getUpBytesStr(),
		t.getDownBytesStr() ,
		format!("{:.2}", t.getRatio()),
		t.status(),
		t.name]);

}
	table.printstd();

}







