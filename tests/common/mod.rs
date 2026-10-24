// Every test binary compiles this module and reads only part of it.
#![allow(dead_code)]

use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

/// A request the fake API received.
pub struct Request {
    pub line: String,
    pub headers: Vec<String>,
    pub body: Value,
}

/// Starts a fake API that accepts every request with 202.
/// It hands each request over before it answers, so the SDK never sees an answer to a request the test cannot see yet.
/// Returns its URL and the requests it receives.
pub fn api() -> (String, mpsc::Receiver<Request>) {
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", server.local_addr().unwrap());
    let (requests, received) = mpsc::channel();
    thread::spawn(move || {
        for stream in server.incoming() {
            let stream = stream.unwrap();
            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let mut headers = Vec::new();
            loop {
                let mut header = String::new();
                reader.read_line(&mut header).unwrap();
                match header.trim_end() {
                    "" => break,
                    header => headers.push(header.to_ascii_lowercase()),
                }
            }
            let length = headers
                .iter()
                .find_map(|header| header.strip_prefix("content-length:"))
                .map_or(0, |length| length.trim().parse().unwrap());
            let mut body = vec![0; length];
            reader.read_exact(&mut body).unwrap();
            let body = serde_json::from_slice(&body).unwrap();
            requests
                .send(Request {
                    line,
                    headers,
                    body,
                })
                .unwrap();
            (&stream)
                .write_all(
                    b"HTTP/1.1 202 Accepted\r\ncontent-length: 0\r\nconnection: close\r\n\r\n",
                )
                .unwrap();
        }
    });
    (url, received)
}
