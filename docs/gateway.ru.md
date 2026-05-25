# Gateway

Gateway - внешняя точка входа.

Он отвечает за transport, TLS, pairing и authentication.

Gateway преобразует внешние запросы в команды для Core.

## Ответственность

Gateway отвечает за:

- TCP listening;
- TLS session handling;
- HTTP parsing;
- REST routing;
- pairing ticket validation;
- device authentication;
- request size limits;
- basic input validation.

## Что Gateway НЕ должен делать

Gateway не должен:

- напрямую писать файлы;
- напрямую вызывать sqlite3;
- принимать sync decisions;
- знать filesystem layout;
- напрямую обновлять metadata.

## TLS Model

nocloud не требует публичного CA или домена для домашнего использования.

Сервер может использовать self-signed certificate.

Trust устанавливается через QR pairing.

QR должен содержать:

```text
server address
server public key fingerprint
one-time pairing ticket
expiration time
```
