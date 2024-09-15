//! Provides compression utilities for encoding records.
//!
//! This module has implementations of gzip, Snappy, as well as a noop compression format that
//! allows encoding and decoding records into a [`Record`](crate::records::Record).

use crate::protocol::buf::{ByteBuf, ByteBufMut};
use anyhow::Result;

/// A trait for record compression algorithms.
pub trait Compressor<B: ByteBufMut> {
    /// Target buffer type for compression.
    type BufMut: ByteBufMut;
    /// Compresses into provided [`ByteBufMut`], with records encoded by `F` into `R`.
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R>;
}

/// A trait for record decompression algorithms.
pub trait Decompressor<B: ByteBuf> {
    /// Target buffer type for decompression.
    type Buf: ByteBuf;
    /// Decompress records from `B` mapped using `F` into `R`.
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R>;
}
