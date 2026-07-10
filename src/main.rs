use std::io;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("192.168.1.152:32768").await?;

    println!("Сервер слушает 192.168.1.152:32768");

    loop {
        let (stream, client_addr) = listener.accept().await?;

        let connection_id =
            NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);

        let connection = Connection::new(
            connection_id,
            client_addr,
            stream,
        );

        tokio::spawn(async move {
            if let Err(error) = connection.run().await {
                eprintln!(
                    "Обработчик {connection_id}: ошибка: {error}"
                );
            }
        });
    }
}

struct Connection {
    id: u64,
    client_addr: SocketAddr,
    stream: TcpStream,
    message_count: u64,
}

impl Connection {
    fn new(
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

    async fn run(mut self) -> io::Result<()> {
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