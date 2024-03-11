use std::io::{BufRead, Write}; //Buffer Readiing and Writing
use std::net::TcpListener; //TCP listener
use std::path::PathBuf; //Path Buffer
use std::fs; //file System

fn handle_client(mut stream: std::net::TcpStream) -> Result<(), std::io::Error> {
    let mut rdr = std::io::BufReader::new(&mut stream);
    let mut request_line = String::new();
    rdr.read_line(&mut request_line)?;
    
    let mut parts = request_line.trim().split_whitespace();
    let method = parts.next().unwrap();
    let resource = parts.next().unwrap();
    
    if method != "GET" {
        stream.write_all(b"HTTP/1.1 405 Method Not Allowed\r\n\r\n")?;
        return Ok(());
    }

    let mut path = PathBuf::new();
    path.push("htdocs");
    path.push(resource.trim_start_matches('/'));

    if path.is_dir() {
        path.push("index.html");
    }

    let response = match fs::read_to_string(&path) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content),
        Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes())?;
    
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9999")?;
    
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            if let Err(err) = handle_client(stream) {
                eprintln!("Error handling client: {}", err);
            }
        });
    }

    Ok(())
}
