//! BrokerHeartbeatResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/BrokerHeartbeatResponse.json).
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


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct BrokerHeartbeatResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// True if the broker has approximately caught up with the latest metadata.
    /// 
    /// Supported API versions: 0
    pub is_caught_up: bool,

    /// True if the broker is fenced.
    /// 
    /// Supported API versions: 0
    pub is_fenced: bool,

    /// True if the broker should proceed with its shutdown.
    /// 
    /// Supported API versions: 0
    pub should_shut_down: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for BrokerHeartbeatResponse {
    type Builder = BrokerHeartbeatResponseBuilder;

    fn builder() -> Self::Builder{
        BrokerHeartbeatResponseBuilder::default()
    }
}

impl Encodable for BrokerHeartbeatResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i32(self.throttle_time_ms);
        buf.put_i16(self.error_code);
        types::Boolean.encode(buf, self.is_caught_up)?;
        types::Boolean.encode(buf, self.is_fenced)?;
        types::Boolean.encode(buf, self.should_shut_down)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 4;
        total_size += 2;
        total_size += types::Boolean.compute_size(self.is_caught_up)?;
        total_size += types::Boolean.compute_size(self.is_fenced)?;
        total_size += types::Boolean.compute_size(self.should_shut_down)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for BrokerHeartbeatResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = buf.try_get_i32()?;
        let error_code = buf.try_get_i16()?;
        let is_caught_up = types::Boolean.decode(buf)?;
        let is_fenced = types::Boolean.decode(buf)?;
        let should_shut_down = types::Boolean.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            throttle_time_ms,
            error_code,
            is_caught_up,
            is_fenced,
            should_shut_down,
            unknown_tagged_fields,
        })
    }
}

impl Default for BrokerHeartbeatResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            is_caught_up: false,
            is_fenced: true,
            should_shut_down: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for BrokerHeartbeatResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for BrokerHeartbeatResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}

