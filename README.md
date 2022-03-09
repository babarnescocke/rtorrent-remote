# rtorrent-remote: A tool for administering rtorrent from the command line

[rtorrent](https://github.com/rakshasa/rtorrent) is a really nice torrenting backend, with two built-in modes, daemon or an ncurses-based UI. Its great, but its not always easy to easily grab information, or a torrent.
This program aims to be [transmision-remote](https://github.com/transmission/transmission/blob/master/utils/remote.cc) for rtorrent.

There are some situations where rtorrent's current setup is less than desirable. E.g. I have a huge directory of files and directories - but not all of them are currently handled by rtorrent, what files aren't or are? In transmission-remote, this can be accomplished many ways, so that functionality is coming over.

The program rtorrent advises you use is xmlrpc-c, which is nice enough, but on some distros, notably alpine, it isn't provided by the distro. Additionally, one-off commands in the xmlrpc client are relatively straightforward, but executing multicalls from the command line is mostly unusable. And parsing the xml from the command line isn't always straightforward.

# A problem

The biggest issue we have to deal with is how to translate how rtorrent wants to communicate about torrents, Hashes, and how transmission-remote does, a 1 indexed list. Transmission-remote, starting with no torrents, will add a torrent at index 1, then 2 - so on, removing/deleting the torrent at index 1 will not effect the index of the other torrents until transmission is reloaded. This is super nice, index 7 is say a CentOS ISO, if I am done seeding it- I can simply delete it - `transmission-remote -rad -t7`. rtorrent would like the hash of the torrent you want to manipulate - this is obviously a pain from a UI and usability perspective. Testing I did indicated rtorrent does typically return the same order of torrents during multicalls but obviously this doesn't maintain a given index over time. A large amount of the program is just insuring index stays consistent.

# Rust

This is also about me learning Rust. I wanted a compilable language so that I can just toss binaries wherever I want them, and worry less about external dependencies.

# Dependencies

 * libtorrent > 0.13.8
 * rtorrent > 0.9.8
 * linux > 3.14trans