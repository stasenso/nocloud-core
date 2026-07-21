// Парсим заголовок на базе Nom
use nom::{
    bytes::complete::tag,
    combinator::verify,
    number::complete::{
        be_u8,
        be_u16,
        be_u32,
        be_u64,
    },
    IResult,
    Parser,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeaderValidationError {
    #[error("Неподдерживаемая версия протокола: {0}")]
    UnsupportedVersion(u8),

    #[error("Неизвестная команда протокола: {0}")]
    UnsupportedCommand(u8),
}

#[derive(Debug)]
pub struct Header {
    version: u8,
    command: u8,
    flags: u16,
    request_id: u32,
    body_size: u64,
}

pub fn parse_header(
    input: &[u8],
    ) -> IResult<&[u8],Header> {
    //1. Парсим магические байты "NCLD"
    let (input, _) = tag(&b"NCLD"[..]).parse(input)?;
    //2. Парсим и валидируем версию
    let (input, version) = be_u8(input)?;
    //3. Парсим и валидируем Header 
    let (input, command) = be_u8(input)?;
    let (input, flags) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, body_size) = be_u64(input)?;

    let local_header = Header{
        version,
        command,
        flags,
        request_id,
        body_size,
    };
    Ok((input,local_header))
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
    // Валидация Header
    pub fn validate(&self) -> Result<(), HeaderValidationError> {
    if self.version != 1 {
        return Err(
            HeaderValidationError::UnsupportedVersion(self.version)
        );
    }

    if self.command != 1 {
        return Err(
            HeaderValidationError::UnsupportedCommand(self.command)
        );
    }

    Ok(())
}
}