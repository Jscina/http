extern crate http;
use crate::http::http::request::HttpRequest;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer);
    let mut lines = request.lines();

    let request_line = lines.next().unwrap_or("");
    let mut req = HttpRequest::parse(request_line);

    lines
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| line.split_once(':'))
        .for_each(|(k, v)| req.set_header(k.trim().into(), v.trim().into()));

    println!("Request: {:#?}", req);

    let body = "<html><body><h1>Hello, World!</h1></body></html>";
    let response =
        format!(
        "{} 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n{}",
        req.version, body.len(), body
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
