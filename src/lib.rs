mod thread;

use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;


use crate::thread::ThreadPool;

pub fn start(address: String, port: u16, workers: usize) {
    let pool = ThreadPool::new(workers);
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    for stream in listener.unwrap().incoming() {
        let stream = stream.unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(5))).expect(
            "Failed to change socket timeout duration");
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    let res = stream.read_to_string(&mut buffer);
    match res {
        Err(e) => {
            match e.kind() {
                ErrorKind::WouldBlock => {
                    eprintln!("Socket timeout expired");
                },
                ErrorKind::InvalidData => {
                    eprintln!("Received invalid UTF data");
                },
                _ => {
                    println!("unknown error kind `{:?}`", e.kind());
                }
            }
        },
        Ok(_) => {
            println!("{}", buffer);
        }
    }
}
