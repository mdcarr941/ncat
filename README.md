## Overview
tcat (TCP cat) is a very basic [netcat](https://linux.die.net/man/1/nc) clone. It is a program
which can be used to send data over TCP sockets.

## Building
tcat is written in Rust so the first step is to [install Rust](https://www.rust-lang.org/tools/install).
Then, you can simply run `cargo build --release` in the root of the repository to produce the
release build. The compiled binary is then available at `target/release/tcat`.

## Usage
Since tcat is basic, it has a basic command line interface:

```
tcat [-l] <hostname or ip address>[:<port number>]
```

If the `-l` option is supplied then tcat will start in server mode and bind to the host and port
provided. Once a client connection is established, the server will output everything it receives
from the client to stdout.

If the `-l` option is not supplied then tcat will start in client mode and attempt to connect
to the host at the port specified. It will then read from stdin until EOF, copying all the data
it reads to the server.

If no port is specified then the default port, 41270, is used.

### Contrived Example
Suppose you wanted to transfer a file called `rick.and.morty.s05e05.1080p.webrip.x264-cakes[eztv.re].mkv`
from you server to your home theater PC (HTPC). You could use scp, nfs, cifs, Dropbox, Google Drive,
OneDrive, a USB drive, OR tcat. Assuming the hostname of the HTPC on your LAN is `entertaintment.lan`
you'd start tcat on that machine in server mode by running `tcat -l entertaintment.lan > s05e05.mkv`, this
will cause it to listen on whatever IP address it was assigned. Then, from your server you'd run:
```
tcat entertaintment.lan < rick.and.morty.s05e05.1080p.webrip.x264-cakes[eztv.re].mkv
```
This will transfer the file without all the overhead of those other tools!