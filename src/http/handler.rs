use crate::http::{request::HttpRequest, response::HttpResponse};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
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

    let response = match req.path.as_str() {
        "/greet" => {
            let name = req.query.get("name").map_or("stranger", |v| v);
            let body = format!("<html><body><h1>Hello, {}!</h1></body></html>", name);
            HttpResponse::new(&req.version, 200, "OK")
                .with_body(&body, "text/html")
                .format()
        }
        "/" => {
            let body = "<html><body><h1>Hello, World!</h1></body></html>";
            HttpResponse::new(&req.version, 200, "OK")
                .with_body(body, "text/html")
                .format()
        }
        _ => {
            let body = "<html><body><h1>404 Not Found</h1></body></html>";
            HttpResponse::new(&req.version, 404, "Not Found")
                .with_body(body, "text/html")
                .format()
        }
    };

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
