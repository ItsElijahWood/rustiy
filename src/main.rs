use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

mod pages {
    pub mod home;
}
mod components {
    pub mod header;
}

fn main() {
    const PORT: &str = ":3000";

    println!("Starting web server on 127.0.0.1{}", PORT);
    server(&PORT);
}

fn server(port: &str) {
    let port = format!("127.0.0.1{}", port);
    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        connection(&mut stream);
    }
}

fn connection(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Error reading stream into buffer in main.rs: {}", e);
    };

    let url = String::from_utf8_lossy(&buffer);

    let status = "HTTP/1.1 200 OK";
    // Home page
    if url.contains("GET / HTTP/1.1") {
        let contents = pages::home::home();
        let length = contents.len();
        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

        return stream.write_all(response.as_bytes()).unwrap();
    }
  
    // Images and fonts
    if url.contains("GET /src/assets/rust_logo.png HTTP/1.1") {
        let contents = fs::read("src/assets/rust_logo.png").unwrap();
        let response = format!(
            "{status}\r\nContent-Type: image/png\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }
    if url.contains("GET /src/assets/rust_crab.png HTTP/1.1") {
        let contents = fs::read("src/assets/rust_crab.png").unwrap();
        let response = format!(
            "{status}\r\nContent-Type: image/png\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }
    if url.contains("GET /src/assets/public-sans.ttf HTTP/1.1") {
        let contents = fs::read("src/assets/public-sans.ttf").unwrap();
        let response = format!(
            "{status}\r\nContent-Type: image/png\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    }
}
