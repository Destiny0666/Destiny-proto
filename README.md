# Destiny Proto

Protocol definitions shared by Destiny services.

This repository contains:

- gRPC service contracts
- Protobuf messages
- RabbitMQ event contracts
- Shared protocol primitives

## Structure

```text
proto/
├── auth/
│   └── v1/
│       ├── auth.proto
│       └── session.proto
├── events/
│   └── v1/
│       └── event.proto
└── common/
    └── v1/
        └── common.proto