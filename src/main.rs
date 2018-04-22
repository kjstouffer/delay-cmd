extern crate clap;

use clap::ArgMatches;
use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::process::Command;
use std::str;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

mod load_config;

struct Times {
    first: SystemTime,
    latest: SystemTime,
    duration: Duration,
}

fn get_stdin_data() -> Vec<u8> {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).unwrap();
    buf
}

fn main() {
    let matches = load_config::app().get_matches();
    if matches.is_present("server") {
        server().unwrap();
    } else {
        client(matches);
    }
}

fn server() -> std::io::Result<()> {
    let cmd_map: Arc<Mutex<HashMap<String, Times>>> = Arc::new(Mutex::new(HashMap::new()));
    let add_cmd = cmd_map.clone();
    let check_cmd = cmd_map.clone();

    let socket = UdpSocket::bind("127.0.0.1:3400")?;
    let (tx, rx) = channel();

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    // let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            //check timer here! :tada:
            let mut locked_map = check_cmd.lock().unwrap();
            let mut keys_to_remove: Vec<String> = vec![];
            for (key, val) in locked_map.iter() {
                let since_latest = val.latest.elapsed().unwrap();
                let duration = val.first.elapsed().unwrap();
                let total = format!(
                    "{}",
                    duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
                );
                if since_latest >= val.duration {
                    let result = Command::new("bash")
                        .arg("-c")
                        .arg(key)
                        .output()
                        .expect(&format!("whoops! couldn't {}", key))
                        .stdout;
                    println!("ran command: '{}'\ntime taken to start command: {}\nwith result:\n{}\n-----------", key, total, str::from_utf8(&result).unwrap());
                    keys_to_remove.push(key.to_string());
                }
            }
            for key in keys_to_remove {
                locked_map.remove(&key);
            }
            if locked_map.is_empty() {
                drop(locked_map);
                //wait for signal from other thread for a new command
                //we don't care what the message is, only that we were signaled
                println!("command map is empty, waiting for channel.");
                rx.recv().unwrap();
            } else {
                drop(locked_map);
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    loop {
        let mut buf: Vec<u8> = vec![0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        socket.send_to(b"", &src)?;
        let mut locked_map = add_cmd.lock().unwrap();
        let cmd = str::from_utf8(&buf[..amt]).unwrap().trim();
        let duration = &cmd[..4].parse::<u64>();
        let duration = match duration {
            Ok(d) => Duration::from_millis(*d),
            _ => {
                println!("duration {} could not be converted to usize.", &cmd[..4]);
                continue;
            }
        };
        let cmd = &cmd[4..];
        let now = SystemTime::now();
        let times = Times {
            first: now,
            latest: now,
            duration: duration,
        };
        let last_time = locked_map.remove(cmd);
        let new_times = match last_time {
            Some(mut v) => {
                v.latest = now;
                v
            }
            None => times,
        };
        locked_map.insert(cmd.to_string(), new_times);
        //just send nothing
        tx.send("").unwrap();
        drop(locked_map);
    }
}

fn client(matches: ArgMatches) {
    let stdindata: Vec<u8>;
    let cmd = if matches.is_present("cmd") {
        matches.value_of("cmd").unwrap()
    } else {
        stdindata = get_stdin_data();
        str::from_utf8(&stdindata).expect("No command specified.")
    };
    let delay = matches
        .value_of("delay")
        .unwrap()
        .parse::<u64>()
        .expect("couldn't parse delay into u64");
    let remote_addr: SocketAddr = "127.0.0.1:3400".parse().unwrap();
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
