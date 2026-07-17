use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use std::io;

pub struct Connection {
    id: u64,
    client_addr: SocketAddr,
    stream: TcpStream,
    message_count: u64,
}

impl Connection {
    pub fn new(
        id: u64,
        client_addr: SocketAddr,
        stream: TcpStream,
    ) -> Self {
        Self {
            id,
            client_addr,
            stream,
            message_count: 0,
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        println!(
            "Обработчик {} создан для {}",
            self.id,
            self.client_addr
        );

        let mut buffer = [0_u8; 1024];

        loop {
            let bytes_read = self.stream.read(&mut buffer).await?;

            if bytes_read == 0 {
                println!(
                    "Обработчик {}: клиент {} отключился",
                    self.id,
                    self.client_addr
                );

                return Ok(());
            }

            self.message_count += 1;

            let received =
                String::from_utf8_lossy(&buffer[..bytes_read]);

            println!(
                "Обработчик {}: сообщение №{}: {:?}",
                self.id,
                self.message_count,
                received
            );

            let response = format!(
                "Обработчик {}: сообщение №{}: {}\n",
                self.id,
                self.message_count,
                received
            );

            self.stream.write_all(response.as_bytes()).await?;
        }
    }
}