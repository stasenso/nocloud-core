# Storage

Storage is a low-level managed-folder layer.

It is responsible for placing files on disk.

Storage is intentionally simple.

## Responsibilities

Storage can:

- write an upload stream to a temporary file;
- create directories;
- perform an atomic replace;
- open files;
- delete files;
- check whether a file exists;
- return filesystem errors.

## What Storage Must Not Know

Storage must not know about:

- device authentication;
- pairing;
- conflict policy;
- HTTP;
- REST;
- SQLite schema;
- sync decisions.

## Directory Layout

Example:

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
