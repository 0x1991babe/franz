//! DescribeClusterRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeClusterRequest.json).
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

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeClusterRequest {
    /// Whether to include cluster authorized operations.
    ///
    /// Supported API versions: 0-1
    pub include_cluster_authorized_operations: bool,

    /// The endpoint type to describe. 1=brokers, 2=controllers.
    ///
    /// Supported API versions: 1
    pub endpoint_type: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeClusterRequest {
    type Builder = DescribeClusterRequestBuilder;

    fn builder() -> Self::Builder {
        DescribeClusterRequestBuilder::default()
    }
}

impl Encodable for DescribeClusterRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Boolean.encode(buf, &self.include_cluster_authorized_operations)?;
        if version >= 1 {
            types::Int8.encode(buf, &self.endpoint_type)?;
        } else {
            if self.endpoint_type != 1 {
                bail!("failed to encode");
            }
        }
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
        total_size += types::Boolean.compute_size(&self.include_cluster_authorized_operations)?;
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.endpoint_type)?;
        } else {
            if self.endpoint_type != 1 {
                bail!("failed to encode");
            }
        }
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

impl Decodable for DescribeClusterRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let include_cluster_authorized_operations = types::Boolean.decode(buf)?;
        let endpoint_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            include_cluster_authorized_operations,
            endpoint_type,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeClusterRequest {
    fn default() -> Self {
        Self {
            include_cluster_authorized_operations: false,
            endpoint_type: 1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeClusterRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for DescribeClusterRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
