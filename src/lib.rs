use std::io::{Read, Write};

mod thread;

use std::net::{TcpListener, TcpStream};

use crate::thread::ThreadPool;

pub fn start(address: String, port: u16, workers: usize) {
    let pool = ThreadPool::new(workers);
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    for stream in listener.unwrap().incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    panic!("Implement me!");
}
