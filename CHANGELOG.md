# Changelog

## Unreleased

- Update protocol to kafka 3.8.0

## v0.12.0

- Add `client` feature to enable encoding of requests and decoding of responses (enabled by default)
- Add `broker` feature to enable encoding of responses and decoding of requests (enabled by default)
- Add `messages_enums` feature to enable `ResponseKind` and `RequestKind` enums
- Add `encode`, `decode` and `header_version` methods to `ResponseKind` and `RequestKind`

## v0.11.0

- Replace `derive_builder` with custom impl.
- Remove `EncodeError`/`DecodeError` aliases
- Remove panic from `RecordBatchEncoder::encode`
- Use `IntoIterator` instead of `Iterator` for `RecordBatchEncoder::encode`
- Use CRC-32 ISO/HDLC instead of CRC-32 CKSUM.
- Add `Display` and more `From<T>` implementations for `StrBytes`.
- Avoid redunand variant names in RequestKind/ResponseKind.

## v0.10.2

- Implement From<T> for RequestKind and ResponseKind.

## v0.10.1

- Fix lz4 compression.

## v0.10.0

- Use `anyhow` for public error types. In general, these errors reflect non-recoverable programmer error and
should not occur when interacting with a well-formed Kafka implementation.
- Rename `StrBytes::from_str` to `from_static_str` to avoid overlap with the `FromStr` trait.
- Improve codegen, including filtering out unstable messages, improving ordering of structs in generated code,
and other misc improvements.
- Upgrade message version to upstream `v3.7.0` Kafka.

## v0.9.0

- Remove the `string` crate.
- Update `derive-builder`.

## v0.8.2

- Expose protocol::buf::gap module.

## v0.8.1

- Re-export `indexmap` as part of the public api.

## v0.8.0

- Implement Zstd and Lz4 compression.

## v0.7.0

- Switch to [crc32c](https://crates.io/crates/crc32c) crate, providing hardware accelration for crc operations
on supported platforms.
- Formatting fixes.
- Miscellaneous dependency updates.
- Fix issue with `derive_builder` implementation in which default fields were not being used correctly.

## v0.6.1

- Miscellaneous dependency updates.

## v0.6.0

- Add `ApiKey::request_header_version` and `ApiKey::response_header_version` to assist deserializing
headers without reference to message type.

## v0.5.1

Bump `uuid` crate version.

## v0.5.0

- Clarify versioning support for upstream Kafka, tracking the latest Kafka
stable release. Improve docs.

#### Enhancements

- `Builder::builder` trait for retrieving builder instances from messages
without requiring extra builder imports.
- `#[non_exhaustive]` added to all items to ensure forward compatability
with protocol upgrades.

## v0.4.0

- Add utilities for dealing with response error codes.

## v0.3.0

#### Enhancements

- Build messages from crate root.
- Upgrade to latest version of Kafka protocol.

## v0.2.0

#### Enhancements

- Add derive builder for all messages.

## v0.1.0

Initial release.
