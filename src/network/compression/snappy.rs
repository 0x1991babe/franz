use bytes::{Bytes, BytesMut};
use snap::raw::*;
use log::error;

use franz_protocol::buf::{ByteBuf, ByteBufMut};
use franz_protocol::{EncodeError, DecodeError};

use super::{Compressor, Decompressor};

pub struct Snappy;

impl<B: ByteBufMut> Compressor<B> for Snappy {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        // Compress directly into the target buffer
        let start_pos = buf.offset();
        let compress_gap = buf.put_gap(max_compress_len(tmp.len()));
        let actual_len = Encoder::new().compress(&tmp, buf.gap_buf(compress_gap)).map_err(|e| {
            error!("Failed to compress buffer: {}", e);
            EncodeError
        })?;
        buf.seek(start_pos + actual_len);

        Ok(res)
    }
}

impl<B: ByteBuf> Decompressor<B> for Snappy {
    type Buf = Bytes;
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>
    {
        // Allocate a temporary buffer to hold the uncompressed bytes
        let buf = buf.to_bytes();
        let actual_len = decompress_len(&buf).map_err(|e| {
            error!("Failed to decompress buffer: {}", e);
            DecodeError
        })?;
        let mut tmp = BytesMut::new();
        tmp.resize(actual_len, 0);

        // Decompress directly from the input buffer
        Decoder::new().decompress(&buf, &mut tmp).map_err(|e| {
            error!("Failed to decompress buffer: {}", e);
            DecodeError
        })?;

        f(&mut tmp.into())
    }
}
