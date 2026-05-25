# Core

Core - orchestration layer.

Он принимает решения о том, что должно происходить во время синхронизации.

Core не выполняет low-level IO напрямую.

## Ответственность

Core решает:

- нужен ли upload;
- существует ли файл;
- изменился ли файл;
- нужно ли обновлять metadata;
- разрешён ли overwrite;
- существует ли конфликт;
- какой результат вернуть Gateway.

## Что Core НЕ должен знать

Core не должен знать:

- sqlite3 API;
- file descriptors;
- sockets;
- TLS;
- HTTP parsing;
- REST routing;
- физическое расположение базы данных.

## Входные данные

Core получает команды.

Пример:

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
