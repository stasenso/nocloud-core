# Gateway

Gateway is the external entry point.

It is responsible for transport, TLS, pairing, and authentication.

Gateway converts external requests into commands for Core.

## Responsibilities

Gateway is responsible for:

- TCP listening;
- TLS session handling;
- HTTP parsing;
- REST routing;
- pairing ticket validation;
- device authentication;
- request size limits;
- basic input validation.

## What Gateway Must Not Do

Gateway must not:

- write files directly;
- call sqlite3 directly;
- make sync decisions;
- know the filesystem layout;
- update metadata directly.

## TLS Model

nocloud does not require a public CA or domain for home use.

The server can use a self-signed certificate.

Trust is established through QR pairing.

The QR payload must contain:

```text
server address
server public key fingerprint
one-time pairing ticket
expiration time
```
