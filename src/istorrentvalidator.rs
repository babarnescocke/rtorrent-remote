use std::path::Path;

// a simple function that validates if a given URL or Path - represented here as a string is OK. OK meaning if Path it is a syntactically valid and readable torrent file. If URL that its a readable 
pub fn isTorrent(inString: &str) -> Result<(bool), ValidatorError> {
	if Path::new(inString).isfile() { // will return true only if inString is readable and file
	  if 
	}
}1