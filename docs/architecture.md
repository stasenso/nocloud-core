# Architecture

nocloud-core is a managed filesystem with synchronization support.

It is NOT:

- object storage;
- an S3 clone;
- a Dropbox clone;
- a distributed clustered storage system.

The main goal is simple self-hosted file synchronization for regular people.

## Core Idea

Files are stored once in ordinary directories.

Metadata is stored separately in SQLite.

A hash is used as a content fingerprint, not as a file name.

## Layers

```text
Gateway
  ↓
REST API
  ↓
Core / Sync logic
  ↓
Storage + Metadata
```
