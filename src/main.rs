use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    thread,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};

mod pages {
    pub mod home;
}
mod components {
    pub mod header;
}

fn main() {
    const PORT: &str = ":443";

    println!("Starting HTTPS server on 0.0.0.0{}", PORT);
    server(&PORT);
}

fn server(port: &str) {
    let port = format!("0.0.0.0{}", port);

    let mut acceptor = match SslAcceptor::mozilla_intermediate(SslMethod::tls()) {
        Ok(acceptor) => acceptor,
        Err(e) => {
            eprintln!("Error creating SSL acceptor: {}", e);
            return;
        }
    };

    if let Err(e) = acceptor.set_private_key_file("key.pem", SslFiletype::PEM) {
        eprintln!("Error loading private key: {}", e);
        return;
    }

    if let Err(e) = acceptor.set_certificate_chain_file("cert.pem") {
        eprintln!("Error loading certificate chain: {}", e);
        return;
    }

    let acceptor = acceptor.build();

    let listener = match TcpListener::bind(&port) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Error binding to port {}: {}", port, e);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone(); 
                thread::spawn(move || {
                    match acceptor.accept(stream) {
                        Ok(ssl_stream) => handle_connection(ssl_stream),
                        Err(e) => eprintln!("SSL handshake failed: {}", e),
                    }
                });
            }
            Err(e) => eprintln!("Failed to establish connection: {}", e),
        }
    }
}

fn handle_connection(mut stream: SslStream<std::net::TcpStream>) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Error reading from stream: {}", e);
        return;
    }

    let url = String::from_utf8_lossy(&buffer);

    let status = "HTTP/1.1 200 OK";

    // Handle Home page
    if url.contains("GET / HTTP/1.1") {
        let contents = pages::home::home();
        let length = contents.len();
        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error writing to stream: {}", e);
        }
        return;
    }

    // Handle Image and Font requests
    if url.contains("GET /src/assets/rust_logo.png HTTP/1.1") {
        if let Err(e) = serve_asset("src/assets/rust_logo.png", "image/png", &mut stream) {
            eprintln!("Error serving rust_logo.png: {}", e);
        }
    } else if url.contains("GET /src/assets/rust_crab.png HTTP/1.1") {
        if let Err(e) = serve_asset("src/assets/rust_crab.png", "image/png", &mut stream) {
            eprintln!("Error serving rust_crab.png: {}", e);
        }
    } else if url.contains("GET /src/assets/public-sans.ttf HTTP/1.1") {
        if let Err(e) = serve_asset("src/assets/public-sans.ttf", "font/ttf", &mut stream) {
            eprintln!("Error serving public-sans.ttf: {}", e);
        }
    } else if url.contains("GET /src/assets/menu_icon.png HTTP/1.1") {
        if let Err(e) = serve_asset("src/assets/menu_icon.png", "image/png", &mut stream) {
            eprintln!("Error serving menu_icon.png: {}", e);
        }
    }
}

fn serve_asset(path: &str, content_type: &str, stream: &mut SslStream<std::net::TcpStream>) -> Result<(), std::io::Error> {
    match fs::read(path) {
        Ok(contents) => {
            let status = "HTTP/1.1 200 OK";
            let response = format!(
                "{status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n",
                contents.len()
            );
            if let Err(e) = stream.write_all(response.as_bytes()) {
                return Err(e);
            }
            if let Err(e) = stream.write_all(&contents) {
                return Err(e);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading asset {}: {}", path, e);
            let error_response = "HTTP/1.1 404 Not Found\r\n\r\n";
            let _ = stream.write_all(error_response.as_bytes());
            Err(e)
        }
    }
}

