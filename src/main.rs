use std::{env, io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};

fn handle_client (mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "uncnown".to_string(), |addr| addr.to_string());
    println!("Handling connection from: {}", peer_addr);

    let mut buffer = [0;1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n ==0 {
                    println!("client {} closed connection",peer_addr);
                    break;
                }
                
                if let Err (e) = stream.write_all(&buffer) {
                    eprintln!("Write error to client {}: {}", peer_addr, e
                );
                break;
                }
            }

            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::ConnectionReset => {
                        println!("Клиент {} Сбросил соединение", peer_addr);
                    }
                    _ => {
                        eprintln!("Read Error frop client {}: {}",
                        peer_addr,
                        e
                    );
                    }
                    
                }
            }
        }
    }
}

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(||"127.0.0.1:32768".to_string());

    let listener = TcpListener::bind(&addr)
        .expect("Failed to bind to address");
    println!("Server listening om {}", addr);

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection {}",e);
            }
        }
    }

}
