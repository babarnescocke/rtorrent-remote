# rtorrent-remote: A tool for administering rtorrent from the command line

[rtorrent](https://github.com/rakshasa/rtorrent) is a really nice torrenting backend, with two built-in modes, daemon or an ncurses-based UI. Its great, but its not always easy to easily grab information, or a torrent.
This program aims to be [transmision-remote](https://github.com/transmission/transmission/blob/master/utils/remote.cc) for rtorrent.

There are some situations where rtorrent's current setup is less than desirable. E.g. I have a huge directory of files and directories - but not all of them are currently handled by rtorrent, what files aren't or are? In transmission-remote, this can be accomplished many ways; or maybe, I am using rtorrent in a container, or on a remote system, and when I interact with it certain assumptions about file paths or whether the system running rtorrent-remote is the computer running rtorrent , so that functionality is coming over.

The program rtorrent advises you use is xmlrpc-c, which is nice enough, but on some distros, notably alpine, it isn't provided by the distro. Additionally, one-off commands in the xmlrpc client are relatively straightforward, but executing multicalls from the command line is mostly unusable. And parsing the xml from the command line isn't always straightforward, you might send a big complex call - and get only a bool back - but that doesn't help you rework what actually happened. So hopefully that will be clearer.

# A problem

The biggest issue we have to deal with is how to translate how rtorrent wants to communicate about torrents, Hashes, and how transmission-remote does, a 1 indexed list. Transmission-remote, starting with no torrents, will add a torrent at index 1, then 2 - so on, removing/deleting the torrent at index 1 will not effect the index of the other torrents until transmission is reloaded. This is super nice, index 7 is say a CentOS ISO, if I am done seeding it- I can simply delete it - `transmission-remote -rad -t7`. rtorrent would like the hash of the torrent you want to manipulate - this is obviously a pain from a UI and usability perspective. Testing I did indicated rtorrent does typically return the same order of torrents during a given session but obviously this doesn't maintain a given index over different sessions. A large amount of the program is just insuring index stays consistent.

There is an rtorrent API field session.time that may be able to effectively eliminate this issue. But as yet I haven't done any testing on how much longer it takes to do a full separate API call to accomplish it. Maybe later.

# Rust

This is also about me learning Rust. I wanted a compilable language so that I can just toss binaries wherever I want them, and worry less about external dependencies.

# Some Rtorrent Security Notes

So a lot of this program is going to be a bit round-hole-square-peg, but one thing to note is the XML-RPC interface for rtorrent, while it is at times a bit stilted as an API, is VERY POWERFUL. And, by design, allows people connected to it to run arbitrary shell commands as the user rtorrent is using. OK, let's take a moment and repeat that:

*If someone can send commands to your Rtorrent's XML-RPC2 interface they can run whatever they would like as your rtorrent user.*

This, according to their docs, is by design, so don't expect it to change. I will use it later to delete removed torrent files.


# Dependencies

 * libtorrent > 0.13.8
 * rtorrent > 0.9.8
 * linux > 3.14trans