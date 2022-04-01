# Changelog
All notable changes to this project will be documented in this file.

##[Todo]
The following flags are not operational: --labels, --bandwidth-high, --bandwidth-low, --bandwidth-normal, --get, --no-get, --priority-high, --priority-normal, --move-path, --find-path, --tracker, --tracker-remove - will error out todo!();

 - A feature I am going to add is to pass something like --ff and get back all files rtorrent currently has. - this will allow easy piping to parse what files rtorrent currently isn't using.


 ## [0.4.3] - 2022.04.01
 ### Added
  - parsing of --torrent and --bandwidth-* --get, --no-get, --priority-high, --priority-normal
  - Added testing of source code.
  
 ## [0.4.2] - 2022.03.29
 ### Added
  - zstd compression to tmp file.
  - Changelog!