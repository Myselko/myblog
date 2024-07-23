use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader, Read, Write};
use std::path::Path;

//prelude - is a list of things imported to all projects, kept as small as possible. https://doc.rust-lang.org/std/prelude/index.html
//std::io::prelude - contains Read, Write, Seek, BufReader

fn connection(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("serving at 127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream){
    let mut buff = [0;512];
    stream.read(&mut buff).unwrap();
    let request = String::from_utf8_lossy(&buff[..]);
    let mut lines = request.lines();
    if let Some(request_line) = lines.next() {
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
    
        let filename = match (method, path) {
            ("GET","/") => "src/index.html",
            ("GET","/about") => "src/about.html",
            ("GET","/contact") => "src/contact.html",
            _ => "src/404.html",
        };
        println!("{}", filename);
        let path = Path::new(filename);
        if path.exists() {
            let contents = fs::read_to_string(filename).unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            println!("file not found");
            let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            let contents = "404 - Not Found";
            let response = format!("{}{}", status_line, contents);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
fn main() {
    connection();
}
