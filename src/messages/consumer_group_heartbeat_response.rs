//! ConsumerGroupHeartbeatResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ConsumerGroupHeartbeatResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct Assignment {
    /// The partitions assigned to the member that can be used immediately.
    ///
    /// Supported API versions: 0
    pub topic_partitions: Vec<TopicPartitions>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for Assignment {
    type Builder = AssignmentBuilder;

    fn builder() -> Self::Builder {
        AssignmentBuilder::default()
    }
}

impl Encodable for Assignment {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactArray(types::Struct { version }).encode(buf, &self.topic_partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.topic_partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for Assignment {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_partitions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            topic_partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for Assignment {
    fn default() -> Self {
        Self {
            topic_partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for Assignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ConsumerGroupHeartbeatResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The top-level error code, or 0 if there was no error
    ///
    /// Supported API versions: 0
    pub error_code: i16,

    /// The top-level error message, or null if there was no error.
    ///
    /// Supported API versions: 0
    pub error_message: Option<StrBytes>,

    /// The member id generated by the coordinator. Only provided when the member joins with MemberEpoch == 0.
    ///
    /// Supported API versions: 0
    pub member_id: Option<StrBytes>,

    /// The member epoch.
    ///
    /// Supported API versions: 0
    pub member_epoch: i32,

    /// The heartbeat interval in milliseconds.
    ///
    /// Supported API versions: 0
    pub heartbeat_interval_ms: i32,

    /// null if not provided; the assignment otherwise.
    ///
    /// Supported API versions: 0
    pub assignment: Option<Assignment>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ConsumerGroupHeartbeatResponse {
    type Builder = ConsumerGroupHeartbeatResponseBuilder;

    fn builder() -> Self::Builder {
        ConsumerGroupHeartbeatResponseBuilder::default()
    }
}

impl Encodable for ConsumerGroupHeartbeatResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::CompactString.encode(buf, &self.error_message)?;
        types::CompactString.encode(buf, &self.member_id)?;
        types::Int32.encode(buf, &self.member_epoch)?;
        types::Int32.encode(buf, &self.heartbeat_interval_ms)?;
        types::OptionStruct { version }.encode(buf, &self.assignment)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::CompactString.compute_size(&self.error_message)?;
        total_size += types::CompactString.compute_size(&self.member_id)?;
        total_size += types::Int32.compute_size(&self.member_epoch)?;
        total_size += types::Int32.compute_size(&self.heartbeat_interval_ms)?;
        total_size += types::OptionStruct { version }.compute_size(&self.assignment)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for ConsumerGroupHeartbeatResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let error_message = types::CompactString.decode(buf)?;
        let member_id = types::CompactString.decode(buf)?;
        let member_epoch = types::Int32.decode(buf)?;
        let heartbeat_interval_ms = types::Int32.decode(buf)?;
        let assignment = types::OptionStruct { version }.decode(buf)?;
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
            error_message,
            member_id,
            member_epoch,
            heartbeat_interval_ms,
            assignment,
            unknown_tagged_fields,
        })
    }
}

impl Default for ConsumerGroupHeartbeatResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: None,
            member_id: None,
            member_epoch: 0,
            heartbeat_interval_ms: 0,
            assignment: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ConsumerGroupHeartbeatResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TopicPartitions {
    /// The topic ID.
    ///
    /// Supported API versions: 0
    pub topic_id: Uuid,

    /// The partitions.
    ///
    /// Supported API versions: 0
    pub partitions: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for TopicPartitions {
    type Builder = TopicPartitionsBuilder;

    fn builder() -> Self::Builder {
        TopicPartitionsBuilder::default()
    }
}

impl Encodable for TopicPartitions {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Uuid.encode(buf, &self.topic_id)?;
        types::CompactArray(types::Int32).encode(buf, &self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Uuid.compute_size(&self.topic_id)?;
        total_size += types::CompactArray(types::Int32).compute_size(&self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for TopicPartitions {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_id = types::Uuid.decode(buf)?;
        let partitions = types::CompactArray(types::Int32).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            topic_id,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicPartitions {
    fn default() -> Self {
        Self {
            topic_id: Uuid::nil(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicPartitions {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for ConsumerGroupHeartbeatResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}