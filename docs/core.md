# Core

Core is the orchestration layer.

It decides what should happen during synchronization.

Core does not perform low-level IO directly.

## Responsibilities

Core decides:

- whether an upload is needed;
- whether a file exists;
- whether a file has changed;
- whether metadata must be updated;
- whether overwrite is allowed;
- whether a conflict exists;
- what result to return to Gateway.

## What Core Must Not Know

Core must not know about:

- sqlite3 API;
- file descriptors;
- sockets;
- TLS;
- HTTP parsing;
- REST routing;
- the physical location of the database.

## Input

Core receives commands.

Example:

```cpp
struct UploadFileCommand {
    DeviceId device_id;
    RelativePath path;
    uint64_t size;
    uint64_t mtime;
    Sha256 sha256;
    InputStream* stream;
};
```
