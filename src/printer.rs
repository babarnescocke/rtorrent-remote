//#[macro_use] extern crate prettytable;
use prettytable::{Table, row, cell, format};
use crate::xmlrpchelper::TorrentInfo;


pub fn lsprinter(torrents: &Vec<TorrentInfo>) {
	let mut table = Table::new();

	table.set_format(*format::consts::FORMAT_CLEAN);

	table.set_titles(row!["ID", "% Done", "Bytes Have", "ETA", "Up", "Down", "Ratio", "Status", "Name"]);
for t in torrents.iter() {
	table.add_row(row![dotAddtoi16(&t.index_val), t.percent_done(), t.bytes_done, t.seconds_left(), t.up_rate, t.down_rate, t.ratio, t.state, t.name]);

}
	table.printstd();
	let value:i16 = 8;
	dotAddtoi16(&value);
}

fn dotAddtoi16(input: &i16) -> String {
	let mut retstring = String::from(input.to_string());
	retstring.push('.');
	return retstring;
}
