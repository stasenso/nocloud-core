# Storage

Storage - низкоуровневый слой управляемой папки.

Он отвечает за размещение файлов на диске.

Storage специально сделан простым.

## Ответственность

Storage умеет:

- записывать upload stream во временный файл;
- создавать директории;
- выполнять atomic replace;
- открывать файлы;
- удалять файлы;
- проверять существование файла;
- возвращать filesystem errors.

## Что Storage НЕ должен знать

Storage не должен знать:

- device authentication;
- pairing;
- conflict policy;
- HTTP;
- REST;
- SQLite schema;
- sync decisions.

## Структура директорий

Пример:

```text
data/
  devices/
    phone_1/
      DCIM/
        IMG_001.jpg

  .nocloud/
    nocloud.db
    tmp/
```
