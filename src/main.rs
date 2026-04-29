use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
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
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // reading the request

    let (status_line, filename) = if request_line == "GET /sleep HTTP/1.1"
     {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")}
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"); // writeing the file in the browser 

    stream.write_all(response.as_bytes()).unwrap();
}