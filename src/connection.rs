use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use std::io;

use crate::protocol::parse_header; 
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
        self.id, self.client_addr
    );

    // Буфер для данных тела сообщения (теперь он не хранит заголовок)
    let mut body_buffer = vec![0_u8; 1024];

    loop {
        // 1. Создаем буфер СТРОГО под размер заголовка
        let mut header_buffer = [0_u8; 20];

        // 2. Читаем из сети РОВНО 20 байт
        // Если клиент закроет соединение до того, как пришлет 20 байт,
        // read_exact вернет ошибку UnexpectedEof.
        if let Err(e) = self.stream.read_exact(&mut header_buffer).await {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                println!(
                    "Обработчик {}: клиент {} отключился (или прислал неполный заголовок)",
                    self.id, self.client_addr
                );
                return Ok(());
            }
            return Err(e); // Другие системные ошибки ввода-вывода (например, обрыв связи)
        }

        // 3. Вызываем ваш парсер заголовка
        let header = match parse_header(&header_buffer) {
            Ok((_remaining, header)) => {
                println!(
                    "Обработчик {}: nom оставил {} байт",
                    self.id,
                    _remaining.len()
                );
                header
            }
               
            Err(error) => {
                println!(
                    "Обработчик {}: ошибка парсинга заголовка: {:?}",
                    self.id, error
                );

        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid header: {error:?}"),
        ));
    }
};

// 
if let Err(error) = header.validate() {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        error,
    ));
}
        self.message_count += 1;
        println!("Обработчик {}: Успешно распарсен заголовок №{}: {:?}", self.id, self.message_count, header);

        // 4. Используем данные из распарсенного заголовка!
        // Теперь мы знаем точный размер тела сообщения благодаря header.body_size
        const MAX_BODY_SIZE: u64 = 1024 * 1024; // 1 МиБ

        if header.body_size() > MAX_BODY_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Размер тела {} превышает лимит {}",
                    header.body_size(),
                    MAX_BODY_SIZE,
                ),
            ));
        }

        let body_len = header.body_size() as usize;
        
        // Подгоняем размер буфера под размер тела (чтобы не читать лишнего)
        if body_len > body_buffer.len() {
            body_buffer.resize(body_len, 0);
        }

        // Читаем из сети тело сообщения (ровно столько байт, сколько указано в заголовке)
        self.stream.read_exact(&mut body_buffer[..body_len]).await?;

        let received_body = String::from_utf8_lossy(&body_buffer[..body_len]);
        println!("Обработчик {}: Получено тело сообщения: {:?}", self.id, received_body);

        // 5. Отправляем ответ клиенту
        let response = format!(
            "Обработчик {}: Заголовок валиден (cmd={}). Тело принято.\n",
            self.id, header.command()
        );
        self.stream.write_all(response.as_bytes()).await?;
    }
}
}