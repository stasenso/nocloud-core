# Metadata

Metadata stores persistent sync state.

The current target implementation is SQLite.

Metadata does not make sync decisions.

## Responsibilities

Metadata stores:

- devices;
- file records;
- relative paths;
- local paths;
- size;
- mtime;
- SHA-256;
- timestamps;
- pairing/device state.

## What Metadata Must Not Do

Metadata must not:

- write files;
- delete files;
- parse HTTP;
- authenticate requests;
- make overwrite decisions;
- resolve conflicts.

## Schema Draft

```sql
CREATE TABLE devices (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    public_key TEXT,
    created_at INTEGER NOT NULL,
    last_seen_at INTEGER
);

CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id TEXT NOT NULL,
    relative_path TEXT NOT NULL,
    local_path TEXT NOT NULL,
    size INTEGER NOT NULL,
    mtime INTEGER NOT NULL,
    sha256 TEXT NOT NULL,
    updated_at INTEGER NOT NULL,

    UNIQUE(device_id, relative_path),

    FOREIGN KEY(device_id) REFERENCES devices(id)
);

CREATE INDEX idx_files_device_path
ON files(device_id, relative_path);

CREATE INDEX idx_files_sha256
ON files(sha256);
```
