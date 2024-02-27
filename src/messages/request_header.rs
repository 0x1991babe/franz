//! RequestHeader
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/RequestHeader.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct RequestHeader {
    /// The API key of this request.
    /// 
    /// Supported API versions: 0-2
    pub request_api_key: i16,

    /// The API version of this request.
    /// 
    /// Supported API versions: 0-2
    pub request_api_version: i16,

    /// The correlation ID of this request.
    /// 
    /// Supported API versions: 0-2
    pub correlation_id: i32,

    /// The client ID string.
    /// 
    /// Supported API versions: 1-2
    pub client_id: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for RequestHeader {
    type Builder = RequestHeaderBuilder;

    fn builder() -> Self::Builder{
        RequestHeaderBuilder::default()
    }
}

impl Encodable for RequestHeader {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i16(self.request_api_key);
        buf.put_i16(self.request_api_version);
        buf.put_i32(self.correlation_id);
        if version >= 1 {
            types::String.encode(buf, &self.client_id)?;
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 2;
        total_size += 2;
        total_size += 4;
        if version >= 1 {
            total_size += types::String.compute_size(&self.client_id)?;
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for RequestHeader {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let request_api_key = buf.try_get_i16()?;
        let request_api_version = buf.try_get_i16()?;
        let correlation_id = buf.try_get_i32()?;
        let client_id = if version >= 1 {
            types::String.decode(buf)?
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
            unknown_tagged_fields,
        })
    }
}

impl Default for RequestHeader {
    fn default() -> Self {
        Self {
            request_api_key: 0,
            request_api_version: 0,
            correlation_id: 0,
            client_id: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for RequestHeader {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

