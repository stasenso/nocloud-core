mod connection;
mod protocol;

use tokio::net::{TcpListener};
use tokio::io;
use std::sync::atomic::{AtomicU64, Ordering};

use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr};

use connection::Connection;

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() -> io::Result<()> {
    let my_ip: IpAddr = local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let listener = TcpListener::bind((my_ip,32768)).await?;

    println!("Сервер слушает {}:32768",my_ip);

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

