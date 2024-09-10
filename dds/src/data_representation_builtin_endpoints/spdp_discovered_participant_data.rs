use super::{
    parameter_id_values::{
        DEFAULT_DOMAIN_TAG, DEFAULT_EXPECTS_INLINE_QOS, DEFAULT_PARTICIPANT_LEASE_DURATION,
        PID_BUILTIN_ENDPOINT_QOS, PID_BUILTIN_ENDPOINT_SET, PID_DEFAULT_MULTICAST_LOCATOR,
        PID_DEFAULT_UNICAST_LOCATOR, PID_DISCOVERED_PARTICIPANT, PID_DOMAIN_ID, PID_DOMAIN_TAG,
        PID_EXPECTS_INLINE_QOS, PID_METATRAFFIC_MULTICAST_LOCATOR, PID_METATRAFFIC_UNICAST_LOCATOR,
        PID_PARTICIPANT_GUID, PID_PARTICIPANT_LEASE_DURATION,
        PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT, PID_PROTOCOL_VERSION, PID_USER_DATA, PID_VENDORID,
    },
    payload_serializer_deserializer::{
        parameter_list_deserializer::ParameterListCdrDeserializer,
        parameter_list_serializer::ParameterListCdrSerializer,
    },
};
use crate::{
    builtin_topics::ParticipantBuiltinTopicData,
    domain::domain_participant_factory::DomainId,
    infrastructure::{error::DdsResult, instance::InstanceHandle, time::Duration},
    rtps::{
        discovery_types::{BuiltinEndpointQos, BuiltinEndpointSet},
        messages::types::Count,
        types::{GuidPrefix, Locator, ProtocolVersion, VendorId},
    },
    topic_definition::type_support::{DdsDeserialize, DdsHasKey, DdsKey, DdsSerialize, DdsTypeXml},
};

pub const DCPS_PARTICIPANT: &str = "DCPSParticipant";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParticipantProxy {
    pub(crate) domain_id: Option<DomainId>,
    pub(crate) domain_tag: String,
    pub(crate) protocol_version: ProtocolVersion,
    pub(crate) guid_prefix: GuidPrefix,
    pub(crate) vendor_id: VendorId,
    pub(crate) expects_inline_qos: bool,
    pub(crate) metatraffic_unicast_locator_list: Vec<Locator>,
    pub(crate) metatraffic_multicast_locator_list: Vec<Locator>,
    pub(crate) default_unicast_locator_list: Vec<Locator>,
    pub(crate) default_multicast_locator_list: Vec<Locator>,
    pub(crate) available_builtin_endpoints: BuiltinEndpointSet,
    pub(crate) manual_liveliness_count: Count,
    pub(crate) builtin_endpoint_qos: BuiltinEndpointQos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpdpDiscoveredParticipantData {
    pub(crate) dds_participant_data: ParticipantBuiltinTopicData,
    pub(crate) participant_proxy: ParticipantProxy,
    pub(crate) lease_duration: Duration,
    pub(crate) discovered_participant_list: Vec<InstanceHandle>,
}

impl DdsSerialize for SpdpDiscoveredParticipantData {
    fn serialize_data(&self) -> DdsResult<Vec<u8>> {
        let mut serializer = ParameterListCdrSerializer::default();
        serializer.write_header()?;

        // dds_participant_data: ParticipantBuiltinTopicData :
        serializer.write(PID_PARTICIPANT_GUID, &self.dds_participant_data.key)?;
        serializer.write_with_default(
            PID_USER_DATA,
            &self.dds_participant_data.user_data,
            &Default::default(),
        )?;

        // participant_proxy: ParticipantProxy :
        if let Some(domain_id) = &self.participant_proxy.domain_id {
            serializer.write(PID_DOMAIN_ID, domain_id)?;
        }
        serializer.write_with_default(
            PID_DOMAIN_TAG,
            &self.participant_proxy.domain_tag,
            &String::from(DEFAULT_DOMAIN_TAG),
        )?;
        serializer.write(
            PID_PROTOCOL_VERSION,
            &self.participant_proxy.protocol_version,
        )?;
        serializer.write(PID_VENDORID, &self.participant_proxy.vendor_id)?;
        serializer.write_with_default(
            PID_EXPECTS_INLINE_QOS,
            &self.participant_proxy.expects_inline_qos,
            &DEFAULT_EXPECTS_INLINE_QOS,
        )?;
        serializer.write_collection(
            PID_METATRAFFIC_UNICAST_LOCATOR,
            &self.participant_proxy.metatraffic_unicast_locator_list,
        )?;
        serializer.write_collection(
            PID_METATRAFFIC_MULTICAST_LOCATOR,
            &self.participant_proxy.metatraffic_multicast_locator_list,
        )?;
        serializer.write_collection(
            PID_DEFAULT_UNICAST_LOCATOR,
            &self.participant_proxy.default_unicast_locator_list,
        )?;
        serializer.write_collection(
            PID_DEFAULT_MULTICAST_LOCATOR,
            &self.participant_proxy.default_multicast_locator_list,
        )?;
        serializer.write(
            PID_BUILTIN_ENDPOINT_SET,
            &self.participant_proxy.available_builtin_endpoints,
        )?;
        serializer.write_with_default(
            PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT,
            &self.participant_proxy.manual_liveliness_count,
            &Default::default(),
        )?;
        serializer.write_with_default(
            PID_BUILTIN_ENDPOINT_QOS,
            &self.participant_proxy.builtin_endpoint_qos,
            &Default::default(),
        )?;

        // Default (DEFAULT_PARTICIPANT_LEASE_DURATION) is ommited compared to the standard due to interoperability reasons :
        serializer.write(PID_PARTICIPANT_LEASE_DURATION, &self.lease_duration)?;

        serializer.write_collection(
            PID_DISCOVERED_PARTICIPANT,
            &self.discovered_participant_list,
        )?;

        serializer.write_sentinel()?;
        Ok(serializer.writer)
    }
}

impl<'de> DdsDeserialize<'de> for SpdpDiscoveredParticipantData {
    fn deserialize_data(serialized_data: &'de [u8]) -> DdsResult<Self> {
        let pl_deserializer = ParameterListCdrDeserializer::new(serialized_data)?;
        let domain_id = match pl_deserializer.read(PID_DOMAIN_ID) {
            Ok(domain_id) => Some(domain_id),
            Err(_) => None,
        };
        Ok(Self {
            dds_participant_data: ParticipantBuiltinTopicData::deserialize_data(serialized_data)?,
            participant_proxy: ParticipantProxy {
                domain_id,
                domain_tag: pl_deserializer
                    .read_with_default(PID_DOMAIN_TAG, Default::default())?,
                protocol_version: pl_deserializer.read(PID_PROTOCOL_VERSION)?,
                guid_prefix: pl_deserializer.read(PID_PARTICIPANT_GUID)?,
                vendor_id: pl_deserializer.read(PID_VENDORID)?,
                expects_inline_qos: pl_deserializer
                    .read_with_default(PID_EXPECTS_INLINE_QOS, DEFAULT_EXPECTS_INLINE_QOS)?,
                metatraffic_unicast_locator_list: pl_deserializer
                    .read_collection(PID_METATRAFFIC_UNICAST_LOCATOR)?,
                metatraffic_multicast_locator_list: pl_deserializer
                    .read_collection(PID_METATRAFFIC_MULTICAST_LOCATOR)?,
                default_unicast_locator_list: pl_deserializer
                    .read_collection(PID_DEFAULT_UNICAST_LOCATOR)?,
                default_multicast_locator_list: pl_deserializer
                    .read_collection(PID_DEFAULT_MULTICAST_LOCATOR)?,
                available_builtin_endpoints: pl_deserializer.read(PID_BUILTIN_ENDPOINT_SET)?,
                // Default value is a deviation from the standard and is used for interoperability reasons:
                manual_liveliness_count: pl_deserializer.read_with_default(
                    PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT,
                    Default::default(),
                )?,
                // Default value is a deviation from the standard and is used for interoperability reasons:
                builtin_endpoint_qos: pl_deserializer
                    .read_with_default(PID_BUILTIN_ENDPOINT_QOS, Default::default())?,
            },
            lease_duration: pl_deserializer.read_with_default(
                PID_PARTICIPANT_LEASE_DURATION,
                DEFAULT_PARTICIPANT_LEASE_DURATION,
            )?,
            discovered_participant_list: pl_deserializer
                .read_collection(PID_DISCOVERED_PARTICIPANT)?,
        })
    }
}

impl SpdpDiscoveredParticipantData {
    pub fn dds_participant_data(&self) -> &ParticipantBuiltinTopicData {
        &self.dds_participant_data
    }

    pub fn domain_id(&self) -> Option<DomainId> {
        self.participant_proxy.domain_id
    }

    pub fn domain_tag(&self) -> &str {
        self.participant_proxy.domain_tag.as_ref()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.participant_proxy.protocol_version
    }

    pub fn guid_prefix(&self) -> GuidPrefix {
        self.participant_proxy.guid_prefix
    }

    pub fn vendor_id(&self) -> VendorId {
        self.participant_proxy.vendor_id
    }

    pub fn expects_inline_qos(&self) -> bool {
        self.participant_proxy.expects_inline_qos
    }

    pub fn metatraffic_unicast_locator_list(&self) -> &[Locator] {
        self.participant_proxy.metatraffic_unicast_locator_list.as_ref()
    }

    pub fn metatraffic_multicast_locator_list(&self) -> &[Locator] {
        self.participant_proxy.metatraffic_multicast_locator_list.as_ref()
    }

    pub fn default_unicast_locator_list(&self) -> &[Locator] {
        self.participant_proxy.default_unicast_locator_list.as_ref()
    }

    pub fn default_multicast_locator_list(&self) -> &[Locator] {
        self.participant_proxy.default_multicast_locator_list.as_ref()
    }

    pub fn available_builtin_endpoints(&self) -> BuiltinEndpointSet {
        self.participant_proxy.available_builtin_endpoints
    }

    pub fn manual_liveliness_count(&self) -> Count {
        self.participant_proxy.manual_liveliness_count
    }

    pub fn builtin_endpoint_qos(&self) -> BuiltinEndpointQos {
        self.participant_proxy.builtin_endpoint_qos
    }

    pub fn lease_duration(&self) -> &Duration {
        &self.lease_duration
    }

    pub fn discovered_participant_list(&self) -> &[InstanceHandle] {
        &self.discovered_participant_list
    }
}

impl DdsHasKey for SpdpDiscoveredParticipantData {
    const HAS_KEY: bool = true;
}

impl DdsKey for SpdpDiscoveredParticipantData {
    type Key = [u8; 16];

    fn get_key(&self) -> DdsResult<Self::Key> {
        Ok(self.dds_participant_data.key().value)
    }

    fn get_key_from_serialized_data(serialized_foo: &[u8]) -> DdsResult<Self::Key> {
        Ok(Self::deserialize_data(serialized_foo)?
            .dds_participant_data
            .key()
            .value)
    }
}

impl DdsTypeXml for SpdpDiscoveredParticipantData {
    fn get_type_xml() -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{builtin_topics::BuiltInTopicKey, infrastructure::qos_policy::UserDataQosPolicy};

    #[test]
    fn serialize_spdp_discovered_participant_data() {
        let locator1 = Locator::new(11, 12, [1; 16]);
        let locator2 = Locator::new(21, 22, [2; 16]);

        let domain_id = Some(1);
        let domain_tag = "ab".to_string();
        let protocol_version = ProtocolVersion::new(2, 4);
        let guid_prefix = [8; 12];
        let vendor_id = [73, 74];
        let expects_inline_qos = true;
        let metatraffic_unicast_locator_list = vec![locator1, locator2];
        let metatraffic_multicast_locator_list = vec![locator1];
        let default_unicast_locator_list = vec![locator1];
        let default_multicast_locator_list = vec![locator1];
        let available_builtin_endpoints =
            BuiltinEndpointSet::new(BuiltinEndpointSet::BUILTIN_ENDPOINT_PARTICIPANT_DETECTOR);
        let manual_liveliness_count = 2;
        let builtin_endpoint_qos = BuiltinEndpointQos::new(
            BuiltinEndpointQos::BEST_EFFORT_PARTICIPANT_MESSAGE_DATA_READER,
        );
        let lease_duration = Duration::new(10, 11);

        let data = SpdpDiscoveredParticipantData {
            dds_participant_data: ParticipantBuiltinTopicData {
                key: BuiltInTopicKey {
                    value: [8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 1, 0xc1],
                },
                user_data: UserDataQosPolicy { value: vec![] },
            },
            participant_proxy: ParticipantProxy {
                domain_id,
                domain_tag,
                protocol_version,
                guid_prefix,
                vendor_id,
                expects_inline_qos,
                metatraffic_unicast_locator_list,
                metatraffic_multicast_locator_list,
                default_unicast_locator_list,
                default_multicast_locator_list,
                available_builtin_endpoints,
                manual_liveliness_count,
                builtin_endpoint_qos,
            },
            lease_duration,
            discovered_participant_list: vec![],
        };

        let expected = vec![
            0x00, 0x03, 0x00, 0x00, // PL_CDR_LE
            0x50, 0x00, 16, 0x00, // PID_PARTICIPANT_GUID, Length
            8, 8, 8, 8, // GuidPrefix
            8, 8, 8, 8, // GuidPrefix
            8, 8, 8, 8, // GuidPrefix
            0, 0, 1, 0xc1, // EntityId
            0x0f, 0x00, 0x04, 0x00, // PID_DOMAIN_ID, Length: 4
            0x01, 0x00, 0x00, 0x00, // DomainId
            0x14, 0x40, 0x08, 0x00, // PID_DOMAIN_TAG, Length: 8
            3, 0x00, 0x00, 0x00, // DomainTag: string length (incl. terminator)
            b'a', b'b', 0, 0x00, // DomainTag: string + padding (1 byte)
            0x15, 0x00, 4, 0x00, // PID_PROTOCOL_VERSION, Length
            0x02, 0x04, 0x00, 0x00, // ProtocolVersion
            0x16, 0x00, 4, 0x00, // PID_VENDORID
            73, 74, 0x00, 0x00, // VendorId
            0x43, 0x00, 0x04, 0x00, // PID_EXPECTS_INLINE_QOS, Length: 4,
            0x01, 0x00, 0x00, 0x00, // True
            0x32, 0x00, 24, 0x00, // PID_METATRAFFIC_UNICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x32, 0x00, 24, 0x00, // PID_METATRAFFIC_UNICAST_LOCATOR
            21, 0x00, 0x00, 0x00, // Locator{kind
            22, 0x00, 0x00, 0x00, // port,
            0x02, 0x02, 0x02, 0x02, //
            0x02, 0x02, 0x02, 0x02, // address
            0x02, 0x02, 0x02, 0x02, //
            0x02, 0x02, 0x02, 0x02, // }
            0x33, 0x00, 24, 0x00, // PID_METATRAFFIC_MULTICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x31, 0x00, 24, 0x00, // PID_DEFAULT_UNICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x48, 0x00, 24, 0x00, // PID_DEFAULT_MULTICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x58, 0x00, 4, 0x00, // PID_BUILTIN_ENDPOINT_SET
            0x02, 0x00, 0x00, 0x00, //
            0x34, 0x00, 4, 0x00, // PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT
            0x02, 0x00, 0x00, 0x00, // Count
            0x77, 0x00, 4, 0x00, // PID_BUILTIN_ENDPOINT_QOS
            0x00, 0x00, 0x00, 0x20, //
            0x02, 0x00, 8, 0x00, // PID_PARTICIPANT_LEASE_DURATION
            10, 0x00, 0x00, 0x00, // Duration: seconds
            11, 0x00, 0x00, 0x00, // Duration: fraction
            0x01, 0x00, 0x00, 0x00, // PID_SENTINEL
        ];
        let result = data.serialize_data().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_spdp_discovered_participant_data() {
        let locator1 = Locator::new(11, 12, [1; 16]);
        let locator2 = Locator::new(21, 22, [2; 16]);

        let domain_id = 1;
        let domain_tag = "ab".to_string();
        let protocol_version = ProtocolVersion::new(2, 4);
        let guid_prefix = [8; 12];
        let vendor_id = [73, 74];
        let expects_inline_qos = true;
        let metatraffic_unicast_locator_list = vec![locator1, locator2];
        let metatraffic_multicast_locator_list = vec![locator1];
        let default_unicast_locator_list = vec![locator1];
        let default_multicast_locator_list = vec![locator1];
        let available_builtin_endpoints =
            BuiltinEndpointSet::new(BuiltinEndpointSet::BUILTIN_ENDPOINT_PARTICIPANT_DETECTOR);
        let manual_liveliness_count = 2;
        let builtin_endpoint_qos = BuiltinEndpointQos::new(
            BuiltinEndpointQos::BEST_EFFORT_PARTICIPANT_MESSAGE_DATA_READER,
        );
        let lease_duration = Duration::new(10, 11);

        let expected = SpdpDiscoveredParticipantData {
            dds_participant_data: ParticipantBuiltinTopicData {
                key: BuiltInTopicKey {
                    value: [8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 1, 0xc1],
                },
                user_data: UserDataQosPolicy { value: vec![] },
            },
            participant_proxy: ParticipantProxy {
                domain_id: Some(domain_id),
                domain_tag,
                protocol_version,
                guid_prefix,
                vendor_id,
                expects_inline_qos,
                metatraffic_unicast_locator_list,
                metatraffic_multicast_locator_list,
                default_unicast_locator_list,
                default_multicast_locator_list,
                available_builtin_endpoints,
                manual_liveliness_count,
                builtin_endpoint_qos,
            },
            lease_duration,
            discovered_participant_list: vec![],
        };

        let mut data = &[
            0x00, 0x03, 0x00, 0x00, // PL_CDR_LE
            0x0f, 0x00, 0x04, 0x00, // PID_DOMAIN_ID, Length: 4
            0x01, 0x00, 0x00, 0x00, // DomainId
            0x14, 0x40, 0x08, 0x00, // PID_DOMAIN_TAG, Length: 8
            3, 0x00, 0x00, 0x00, // DomainTag: string length (incl. terminator)
            b'a', b'b', 0, 0x00, // DomainTag: string + padding (1 byte)
            0x15, 0x00, 4, 0x00, // PID_PROTOCOL_VERSION, Length
            0x02, 0x04, 0x00, 0x00, // ProtocolVersion
            0x50, 0x00, 16, 0x00, // PID_PARTICIPANT_GUID, Length
            8, 8, 8, 8, // GuidPrefix
            8, 8, 8, 8, // GuidPrefix
            8, 8, 8, 8, // GuidPrefix
            0, 0, 1, 0xc1, // EntityId,
            0x16, 0x00, 4, 0x00, // PID_VENDORID
            73, 74, 0x00, 0x00, // VendorId
            0x43, 0x00, 0x04, 0x00, // PID_EXPECTS_INLINE_QOS, Length: 4,
            0x01, 0x00, 0x00, 0x00, // True
            0x32, 0x00, 24, 0x00, // PID_METATRAFFIC_UNICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x32, 0x00, 24, 0x00, // PID_METATRAFFIC_UNICAST_LOCATOR
            21, 0x00, 0x00, 0x00, // Locator{kind
            22, 0x00, 0x00, 0x00, // port,
            0x02, 0x02, 0x02, 0x02, //
            0x02, 0x02, 0x02, 0x02, // address
            0x02, 0x02, 0x02, 0x02, //
            0x02, 0x02, 0x02, 0x02, // }
            0x33, 0x00, 24, 0x00, // PID_METATRAFFIC_MULTICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x31, 0x00, 24, 0x00, // PID_DEFAULT_UNICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x48, 0x00, 24, 0x00, // PID_DEFAULT_MULTICAST_LOCATOR
            11, 0x00, 0x00, 0x00, // Locator{kind
            12, 0x00, 0x00, 0x00, // port,
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // address
            0x01, 0x01, 0x01, 0x01, //
            0x01, 0x01, 0x01, 0x01, // }
            0x58, 0x00, 4, 0x00, // PID_BUILTIN_ENDPOINT_SET
            0x02, 0x00, 0x00, 0x00, //
            0x34, 0x00, 4, 0x00, // PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT
            0x02, 0x00, 0x00, 0x00, // Count
            0x77, 0x00, 4, 0x00, // PID_BUILTIN_ENDPOINT_QOS
            0x00, 0x00, 0x00, 0x20, //
            0x02, 0x00, 8, 0x00, // PID_PARTICIPANT_LEASE_DURATION
            10, 0x00, 0x00, 0x00, // Duration: seconds
            11, 0x00, 0x00, 0x00, // Duration: fraction
            0x01, 0x00, 0x00, 0x00, // PID_SENTINEL
        ][..];
        let result = SpdpDiscoveredParticipantData::deserialize_data(&mut data).unwrap();
        assert_eq!(result, expected);
    }
}
