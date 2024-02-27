//! ListOffsetsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ListOffsetsResponse.json).
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


/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ListOffsetsPartitionResponse {
    /// The partition index.
    /// 
    /// Supported API versions: 0-7
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-7
    pub error_code: i16,

    /// The result offsets.
    /// 
    /// Supported API versions: 0
    pub old_style_offsets: Vec<i64>,

    /// The timestamp associated with the returned offset.
    /// 
    /// Supported API versions: 1-7
    pub timestamp: i64,

    /// The returned offset.
    /// 
    /// Supported API versions: 1-7
    pub offset: i64,

    /// 
    /// 
    /// Supported API versions: 4-7
    pub leader_epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ListOffsetsPartitionResponse {
    type Builder = ListOffsetsPartitionResponseBuilder;

    fn builder() -> Self::Builder{
        ListOffsetsPartitionResponseBuilder::default()
    }
}

impl Encodable for ListOffsetsPartitionResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i32(self.partition_index);
        buf.put_i16(self.error_code);
        if version == 0 {
            types::Array(types::Int64).encode(buf, &self.old_style_offsets)?;
        } else {
            if !self.old_style_offsets.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            buf.put_i64(self.timestamp);
        } else {
            if self.timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            buf.put_i64(self.offset);
        } else {
            if self.offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            buf.put_i32(self.leader_epoch);
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
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
        total_size += 4;
        total_size += 2;
        if version == 0 {
            total_size += types::Array(types::Int64).compute_size(&self.old_style_offsets)?;
        } else {
            if !self.old_style_offsets.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += 8;
        } else {
            if self.timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += 8;
        } else {
            if self.offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            total_size += 4;
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
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

impl Decodable for ListOffsetsPartitionResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = buf.try_get_i32()?;
        let error_code = buf.try_get_i16()?;
        let old_style_offsets = if version == 0 {
            types::Array(types::Int64).decode(buf)?
        } else {
            Default::default()
        };
        let timestamp = if version >= 1 {
            buf.try_get_i64()?
        } else {
            -1
        };
        let offset = if version >= 1 {
            buf.try_get_i64()?
        } else {
            -1
        };
        let leader_epoch = if version >= 4 {
            buf.try_get_i32()?
        } else {
            -1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            partition_index,
            error_code,
            old_style_offsets,
            timestamp,
            offset,
            leader_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListOffsetsPartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            old_style_offsets: Default::default(),
            timestamp: -1,
            offset: -1,
            leader_epoch: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListOffsetsPartitionResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ListOffsetsTopicResponse {
    /// The topic name
    /// 
    /// Supported API versions: 0-7
    pub name: super::TopicName,

    /// Each partition in the response.
    /// 
    /// Supported API versions: 0-7
    pub partitions: Vec<ListOffsetsPartitionResponse>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ListOffsetsTopicResponse {
    type Builder = ListOffsetsTopicResponseBuilder;

    fn builder() -> Self::Builder{
        ListOffsetsTopicResponseBuilder::default()
    }
}

impl Encodable for ListOffsetsTopicResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 6 {
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
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 6 {
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

impl Decodable for ListOffsetsTopicResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListOffsetsTopicResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListOffsetsTopicResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ListOffsetsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 2-7
    pub throttle_time_ms: i32,

    /// Each topic in the response.
    /// 
    /// Supported API versions: 0-7
    pub topics: Vec<ListOffsetsTopicResponse>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ListOffsetsResponse {
    type Builder = ListOffsetsResponseBuilder;

    fn builder() -> Self::Builder{
        ListOffsetsResponseBuilder::default()
    }
}

impl Encodable for ListOffsetsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            buf.put_i32(self.throttle_time_ms);
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 6 {
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
        if version >= 2 {
            total_size += 4;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 6 {
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

impl Decodable for ListOffsetsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 2 {
            buf.try_get_i32()?
        } else {
            0
        };
        let topics = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListOffsetsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListOffsetsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

impl HeaderVersion for ListOffsetsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            1
        } else {
            0
        }
    }
}

