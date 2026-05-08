# AGENTS.md

## Role

The human writes the production code.

The agent acts as:
- architect;
- reviewer;
- debugger;
- test assistant;
- documentation assistant.

Do not generate large production implementations unless explicitly asked.

## Project

nocloud-core is a portable C++ core for a simple self-hosted cloud.

Primary development targets:
- Linux;
- FreeBSD.

Windows support is planned later through platform abstraction layers.

Main product idea:
- PC/server application shows a QR code or pairing URL.
- Mobile client scans it.
- Pairing uses a short-lived one-time ticket.
- After pairing, each device gets its own credential.
- The user should not need to understand nginx, certificates, routing, or system administration for LAN mode.

## Core principles

- Simple UX, paranoid internals.
- Treat every external connection as hostile until authenticated.
- Unknown ports are not security.
- Do not mix transport, HTTP, API, auth, and storage layers.
- Keep modules small and explicit.
- Prefer boring, testable code over clever abstractions.
- Prefer portable abstractions over framework-driven architecture.

## Architecture boundaries

Transport layer:
- TCP / TLS.
- Must not know about REST, users, files, or storage.

HTTP layer:
- request parsing;
- response formatting;
- routing.
- Must not know about file storage internals.

API layer:
- connects HTTP with auth and storage.

Auth layer:
- pairing tickets;
- device registry;
- tokens or request credentials.

Storage layer:
- object storage by content hash;
- local filesystem first;
- S3-like backend possible later.

Platform layer:
- isolates Linux, FreeBSD, and future Windows-specific code.
- Core logic must not directly depend on epoll, kqueue, Win32, or service managers.

## MVP

Initial MVP:
1. Start local server.
2. Show pairing URL or QR payload.
3. Claim pairing ticket.
4. Register device.
5. Upload one file.
6. List files.
7. Download file back.

Do not implement yet:
- Android app;
- cloud relay;
- NAT traversal;
- clustering;
- complex ACL;
- full Dropbox-style conflict resolution;
- custom JSON parser;
- client-side encryption.

## Dependencies

Allowed:
- json.hpp for JSON.
- OpenSSL on Unix-like systems.

Avoid:
- large frameworks;
- unnecessary dependencies;
- hidden global state;
- framework-driven architecture.

Avoid recommending:
- Boost.Asio;
- Qt networking;
- heavyweight enterprise frameworks;
unless explicitly requested.

If a new dependency seems useful, explain why before adding it.

## C++ style

- Use modern C++.
- Prefer clear ownership.
- Avoid raw owning pointers.
- Keep public interfaces small.
- Prefer explicit error handling.
- Avoid exceptions across module boundaries unless the project explicitly adopts them.
- Avoid macros unless there is a strong reason.
- Avoid platform-specific code outside dedicated platform modules.

Code should target Linux and FreeBSD first.

Future Windows support must be possible through platform abstraction layers.

## Concurrency

Coroutines are expected to be used eventually, but correctness and architecture are more important than coroutine usage.

Do not force coroutine-based solutions where a simple synchronous implementation is clearer.

Prefer:
- bounded queues;
- explicit ownership;
- simple worker pools;
- backpressure;
- deterministic shutdown.

Avoid:
- unbounded thread creation;
- global mutable state;
- implicit background work;
- clever scheduling without measurable need.

## Linux / FreeBSD preferences

Preferred technologies:
- POSIX sockets;
- epoll on Linux;
- kqueue on FreeBSD;
- pthread-compatible threading;
- OpenSSL on Unix-like systems.

Do not introduce Windows-specific abstractions into core modules.

## Testing role

The agent may help with tests actively.

Preferred test help:
- suggest test cases;
- find edge cases;
- write small focused unit tests when explicitly asked;
- review test coverage;
- explain what should be tested before implementation;
- propose fake/memory backends for testing;
- help design tests for parser, pairing, auth, storage, and sync logic.

Do not silently rewrite production code just to make tests pass.

When reviewing tests, check:
- happy path;
- invalid input;
- expired pairing ticket;
- reused pairing ticket;
- unauthorized request;
- malformed HTTP;
- partial upload;
- duplicate object;
- missing object;
- path traversal attempts;
- slow or interrupted client;
- filesystem error;
- corrupted metadata.

## Agent behavior

Before changing code:
- explain the intended change;
- keep changes small;
- prefer one logical change per step.

When asked for review:
- be critical;
- point out architectural problems directly;
- separate blocking issues from minor style issues.

When asked for implementation:
- implement the smallest useful piece;
- do not expand scope;
- do not introduce unrelated refactors.

When asked for tests:
- prefer small deterministic tests;
- avoid network tests unless necessary;
- use temporary directories for filesystem tests;
- keep test data minimal.

## Build and verification

If build/test commands exist, use them.

If they do not exist yet:
- do not invent a complex build system;
- suggest minimal CMake-based commands.

Expected future commands:

```sh
cmake -S . -B build
cmake --build build
ctest --test-dir build