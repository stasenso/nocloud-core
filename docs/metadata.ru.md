# Metadata

Metadata хранит persistent sync state.

Текущая целевая реализация - SQLite.

Metadata не принимает sync decisions.

## Ответственность

Metadata хранит:

- devices;
- file records;
- relative paths;
- local paths;
- size;
- mtime;
- SHA-256;
- timestamps;
- pairing/device state.

## Что Metadata НЕ должна делать

Metadata не должна:

- писать файлы;
- удалять файлы;
- парсить HTTP;
- аутентифицировать запросы;
- принимать overwrite decisions;
- разрешать конфликты.

## Черновик схемы

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
