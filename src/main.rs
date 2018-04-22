//! A UDP client that just sends everything it gets via `stdio` in a single datagram, and then
//! waits for a reply.
//!
//! For the reasons of simplicity data from `stdio` is read until `EOF` in a blocking manner.
//!
//! You can test this out by running an echo server:
//!
//! ```
//!     $ cargo run --example echo-udp -- 127.0.0.1:8080
//! ```
//!
//! and running the client in another terminal:
//!
//! ```
//!     $ cargo run --example udp-client
//! ```
//!
//! You can optionally provide any custom endpoint address for the client:
//!
//! ```
//!     $ cargo run --example udp-client -- 127.0.0.1:8080
//! ```
//!
//! Don't forget to pass `EOF` to the standard input of the client!
//!
//! Please mind that since the UDP protocol doesn't have any capabilities to detect a broken
//! connection the server needs to be run first, otherwise the client will block forever.
extern crate clap;

use std::io::stdin;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::io::Read;
use std::str;

mod load_config;

fn get_stdin_data() -> Vec<u8> {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    buf
}

fn main() {
    let stdindata:Vec<u8>;
    let matches = load_config::app().get_matches();
    let cmd = if matches.is_present("cmd") {
        matches.value_of("cmd").unwrap()
    } else {
        stdindata = get_stdin_data();
        str::from_utf8(&stdindata).expect("No command specified.")
    };
    let delay = matches.value_of("delay").unwrap().parse::<u64>().expect("couldn't parse delay into u64");
    let remote_addr: SocketAddr = "127.0.0.1:3400"
        .parse()
        .unwrap();
    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let socket = UdpSocket::bind(&local_addr).unwrap();
    socket.connect(&remote_addr).unwrap();
    const MAX_DATAGRAM_SIZE: usize = 1_024;
    //combine delay and cmd
    let to_send = format!("{:04} {}", delay, cmd);
    socket.send(to_send.as_bytes()).unwrap();
    socket.recv(&mut vec![0u8; MAX_DATAGRAM_SIZE]).unwrap();
}
