use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:32768").await?;

    println!("Сервер слушает 127.0.0.1:32768");

    loop {
        let (stream, client_addr) = listener.accept().await?;

        println!("Подключился клиент: {}", client_addr);
        
        tokio::spawn(async move {
            if let Err(error) = handle_client(stream).await {
                eprintln!("Ошибка клиента {client_addr}: {error}");
            }
            println!("Клиент {client_addr} отключился");
        });
    }
}

async fn handle_client(mut stream:TcpStream) -> io::Result<()> {
    let mut buffer = [0_u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            return  Ok(());
        }

        println!("получено {bytes_read} байт");

        stream.write_all(&buffer[..bytes_read]).await?;
    }

}