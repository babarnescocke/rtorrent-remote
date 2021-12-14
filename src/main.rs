#![allow(non_snake_case)]
use structopt::StructOpt;
use url::Url;



#[derive(Debug, StructOpt)]
#[structopt(name = "rtorrent-remote", about = "a transmission-remote-like client for rtorrent")]
 struct Cli {


  /// Add Torrent
  // Add torrent by filename, URL or Magnet URL
  #[structopt(short = "a", long = "add")]
  addtorrent: Option<Vec<String>>,

  /// Incomplete Directory
  // Where to store new torrents until they are complete
  #[structopt(short = "c", long = "incomplete-dir")]
  incompletedir: Option<String>,

  /// No Incomplete Directory
  // Don't store incomplete torrents in a different location
  #[structopt(short = "C", long = "no-incomplete-dir")]
  incompletedirbool: bool,

  /// Debug
  // Print Debug information
  #[structopt(short, long)]
  debug: bool,

  // Cache
  // Set the maximum size of the sessions memory cache. Reset if rtorrent is restarted or closed.
  #[structopt(short = "e", long = "cache")]
  cachesize: Option<i32>,

  /// Exit
  // Tell rtorrent to close down
  #[structopt(long = "exit")]
  exitrtorrent: bool,

  /// Files
  // List the current torrent(s) files.
  #[structopt(short = "f", long = "files")]
  files: bool,

  /// Info
  // Show the current torrent(s) details
  #[structopt(long = "info")]
  infobool: bool,

  /// Info Files
  // List the current torrent(s) files.
  #[structopt(long = "info-files")]
  infofilebool: bool,

  /// Info Peers
  // List the current torrent(s)' peers.
  #[structopt( long = "info-peers")]
  infopeerbool: bool,

  /// Info pieces
  // List the current torrent(s)' pieces.
  #[structopt(long = "info-pieces")]
  infopieces: bool,

  /// Info trackers
  // List the current torrent(s) trackers.
  #[structopt(long = "info-trackers")]
  infotracker: bool,

  /// Session info
  // show the session's detail
  #[structopt(long = "session-info")]
  sessioninfo: bool,

  /// Session stats
  // Show the session's statistics
  #[structopt(long = "session-stats")]
  sessionstats: bool,

  /// List Torrents
  // List torrents
  #[structopt(short = "l", long = "list")]
  list: bool,

  /// Labels
  // set the current torrent(s)' labels
  #[structopt(short = "L", long = "labels")]
  labels: Option<Option<String>>,

  /// Move
  // Move Current torrent's data to a new folder
  #[structopt(long = "move")]
  movepath: Option<Option<String>>,

  /// Find
  // Tell Transmission where to find a torrent's data.
  #[structopt(long = "find")]
  findpath: Option<Option<String>>,

  // Host
  // the URL of rtorrent
  #[structopt(default_value = "http://localhost:8080/RPC2", parse(try_from_str = Url::parse))]
    /////// https://github.com/rakshasa/rtorrent/wiki/RPC-Setup-XMLRPC gives this as the main 
  rtorrenturl: Url,

  /// Tracker-Add
  // Add tracker to current torrent(s)
  #[structopt(long = "tracker-add")]
  tracker: Option<String>,       

  /// Tracker-Remove
  // Remove Tracker from current torrent(s)'
  #[structopt(long = "tracker-remove")]
  trackerid: Option<String>,

  /// Start Torrent(s)
  //Start the current torrents
  #[structopt(short = "s", long = "start")]
  start: bool,

  /// Stop torrent(s)
  // stop the current torrent(s)
  #[structopt(short = "S", long = "stop")]
  stop: bool,

  /// Start paused
  // Start added torrents paused
  #[structopt(long = "start-paused")]
  starttorpaused: bool,

  #[structopt(long = "remove")]
  remove: bool,


  #[structopt(long = "remove-and-delete", long= "rad")]
  removeAndDelete: bool,
  /// Start added torrents unpaused
  // start added torrents unpaused
  #[structopt(long = "no-start-paused")]
  starttorunpaused: bool,

  /// torrent
  // Set the current torrent(s) for use by subsequent options. The literal all will apply following requests to all torrents; the literal active will apply following requests to recently-active torrents; and specific torrents can be chosen by id or hash.  To set more than one current torrent, join their ids together in a list, such as "-t2,4,6-8" to operate on the torrents whose IDs are 2, 4, 6, 7, and 8.
  #[structopt(short = "t", long = "torrent")]
  torrent: Option<Vec<String>>,

  /// Enable UTP
  #[structopt(long = "utp")]
  utp: bool,

  /// Disable UTP
  #[structopt(long = "no-utp")]
  noutp: bool,

  /// Verify Current Torrent(s)
  #[structopt(long = "verify", short = "V")]
  verify: bool,

  #[structopt(long= "tempdir", default_value = "/tmp/")]
  tempdir: String,
}

mod xmlrpchelper;
mod printer;
mod tempfile;
fn main() {

   let cliargs = Cli::from_args();
 //println!("{:?}", cliargs.rtorrenturl);
  // if user passes list argument, pass url to list function, and print.
  if cliargs.list {

     xmlrpchelper::xmlLister(&cliargs.rtorrenturl);

  // else if user passes some -t or torrents we need to parse those and do some action
  } else if cliargs.torrent.is_some() { 
    if cliargs.torrent.as_ref().unwrap().len() > 0 {
      for torr in cliargs.torrent.unwrap().iter().into_iter() {
        // user has passed some value for t - we are going to walk the options that need to be checked for that.
        torrentHelper(torr,cliargs.rtorrenturl.clone(),cliargs.tempdir.clone(),cliargs.incompletedir.clone(),cliargs.files,cliargs.infobool,cliargs.infofilebool,cliargs.infopieces,cliargs.infotracker,cliargs.labels.clone(),cliargs.movepath.clone(),cliargs.findpath.clone(),cliargs.tracker.clone(),cliargs.trackerid.clone(),cliargs.stop,cliargs.start,cliargs.verify, cliargs.remove, cliargs.removeAndDelete);
      }
    } else {
        println!("torrent flag specified, no torrents provided");
      }
    
  
  } else if cliargs.addtorrent.is_some() {
    if cliargs.addtorrent.as_ref().unwrap().len() > 0 {
      for possibleTor in cliargs.addtorrent.unwrap().into_iter() {
         if isUrl(&possibleTor) {
           xmlrpchelper::addTorrentFromURL(&possibleTor, &cliargs.rtorrenturl);
         } else if isPath(possibleTor.clone()) {
         println!("{} is a torrent file", possibleTor.to_string());
       }
    }

  }

} else {
  rtorrenthelper(&cliargs.rtorrenturl, cliargs.incompletedir, cliargs.incompletedirbool,cliargs.cachesize,cliargs.exitrtorrent, cliargs.sessioninfo, cliargs.sessionstats, cliargs.utp, cliargs.noutp);
}

}
fn isUrl(inputFromTorrent: &String) -> bool {
  let _potentialURL = match Url::parse(inputFromTorrent) {
    Ok(_potentialURL) => {if _potentialURL.scheme() == "file" {
      return false
    } else { 
      return true
    };
  }
    Err(error) => return false
  };
}

pub fn isPath(inputFromTorrent: String) -> bool {
  std::path::Path::new(&inputFromTorrent).is_file()
}

pub fn torrentHelper(torrent: &String, rtorrenturl: Url, tempdir: String, incompletedir: Option<String>, files: bool, infobool: bool, infofilebool:bool, infopieces: bool, infotracker: bool, labels: Option<Option<String>>, movepath: Option<Option<String>>, findpath: Option<Option<String>>, tracker: Option<String>, trackerid: Option<String>, stop:bool, start: bool, verify: bool, remove: bool, removeAndDelete: bool) {
  let onlyAlphanumericRtorrentURL: String = rtorrenturl.to_string().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
  match torrent.parse::<i16>() {
    Ok(ok) => {
      // now that we know the string we got is probably a valid 16-bit integer, we can see if that integer refers to a given torrent, aka a hash, storred in /tmp/
      let hashmap = tempfile::deserCompare::returnDeserializedHashMap(tempfile::previousRtorrentRemoteJSONS(tempdir.clone() ,&onlyAlphanumericRtorrentURL));
      let torInt = ok - 1;
      let value: String = hashmap.get(&torInt).unwrap().to_string();
      if incompletedir.is_some() {
        unimplemented!();
      }
      if files {
        unimplemented!();
      }
      if infofilebool {
        unimplemented!();
      }
      if infotracker {
        unimplemented!();
      }
      if infopieces {
        unimplemented!();
      }
      if movepath.is_some() {
        unimplemented!();
      }
      if findpath.is_some() {
        unimplemented!();
      }
      if tracker.is_some() {
        unimplemented!();
      }
      if trackerid.is_some() {
        unimplemented!();
      }
      if stop {
        unimplemented!();
      }
      if start {
        unimplemented!();
      }
      if verify {
        unimplemented!();
      }
      if remove  {
      xmlrpchelper::erase(&rtorrenturl,value);
    }
     if removeAndDelete {
      unimplemented!();
     }

      },
    Err(_) => println!("Unable to detect {} as an 16 bit integer", torrent),
  }

}

pub fn rtorrenthelper(rtorrenturl: &Url, incompletedir: Option<String>, incompletedirbool: bool, cache: Option<i32>, exitrtorrent: bool, sessioninfo: bool, sessionstats: bool, utp: bool, noutp: bool) {
  if exitrtorrent {
    xmlrpchelper::exitRtorrent(rtorrenturl);
  }
  if incompletedir.is_some() {
    unimplemented!();
  }
  if incompletedirbool {
    unimplemented!();
  }
  if cache.is_some() {
    unimplemented!();
  }

  
}