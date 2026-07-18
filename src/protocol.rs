use thiserror::Error;

#[derive(Debug)]
pub struct Header {
    version: u8,
    command: u8,
    flags: u16,
    request_id: u32,
    body_size: u64,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Заголовок пустой или имеет неверный формат")]
    InvalidFormat,
    #[error("Длина имени файла ({actual}) превышает допустимый лимит в {max} байт")]
    FilenameTooLong {
    actual: u16,
    max: u16,
}   
}

pub async fn parce_header (bytes: &[u8;20]) -> Result<Header, ParseError> {
    // Получаем флаги
    let flags_raw = u16::from_be_bytes([bytes[6],bytes[7]]);
    let request_id_raw = u32::from_be_bytes([bytes[8],bytes[9],bytes[10],bytes[11]]);
    let body_size_raw = u64::from_be_bytes([bytes[12],bytes[13],bytes[14],bytes[15],bytes[16],bytes[17],bytes[18],bytes[19]]);
    // Валидация номера версии, magic и команды
    let [
        b'1'..=b'3', 
        b'N', b'C', b'L', b'D', 
        _cmd @ (1..=4 | 80 | 81),
        _, _, 
        ..
    ] = bytes else {
        return Err(ParseError::InvalidFormat);
    };
    // Валидация флагов
    if (flags_raw & 0xFFF8) !=0 {
        return Err(ParseError::InvalidFormat);
    }


    let local_header = Header{
        version: bytes[0],
        command: *_cmd,
        flags: flags_raw,
        request_id: request_id_raw,
        body_size: body_size_raw,
    };
    Ok(local_header)
}

// Реализация публичных методов-геттеров для безопасного доступа к приватным полям
impl Header {
    /// Возвращает версию протокола пакета
    pub fn version(&self) -> u8 {
        // Так как тип u8 реализует трейт Copy, значение просто копируется наружу
        self.version
    }

    /// Возвращает код команды (например, 1, 2, 80 или 81)
    pub fn command(&self) -> u8 {
        self.command
    }

    /// Возвращает битовые флаги пакета (уже прошедшие валидацию)
    pub fn flags(&self) -> u16 {
        self.flags
    }

    /// Возвращает уникальный идентификатор запроса клиента
    pub fn request_id(&self) -> u32 {
        self.request_id
    }

    /// Возвращает размер тела сообщения в байтах
    pub fn body_size(&self) -> u64 {
        self.body_size
    }
}