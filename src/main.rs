use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let host = std::env::var("HOST").expect("HOST must be set.");
    let port = std::env::var("PORT").expect("PORT must be set.");

    let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();

    println!("Server up and running on {host}:{port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_chunks: Vec<_> = request_line.trim()
    .split_whitespace().collect();
    let method = request_chunks[0];
    let pathname = request_chunks[1];

    println!("{method} for path {pathname}");

    let (status_line, filename) = match &pathname[..] {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        "/sleep" => ("HTTP/1.1 200 OK", "sleep.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}