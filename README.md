# rtorrent-remote: A tool for administering rtorrent from the command line

[rtorrent](https://github.com/rakshasa/rtorrent) is a really nice torrenting backend, with two built-in modes, daemon or an ncurses-based UI. Its great, but its not always easy to easily grab information, or a torrent.
This program aims to be [transmision-remote](https://github.com/transmission/transmission/blob/master/utils/remote.cc) for rtorrent.

There are some situations where rtorrent's current setup is less than desirable. E.g. I have a huge directory of files and directories - but not all of them are currently handled by rtorrent, what files aren't or are? In transmsission-remote, this can be accomplished many ways, so that functionality is coming over.

The program rtorrent advises you use is xmlrpc-c, which is nice enough, but on some distros, notably alpine, it isn't provided by the distro. Additionally, one-off commands in the xmlrpc client are relatively straightforward, but executing multicalls from the command line is mostly unusable.

# Rust

This is also about me learning Rust. I wanted a compilable language so that I can just toss binaries wherever I want them, and worry less about external dependencies.

# Dependencies

 * libtorrent > 0.13.8
 * rtorrent > 0.9.8
 * linux > 3.14