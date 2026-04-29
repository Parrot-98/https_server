use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // start server 

    //start accepting connections 
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); // turn the connection into readable text 
    let http_request: Vec<_> = buf_reader
        .lines() // reads one line at a time
        .map(|result| result.unwrap()) // extract strings
        .take_while(|line| !line.is_empty())// read until a white space as heared in html eand with it
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); // print this on to the server form the html 

    stream.write_all(response.as_bytes()).unwrap();
}