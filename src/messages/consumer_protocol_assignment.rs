//! ConsumerProtocolAssignment
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ConsumerProtocolAssignment.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, DecodeError,
    Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message,
    StrBytes, VersionRange,
};

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsumerProtocolAssignment {
    ///
    ///
    /// Supported API versions: 0-3
    pub assigned_partitions: indexmap::IndexMap<super::TopicName, TopicPartition>,

    ///
    ///
    /// Supported API versions: 0-3
    pub user_data: Option<Bytes>,
}

impl ConsumerProtocolAssignment {
    /// Sets `assigned_partitions` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-3
    pub fn with_assigned_partitions(
        mut self,
        value: indexmap::IndexMap<super::TopicName, TopicPartition>,
    ) -> Self {
        self.assigned_partitions = value;
        self
    }
    /// Sets `user_data` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-3
    pub fn with_user_data(mut self, value: Option<Bytes>) -> Self {
        self.user_data = value;
        self
    }
}

impl Encodable for ConsumerProtocolAssignment {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.assigned_partitions)?;
        types::Bytes.encode(buf, &self.user_data)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size +=
            types::Array(types::Struct { version }).compute_size(&self.assigned_partitions)?;
        total_size += types::Bytes.compute_size(&self.user_data)?;

        Ok(total_size)
    }
}

impl Decodable for ConsumerProtocolAssignment {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let assigned_partitions = types::Array(types::Struct { version }).decode(buf)?;
        let user_data = types::Bytes.decode(buf)?;
        Ok(Self {
            assigned_partitions,
            user_data,
        })
    }
}

impl Default for ConsumerProtocolAssignment {
    fn default() -> Self {
        Self {
            assigned_partitions: Default::default(),
            user_data: None,
        }
    }
}

impl Message for ConsumerProtocolAssignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartition {
    ///
    ///
    /// Supported API versions: 0-3
    pub partitions: Vec<i32>,
}

impl TopicPartition {
    /// Sets `partitions` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-3
    pub fn with_partitions(mut self, value: Vec<i32>) -> Self {
        self.partitions = value;
        self
    }
}

impl MapEncodable for TopicPartition {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(
        &self,
        key: &Self::Key,
        buf: &mut B,
        version: i16,
    ) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Int32).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Int32).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl MapDecodable for TopicPartition {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Int32).decode(buf)?;
        Ok((key_field, Self { partitions }))
    }
}

impl Default for TopicPartition {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for TopicPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}
