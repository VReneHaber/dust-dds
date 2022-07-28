use std::{
    io::{self, ErrorKind},
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
    sync::Mutex,
};

use crate::{
    dcps_psm::{DomainId, StatusMask},
    infrastructure::{
        entity::Entity,
        qos::{DomainParticipantFactoryQos, DomainParticipantQos},
    },
};
use crate::{
    implementation::{
        dds_impl::domain_participant_impl::{
            AnnounceParticipant, CreateBuiltIns, DomainParticipantImpl, ReceiveBuiltInData,
            ReceiveUserDefinedData, SedpReaderDiscovery, SedpWriterDiscovery, SendBuiltInData,
            SendUserDefinedData, SpdpParticipantDiscovery,
        },
        task_manager::TaskManager,
        utils::shared_object::DdsShared,
    },
    return_type::{DdsError, DdsResult},
};
use mac_address::MacAddress;

use lazy_static::lazy_static;

use rtps_pim::structure::types::{GuidPrefix, LOCATOR_KIND_UDPv4, Locator};
use rtps_udp_psm::udp_transport::{UdpMulticastTransport, UdpUnicastTransport};
use socket2::Socket;

use crate::domain::domain_participant_listener::DomainParticipantListener;

use super::domain_participant::DomainParticipant;

/// The DomainParticipant object plays several roles:
/// - It acts as a container for all other Entity objects.
/// - It acts as factory for the Publisher, Subscriber, Topic, and MultiTopic Entity objects.
/// - It represents the participation of the application on a communication plane that isolates applications running on the
/// same set of physical computers from each other. A domain establishes a “virtual network” linking all applications that
/// share the same domainId and isolating them from applications running on different domains. In this way, several
/// independent distributed applications can coexist in the same physical network without interfering, or even being aware
/// of each other.
/// - It provides administration services in the domain, offering operations that allow the application to ‘ignore’ locally any
/// information about a given participant (ignore_participant), publication (ignore_publication), subscription
/// (ignore_subscription), or topic (ignore_topic).
///
/// The following sub clauses explain all the operations in detail.
/// The following operations may be called even if the DomainParticipant is not enabled. Other operations will have the value
/// NOT_ENABLED if called on a disabled DomainParticipant:
/// - Operations defined at the base-class level namely, set_qos, get_qos, set_listener, get_listener, and enable.
/// - Factory methods: create_topic, create_publisher, create_subscriber, delete_topic, delete_publisher,
/// delete_subscriber
/// - Operations that access the status: get_statuscondition

// As of 9.6.1.4.1  Default multicast address
const DEFAULT_MULTICAST_LOCATOR_ADDRESS: [u8; 16] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 239, 255, 0, 1];

const PB: u16 = 7400;
const DG: u16 = 250;
const PG: u16 = 2;
#[allow(non_upper_case_globals)]
const d0: u16 = 0;
#[allow(non_upper_case_globals)]
const d1: u16 = 10;
#[allow(non_upper_case_globals)]
const _d2: u16 = 1;
#[allow(non_upper_case_globals)]
const d3: u16 = 11;

pub fn port_builtin_multicast(domain_id: u16) -> u16 {
    PB + DG * domain_id + d0
}

pub fn port_builtin_unicast(domain_id: u16, participant_id: u16) -> u16 {
    PB + DG * domain_id + d1 + PG * participant_id
}

pub fn port_user_unicast(domain_id: u16, participant_id: u16) -> u16 {
    PB + DG * domain_id + d3 + PG * participant_id
}

pub fn get_multicast_socket(multicast_address: Ipv4Addr, port: u16) -> io::Result<UdpSocket> {
    let socket_addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    let socket = Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::DGRAM,
        Some(socket2::Protocol::UDP),
    )?;

    socket.set_reuse_address(true)?;

    //socket.set_nonblocking(true).ok()?;
    socket.set_read_timeout(Some(std::time::Duration::from_millis(50)))?;

    socket.bind(&socket_addr.into())?;

    socket.join_multicast_v4(&multicast_address, &Ipv4Addr::UNSPECIFIED)?;
    socket.set_multicast_loop_v4(true)?;

    Ok(socket.into())
}

pub fn get_unicast_socket(port: u16) -> io::Result<UdpSocket> {
    let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))?;
    socket.set_nonblocking(true)?;

    Ok(socket)
}

fn ipv4_from_locator(address: &[u8; 16]) -> Ipv4Addr {
    [address[12], address[13], address[14], address[15]].into()
}

#[rustfmt::skip]
fn locator_from_ipv4(address: Ipv4Addr) -> [u8; 16] {
    [0, 0, 0, 0,
     0, 0, 0, 0,
     0, 0, 0, 0,
     address.octets()[0], address.octets()[1], address.octets()[2], address.octets()[3]]
}

pub struct Communications {
    pub domain_id: DomainId,
    pub participant_id: usize,
    pub guid_prefix: GuidPrefix,
    pub unicast_address_list: Vec<Ipv4Addr>,
    pub multicast_address: Ipv4Addr,
    pub metatraffic_multicast: UdpMulticastTransport,
    pub metatraffic_unicast: UdpUnicastTransport,
    pub default_unicast: UdpUnicastTransport,
}

impl Communications {
    pub fn find_available(
        domain_id: DomainId,
        mac_address: [u8; 6],
        unicast_address_list: Vec<Ipv4Addr>,
        multicast_address: Ipv4Addr,
    ) -> DdsResult<Self> {
        let metatraffic_multicast_socket =
            get_multicast_socket(multicast_address, port_builtin_multicast(domain_id as u16))
                .map_err(|e| DdsError::PreconditionNotMet(format!("{}", e)))?;

        let (participant_id, metatraffic_unicast_socket, default_unicast_socket) = (0..)
            .map(
                |participant_id| -> io::Result<(usize, UdpSocket, UdpSocket)> {
                    Ok((
                        participant_id,
                        get_unicast_socket(port_builtin_unicast(
                            domain_id as u16,
                            participant_id as u16,
                        ))?,
                        get_unicast_socket(port_user_unicast(
                            domain_id as u16,
                            participant_id as u16,
                        ))?,
                    ))
                },
            )
            .find(|result| match result {
                Err(e) => e.kind() != ErrorKind::AddrInUse,
                _ => true,
            })
            .unwrap()
            .map_err(|e| DdsError::PreconditionNotMet(format!("{}", e)))?;

        #[rustfmt::skip]
        let guid_prefix = GuidPrefix([
            mac_address[0], mac_address[1], mac_address[2],
            mac_address[3], mac_address[4], mac_address[5],
            domain_id as u8, participant_id as u8, 0, 0, 0, 0
        ]);

        Ok(Communications {
            domain_id,
            participant_id,
            guid_prefix,
            unicast_address_list,
            multicast_address,
            metatraffic_multicast: UdpMulticastTransport::new(metatraffic_multicast_socket),
            metatraffic_unicast: UdpUnicastTransport::new(metatraffic_unicast_socket),
            default_unicast: UdpUnicastTransport::new(default_unicast_socket),
        })
    }

    pub fn metatraffic_multicast_locator_list(&self) -> Vec<Locator> {
        vec![Locator::new(
            LOCATOR_KIND_UDPv4,
            port_builtin_multicast(self.domain_id as u16) as u32,
            locator_from_ipv4(self.multicast_address),
        )]
    }

    pub fn metatraffic_unicast_locator_list(&self) -> Vec<Locator> {
        self.unicast_address_list
            .iter()
            .map(|&address| {
                Locator::new(
                    LOCATOR_KIND_UDPv4,
                    port_builtin_unicast(self.domain_id as u16, self.participant_id as u16) as u32,
                    locator_from_ipv4(address),
                )
            })
            .collect()
    }

    pub fn default_unicast_locator_list(&self) -> Vec<Locator> {
        self.unicast_address_list
            .iter()
            .map(|&address| {
                Locator::new(
                    LOCATOR_KIND_UDPv4,
                    port_user_unicast(self.domain_id as u16, self.participant_id as u16) as u32,
                    locator_from_ipv4(address),
                )
            })
            .collect()
    }
}

struct ParticipantManager {
    participant: DdsShared<DomainParticipantImpl>,
    task_manager: TaskManager,
}

lazy_static! {
    static ref PARTICIPANT_MANAGER_LIST: Mutex<Vec<ParticipantManager>> = Mutex::new(vec![]);
}

pub struct DomainParticipantFactory;

impl DomainParticipantFactory {
    /// This operation creates a new DomainParticipant object. The DomainParticipant signifies that the calling application intends
    /// to join the Domain identified by the domain_id argument.
    /// If the specified QoS policies are not consistent, the operation will fail and no DomainParticipant will be created.
    /// The special value PARTICIPANT_QOS_DEFAULT can be used to indicate that the DomainParticipant should be created
    /// with the default DomainParticipant QoS set in the factory. The use of this value is equivalent to the application obtaining the
    /// default DomainParticipant QoS by means of the operation get_default_participant_qos (2.2.2.2.2.6) and using the resulting
    /// QoS to create the DomainParticipant.
    /// In case of failure, the operation will return a ‘nil’ value (as specified by the platform).
    pub fn create_participant(
        &self,
        domain_id: DomainId,
        qos: Option<DomainParticipantQos>,
        _a_listener: Option<Box<dyn DomainParticipantListener>>,
        _mask: StatusMask,
    ) -> DdsResult<DomainParticipant> {
        let qos = qos.unwrap_or_default();

        let unicast_address_list: Vec<_> = ifcfg::IfCfg::get()
            .expect("Could not scan interfaces")
            .into_iter()
            .flat_map(|i| {
                i.addresses.into_iter().filter_map(|a| match a.address? {
                    SocketAddr::V4(v4) if !v4.ip().is_loopback() => Some(*v4.ip()),
                    _ => None,
                })
            })
            .collect();

        assert!(
            !unicast_address_list.is_empty(),
            "Could not find any IPv4 address"
        );

        let mac_address = ifcfg::IfCfg::get()
            .expect("Could not scan interfaces")
            .into_iter()
            .filter_map(|i| MacAddress::from_str(&i.mac).ok())
            .find(|&mac| mac != MacAddress::new([0, 0, 0, 0, 0, 0]))
            .expect("Could not find any mac address");

        let communications = Communications::find_available(
            domain_id,
            mac_address.bytes(),
            unicast_address_list,
            ipv4_from_locator(&DEFAULT_MULTICAST_LOCATOR_ADDRESS),
        )?;

        let domain_participant = DomainParticipantImpl::new(
            communications.guid_prefix,
            domain_id,
            "".to_string(),
            qos.clone(),
            communications.metatraffic_unicast_locator_list(),
            communications.metatraffic_multicast_locator_list(),
            communications.default_unicast_locator_list(),
            vec![],
        );

        if qos.entity_factory.autoenable_created_entities {
            domain_participant.enable()?;
        }

        domain_participant.create_builtins()?;

        let mut participant_manager = ParticipantManager {
            participant: domain_participant,
            task_manager: TaskManager::new(),
        };

        if qos.entity_factory.autoenable_created_entities {
            self.enable(&mut participant_manager, communications)?;
        }

        let participant_proxy = DomainParticipant::new(participant_manager.participant.downgrade());

        PARTICIPANT_MANAGER_LIST
            .lock()
            .unwrap()
            .push(participant_manager);

        Ok(participant_proxy)
    }

    /// This operation deletes an existing DomainParticipant. This operation can only be invoked if all domain entities belonging to
    /// the participant have already been deleted. Otherwise the error PRECONDITION_NOT_MET is returned.
    /// Possible error codes returned in addition to the standard ones: PRECONDITION_NOT_MET.
    pub fn delete_participant(&self, participant: &DomainParticipant) -> DdsResult<()> {
        let mut participant_list = PARTICIPANT_MANAGER_LIST
            .lock()
            .map_err(|e| DdsError::PreconditionNotMet(e.to_string()))?;

        let index = participant_list
            .iter()
            .position(|pm| DomainParticipant::new(pm.participant.downgrade()).eq(participant))
            .ok_or(DdsError::AlreadyDeleted)?;

        participant_list.remove(index);

        Ok(())
    }

    /// This operation returns the DomainParticipantFactory singleton. The operation is idempotent, that is, it can be called multiple
    /// times without side-effects and it will return the same DomainParticipantFactory instance.
    /// The get_instance operation is a static operation implemented using the syntax of the native language and can therefore not be
    /// expressed in the IDL PSM.
    /// The pre-defined value TheParticipantFactory can also be used as an alias for the singleton factory returned by the operation
    /// get_instance.
    pub fn get_instance() -> Self {
        Self
    }

    /// This operation retrieves a previously created DomainParticipant belonging to specified domain_id. If no such
    /// DomainParticipant exists, the operation will return a ‘nil’ value.
    /// If multiple DomainParticipant entities belonging to that domain_id exist, then the operation will return one of them. It is not
    /// specified which one.
    pub fn lookup_participant(&self, _domain_id: DomainId) -> DdsResult<DomainParticipant> {
        todo!()
    }

    /// This operation sets a default value of the DomainParticipant QoS policies which will be used for newly created
    /// DomainParticipant entities in the case where the QoS policies are defaulted in the create_participant operation.
    /// This operation will check that the resulting policies are self consistent; if they are not, the operation will have no effect and
    /// return INCONSISTENT_POLICY.
    pub fn set_default_participant_qos(&self, _qos: Option<DomainParticipantQos>) {
        todo!()
    }

    /// This operation retrieves the default value of the DomainParticipant QoS, that is, the QoS policies which will be used for
    /// newly created DomainParticipant entities in the case where the QoS policies are defaulted in the create_participant
    /// operation.
    /// The values retrieved get_default_participant_qos will match the set of values specified on the last successful call to
    /// set_default_participant_qos, or else, if the call was never made, the default values listed in the QoS table in 2.2.3,
    /// Supported QoS.
    pub fn get_default_participant_qos(&self) -> DdsResult<DomainParticipantQos> {
        todo!()
    }

    /// This operation sets the value of the DomainParticipantFactory QoS policies. These policies control the behavior of the object
    /// a factory for entities.
    /// Note that despite having QoS, the DomainParticipantFactory is not an Entity.
    /// This operation will check that the resulting policies are self consistent; if they are not, the operation will have no effect and
    /// return INCONSISTENT_POLICY.
    pub fn get_qos(&self) -> DdsResult<DomainParticipantFactoryQos> {
        todo!()
    }

    /// This operation returns the value of the DomainParticipantFactory QoS policies.
    pub fn set_qos(&self, _qos: Option<DomainParticipantFactoryQos>) {
        todo!()
    }
}

impl DomainParticipantFactory {
    fn enable(
        &self,
        participant_manager: &mut ParticipantManager,
        communications: Communications,
    ) -> DdsResult<()> {
        let mut metatraffic_multicast_communication = communications.metatraffic_multicast;
        let mut metatraffic_unicast_communication = communications.metatraffic_unicast;
        let mut default_unicast_communication = communications.default_unicast;

        // //////////// SPDP Communication

        // ////////////// SPDP participant discovery
        {
            let domain_participant = participant_manager.participant.clone();
            participant_manager
                .task_manager
                .spawn_enabled_periodic_task(
                    "builtin multicast communication",
                    move || {
                        domain_participant
                            .receive_built_in_data(&mut metatraffic_multicast_communication);
                    },
                    std::time::Duration::from_millis(500),
                );
        }

        // ////////////// SPDP builtin endpoint configuration
        {
            let domain_participant = participant_manager.participant.clone();

            participant_manager
                .task_manager
                .spawn_enabled_periodic_task(
                    "spdp endpoint configuration",
                    move || match domain_participant.discover_matched_participants() {
                        Ok(()) => (),
                        Err(e) => println!("spdp discovery failed: {:?}", e),
                    },
                    std::time::Duration::from_millis(500),
                );
        }

        // //////////// Unicast Communication
        {
            let domain_participant = participant_manager.participant.clone();
            participant_manager
                .task_manager
                .spawn_enabled_periodic_task(
                    "builtin unicast communication",
                    move || {
                        domain_participant
                            .send_built_in_data(&mut metatraffic_unicast_communication);
                        domain_participant
                            .receive_built_in_data(&mut metatraffic_unicast_communication);
                    },
                    std::time::Duration::from_millis(500),
                );
        }

        // ////////////// SEDP user-defined endpoint configuration
        {
            let domain_participant = participant_manager.participant.clone();

            participant_manager
                .task_manager
                .spawn_enabled_periodic_task(
                    "sedp user endpoint configuration",
                    move || {
                        match domain_participant.discover_matched_writers() {
                            Ok(()) => (),
                            Err(e) => println!("sedp writer discovery failed: {:?}", e),
                        }
                        match domain_participant.discover_matched_readers() {
                            Ok(()) => (),
                            Err(e) => println!("sedp reader discovery failed: {:?}", e),
                        }
                    },
                    std::time::Duration::from_millis(500),
                );
        }

        // //////////// User-defined Communication
        {
            let domain_participant = participant_manager.participant.clone();
            participant_manager
                .task_manager
                .spawn_enabled_periodic_task(
                    "user-defined communication",
                    move || {
                        domain_participant
                            .send_user_defined_data(&mut default_unicast_communication);
                        domain_participant
                            .receive_user_defined_data(&mut default_unicast_communication);
                    },
                    std::time::Duration::from_millis(50),
                );
        }

        // //////////// Announce participant
        let domain_participant = participant_manager.participant.clone();
        participant_manager
            .task_manager
            .spawn_enabled_periodic_task(
                "participant announcement",
                move || match domain_participant.announce_participant() {
                    Ok(_) => (),
                    Err(e) => println!("participant announcement failed: {:?}", e),
                },
                std::time::Duration::from_millis(5000),
            );

        // //////////// Start running tasks
        participant_manager.task_manager.enable_tasks();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        publication::data_writer::DataWriterProxy,
        publication::data_writer_listener::DataWriterListener,
        subscription::{
            data_reader::DataReader, data_reader_listener::DataReaderListener,
            subscriber::Subscriber,
        },
        topic_definition::topic::Topic,
    };

    use super::*;
    use crate::{
        dcps_psm::{
            BuiltInTopicKey, PublicationMatchedStatus, SubscriptionMatchedStatus,
            ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE,
        },
        infrastructure::{
            entity::Entity, qos::DataReaderQos, qos_policy::ReliabilityQosPolicyKind,
        },
    };
    use crate::{
        dds_type::{DdsDeserialize, DdsSerialize, DdsType},
        implementation::{
            data_representation_builtin_endpoints::{
                discovered_reader_data::{DiscoveredReaderData, DCPS_SUBSCRIPTION},
                discovered_topic_data::{DiscoveredTopicData, DCPS_TOPIC},
                discovered_writer_data::{DiscoveredWriterData, DCPS_PUBLICATION},
                spdp_discovered_participant_data::{
                    SpdpDiscoveredParticipantData, DCPS_PARTICIPANT,
                },
            },
            dds_impl::domain_participant_impl::CreateBuiltIns,
        },
    };
    use mockall::mock;
    use rtps_pim::structure::types::{Guid, ENTITYID_PARTICIPANT};

    #[test]
    fn communicaitons_make_different_guids() {
        let comm1 = Communications::find_available(
            0,
            [0; 6],
            vec![[127, 0, 0, 1].into()],
            ipv4_from_locator(&DEFAULT_MULTICAST_LOCATOR_ADDRESS),
        )
        .unwrap();

        let comm2 = Communications::find_available(
            0,
            [0; 6],
            vec![[127, 0, 0, 1].into()],
            ipv4_from_locator(&DEFAULT_MULTICAST_LOCATOR_ADDRESS),
        )
        .unwrap();

        assert_ne!(comm1.guid_prefix, comm2.guid_prefix);
    }

    #[test]
    fn multicast_socket_behaviour() {
        let port = 6000;
        let multicast_ip = [239, 255, 0, 1];
        let multicast_addr = SocketAddr::from((multicast_ip, port));

        let socket1 = get_multicast_socket(multicast_ip.into(), port).unwrap();
        let socket2 = get_multicast_socket(multicast_ip.into(), port).unwrap();
        let socket3 = get_multicast_socket(multicast_ip.into(), port).unwrap();

        socket1.send_to(&[1, 2, 3, 4], multicast_addr).unwrap();

        // Everyone receives the data
        let mut buf = [0; 4];
        let (size, _) = socket1.recv_from(&mut buf).unwrap();
        assert_eq!(4, size);
        let (size, _) = socket2.recv_from(&mut buf).unwrap();
        assert_eq!(4, size);
        let (size, _) = socket3.recv_from(&mut buf).unwrap();
        assert_eq!(4, size);

        // Data is received only once
        assert!(socket1.recv_from(&mut buf).is_err());
        assert!(socket2.recv_from(&mut buf).is_err());
        assert!(socket3.recv_from(&mut buf).is_err());
    }

    #[test]
    fn test_spdp_send_receive() {
        let domain_id = 4;
        let interface_address = [127, 0, 0, 1];
        let multicast_ip = [239, 255, 0, 1];

        // ////////// Create 2 participants

        let mut communications1 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![interface_address.into()],
            multicast_ip.into(),
        )
        .unwrap();
        let participant1 = DomainParticipantImpl::new(
            communications1.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications1.metatraffic_unicast_locator_list(),
            communications1.metatraffic_multicast_locator_list(),
            communications1.default_unicast_locator_list(),
            vec![],
        );
        participant1.enable().unwrap();
        participant1.create_builtins().unwrap();

        let mut communications2 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![interface_address.into()],
            multicast_ip.into(),
        )
        .unwrap();

        let participant2 = DomainParticipantImpl::new(
            communications2.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications2.metatraffic_unicast_locator_list(),
            communications2.metatraffic_multicast_locator_list(),
            communications2.default_unicast_locator_list(),
            vec![],
        );
        participant2.enable().unwrap();
        participant2.create_builtins().unwrap();

        // ////////// Send and receive SPDP data
        {
            participant1.announce_participant().unwrap();
            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_multicast);
        }

        // ////////// Participant 2 receives discovered participant data
        let spdp_discovered_participant_data_sample = {
            let participant2_proxy = DomainParticipant::new(participant2.downgrade());

            let subscriber = Subscriber::new(
                participant2
                    .get_builtin_subscriber()
                    .as_ref()
                    .unwrap()
                    .downgrade(),
            );

            let participant_topic: Topic<SpdpDiscoveredParticipantData> = participant2_proxy
                .lookup_topicdescription(DCPS_PARTICIPANT)
                .unwrap();
            let participant2_builtin_participant_data_reader =
                subscriber.lookup_datareader(&participant_topic).unwrap();

            &participant2_builtin_participant_data_reader
                .read(1, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                .unwrap()[0]
        };

        // ////////// Check that the received data is correct
        {
            assert_eq!(
                BuiltInTopicKey {
                    value: Guid::new(communications1.guid_prefix, ENTITYID_PARTICIPANT).into()
                },
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .dds_participant_data
                    .key,
            );

            assert_eq!(
                domain_id,
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .participant_proxy
                    .domain_id as i32
            );

            assert_eq!(
                communications1.guid_prefix,
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .participant_proxy
                    .guid_prefix
            );

            assert_eq!(
                communications1.metatraffic_unicast_locator_list(),
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .participant_proxy
                    .metatraffic_unicast_locator_list
            );

            assert_eq!(
                communications1.metatraffic_multicast_locator_list(),
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .participant_proxy
                    .metatraffic_multicast_locator_list
            );

            assert_eq!(
                participant1.default_unicast_locator_list(),
                spdp_discovered_participant_data_sample
                    .data
                    .as_ref()
                    .unwrap()
                    .participant_proxy
                    .default_unicast_locator_list
            );
        }
    }

    struct UserData(u8);

    impl DdsType for UserData {
        fn type_name() -> &'static str {
            "UserData"
        }

        fn has_key() -> bool {
            false
        }
    }

    impl<'de> DdsDeserialize<'de> for UserData {
        fn deserialize(buf: &mut &'de [u8]) -> crate::return_type::DdsResult<Self> {
            Ok(UserData(buf[0]))
        }
    }

    impl DdsSerialize for UserData {
        fn serialize<W: std::io::Write, E: crate::dds_type::Endianness>(
            &self,
            mut writer: W,
        ) -> crate::return_type::DdsResult<()> {
            writer
                .write(&[self.0])
                .map(|_| ())
                .map_err(|e| DdsError::PreconditionNotMet(format!("{}", e)))
        }
    }

    #[test]
    fn test_sedp_send_receive() {
        let domain_id = 5;
        let unicast_address = [127, 0, 0, 1];
        let multicast_address = [239, 255, 0, 1];

        // ////////// Create 2 participants

        let mut communications1 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![unicast_address.into()],
            multicast_address.into(),
        )
        .unwrap();

        let participant1 = DomainParticipantImpl::new(
            communications1.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications1.metatraffic_unicast_locator_list(),
            communications1.metatraffic_multicast_locator_list(),
            communications1.default_unicast_locator_list(),
            vec![],
        );
        participant1.enable().unwrap();
        let participant1_proxy = DomainParticipant::new(participant1.downgrade());
        participant1.create_builtins().unwrap();

        let mut communications2 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![[127, 0, 0, 1].into()],
            ipv4_from_locator(&DEFAULT_MULTICAST_LOCATOR_ADDRESS),
        )
        .unwrap();

        let participant2 = DomainParticipantImpl::new(
            communications2.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications2.metatraffic_unicast_locator_list(),
            communications2.metatraffic_multicast_locator_list(),
            communications2.default_unicast_locator_list(),
            vec![],
        );
        participant2.enable().unwrap();
        let participant2_proxy = DomainParticipant::new(participant2.downgrade());
        participant2.create_builtins().unwrap();

        // Match SEDP endpoints
        {
            participant1.announce_participant().unwrap();
            participant2.announce_participant().unwrap();

            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_multicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_multicast);

            participant1.discover_matched_participants().unwrap();
            participant2.discover_matched_participants().unwrap();
        }

        // ////////// Create user endpoints
        let user_publisher = participant1_proxy.create_publisher(None, None, 0).unwrap();
        let user_subscriber = participant1_proxy.create_subscriber(None, None, 0).unwrap();

        let user_topic = participant1_proxy
            .create_topic::<UserData>("UserTopic", None, None, 0)
            .unwrap();
        let _user_writer = user_publisher
            .create_datawriter(&user_topic, None, None, 0)
            .unwrap();
        let _user_reader = user_subscriber
            .create_datareader(&user_topic, None, None, 0)
            .unwrap();

        // ////////// Send and receive SEDP data
        {
            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_unicast);
        }

        // ////////// Check that the received data corresponds to the sent data

        let sedp_topic_publication: Topic<DiscoveredWriterData> = participant2_proxy
            .lookup_topicdescription(DCPS_PUBLICATION)
            .unwrap();
        let sedp_topic_subscription: Topic<DiscoveredReaderData> = participant2_proxy
            .lookup_topicdescription(DCPS_SUBSCRIPTION)
            .unwrap();
        let sedp_topic_topic: Topic<DiscoveredTopicData> = participant2_proxy
            .lookup_topicdescription(DCPS_TOPIC)
            .unwrap();

        let participant2_subscriber = Subscriber::new(
            participant2
                .get_builtin_subscriber()
                .as_ref()
                .unwrap()
                .downgrade(),
        );

        let _participant2_publication_datareader = participant2_subscriber
            .lookup_datareader(&sedp_topic_publication)
            .unwrap();
        let _participant2_subscription_datareader = participant2_subscriber
            .lookup_datareader(&sedp_topic_subscription)
            .unwrap();
        let participant2_topic_datareader = participant2_subscriber
            .lookup_datareader(&sedp_topic_topic)
            .unwrap();

        let discovered_topic_data_sample = &participant2_topic_datareader
            .read(1, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
            .unwrap()[0];
        assert_eq!(
            UserData::type_name(),
            discovered_topic_data_sample
                .data
                .as_ref()
                .unwrap()
                .topic_builtin_topic_data
                .type_name,
        );
        assert_eq!(
            user_topic.get_name().unwrap(),
            discovered_topic_data_sample
                .data
                .as_ref()
                .unwrap()
                .topic_builtin_topic_data
                .name,
        );
    }

    mock! {
        #[derive(Clone)]
        ReaderListener {}

        impl DataReaderListener for ReaderListener {
            type Foo = UserData;
            fn on_subscription_matched(&mut self, the_reader: &DataReader<UserData>, status: SubscriptionMatchedStatus);
            fn on_data_available(&mut self, the_reader: &DataReader<UserData>);
        }
    }

    mock! {
        #[derive(Clone)]
        WriterListener {}

        impl DataWriterListener for WriterListener {
            type Foo = UserData;

            fn on_publication_matched(
                &mut self,
                the_writer: &DataWriterProxy<UserData>,
                status: PublicationMatchedStatus,
            );

            fn on_liveliness_lost(
                &mut self,
                _the_writer: &DataWriterProxy<UserData>,
                _status: crate::dcps_psm::LivelinessLostStatus,
            );

            fn on_offered_deadline_missed(
                &mut self,
                _the_writer: &DataWriterProxy<UserData>,
                _status: crate::dcps_psm::OfferedDeadlineMissedStatus,
            );

            fn on_offered_incompatible_qos(
                &mut self,
                _the_writer: &DataWriterProxy<UserData>,
                _status: crate::dcps_psm::OfferedIncompatibleQosStatus,
            );
        }
    }

    #[test]
    fn test_reader_writer_matching_listener() {
        let domain_id = 6;
        let unicast_address = [127, 0, 0, 1];
        let multicast_address = [239, 255, 0, 1];

        // ////////// Create 2 participants
        let mut communications1 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![unicast_address.into()],
            multicast_address.into(),
        )
        .unwrap();

        let participant1 = DomainParticipantImpl::new(
            communications1.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications1.metatraffic_unicast_locator_list(),
            communications1.metatraffic_multicast_locator_list(),
            communications1.default_unicast_locator_list(),
            vec![],
        );
        participant1.enable().unwrap();
        let participant1_proxy = DomainParticipant::new(participant1.downgrade());
        participant1.create_builtins().unwrap();

        let mut communications2 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![unicast_address.into()],
            multicast_address.into(),
        )
        .unwrap();

        let participant2 = DomainParticipantImpl::new(
            communications2.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications2.metatraffic_unicast_locator_list(),
            communications2.metatraffic_multicast_locator_list(),
            communications2.default_unicast_locator_list(),
            vec![],
        );
        participant2.enable().unwrap();
        let participant2_proxy = DomainParticipant::new(participant2.downgrade());
        participant2.create_builtins().unwrap();

        // ////////// Match SEDP endpoints
        {
            participant1.announce_participant().unwrap();
            participant2.announce_participant().unwrap();

            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_multicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_multicast);

            participant1.discover_matched_participants().unwrap();
            participant2.discover_matched_participants().unwrap();
        }

        // ////////// Write SEDP discovery data
        let user_publisher = participant1_proxy.create_publisher(None, None, 0).unwrap();
        let user_subscriber = participant2_proxy.create_subscriber(None, None, 0).unwrap();

        let user_topic = participant1_proxy
            .create_topic::<UserData>("UserTopic", None, None, 0)
            .unwrap();
        let user_writer = user_publisher
            .create_datawriter(
                &user_topic,
                None,
                Some(Box::new(MockWriterListener::new())),
                0,
            )
            .unwrap();
        let user_reader = user_subscriber
            .create_datareader(
                &user_topic,
                None,
                Some(Box::new(MockReaderListener::new())),
                0,
            )
            .unwrap();

        // ////////// Send SEDP data
        {
            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_unicast);
        }

        // ////////// Process SEDP data

        // Writer listener must be called once on reader discovery
        {
            let mut writer_listener = Box::new(MockWriterListener::new());
            writer_listener
                .expect_on_publication_matched()
                .once()
                .return_const(());
            user_writer.set_listener(Some(writer_listener), 0).unwrap();

            participant1.discover_matched_readers().unwrap();

            user_writer
                .set_listener(Some(Box::new(MockWriterListener::new())), 0)
                .unwrap();
        }

        // Reader listener must be called once on writer discovery
        {
            let mut reader_listener = Box::new(MockReaderListener::new());
            reader_listener
                .expect_on_subscription_matched()
                .once()
                .return_const(());
            user_reader.set_listener(Some(reader_listener), 0).unwrap();

            participant2.discover_matched_writers().unwrap();

            user_reader
                .set_listener(Some(Box::new(MockReaderListener::new())), 0)
                .unwrap();
        }
    }

    #[test]
    fn test_reader_available_data_listener() {
        let domain_id = 7;
        let unicast_address = [127, 0, 0, 1];
        let multicast_address = [239, 255, 0, 1];

        // ////////// Create 2 participants
        let mut communications1 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![unicast_address.into()],
            multicast_address.into(),
        )
        .unwrap();

        let participant1 = DomainParticipantImpl::new(
            communications1.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications1.metatraffic_unicast_locator_list(),
            communications1.metatraffic_multicast_locator_list(),
            communications1.default_unicast_locator_list(),
            vec![],
        );
        participant1.enable().unwrap();
        let participant1_proxy = DomainParticipant::new(participant1.downgrade());
        participant1.create_builtins().unwrap();

        let mut communications2 = Communications::find_available(
            domain_id,
            [0; 6],
            vec![unicast_address.into()],
            multicast_address.into(),
        )
        .unwrap();

        let participant2 = DomainParticipantImpl::new(
            communications2.guid_prefix,
            domain_id,
            "".to_string(),
            DomainParticipantQos::default(),
            communications2.metatraffic_unicast_locator_list(),
            communications2.metatraffic_multicast_locator_list(),
            communications2.default_unicast_locator_list(),
            vec![],
        );
        participant2.enable().unwrap();
        let participant2_proxy = DomainParticipant::new(participant2.downgrade());
        participant2.create_builtins().unwrap();

        // ////////// Match SEDP endpoints
        {
            participant1.announce_participant().unwrap();
            participant2.announce_participant().unwrap();

            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_multicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_multicast);

            participant1.discover_matched_participants().unwrap();
            participant2.discover_matched_participants().unwrap();
        }

        // ////////// Create user endpoints
        let user_publisher = participant1_proxy.create_publisher(None, None, 0).unwrap();
        let user_subscriber = participant2_proxy.create_subscriber(None, None, 0).unwrap();

        let user_topic = participant1_proxy
            .create_topic::<UserData>("UserTopic", None, None, 0)
            .unwrap();
        let user_writer = user_publisher
            .create_datawriter(&user_topic, None, None, 0)
            .unwrap();

        let mut reader_qos = DataReaderQos::default();
        reader_qos.reliability.kind = ReliabilityQosPolicyKind::ReliableReliabilityQos;
        let user_reader = user_subscriber
            .create_datareader(
                &user_topic,
                Some(reader_qos),
                Some(Box::new(MockReaderListener::new())),
                0,
            )
            .unwrap();

        // ////////// Activate SEDP
        {
            participant1.send_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.send_built_in_data(&mut communications2.metatraffic_unicast);

            participant1.receive_built_in_data(&mut communications1.metatraffic_unicast);
            participant2.receive_built_in_data(&mut communications2.metatraffic_unicast);

            // ////////// Process SEDP data
            participant1.discover_matched_readers().unwrap();

            // We expect the subscription matched listener to be called when matching
            let mut reader_listener = Box::new(MockReaderListener::new());
            reader_listener
                .expect_on_subscription_matched()
                .return_const(());
            user_reader.set_listener(Some(reader_listener), 0).unwrap();

            participant2.discover_matched_writers().unwrap();

            // No more listener should be called for now
            user_reader
                .set_listener(Some(Box::new(MockReaderListener::new())), 0)
                .unwrap();
        }

        // ////////// Write user data
        user_writer.write(&UserData(8), None).unwrap();

        // ////////// Send user data
        {
            participant1.send_user_defined_data(&mut communications1.default_unicast);

            // On receive the available data listener should be called
            let mut reader_listener = Box::new(MockReaderListener::new());
            reader_listener
                .expect_on_data_available()
                .once()
                .return_const(());
            user_reader.set_listener(Some(reader_listener), 0).unwrap();

            participant2.receive_user_defined_data(&mut communications2.default_unicast);

            // From now on no listener should be called anymore
            user_reader
                .set_listener(Some(Box::new(MockReaderListener::new())), 0)
                .unwrap();
        }
    }

    fn reset_singleton() {
        PARTICIPANT_MANAGER_LIST.lock().unwrap().clear();
    }

    #[test]
    fn test_delete_participant() {
        reset_singleton();

        let participant_factory = DomainParticipantFactory::get_instance();

        let participant1 = participant_factory
            .create_participant(1, None, None, 0)
            .unwrap();

        let participant2 = participant_factory
            .create_participant(2, None, None, 0)
            .unwrap();

        participant_factory
            .delete_participant(&participant1)
            .unwrap();

        assert_eq!(1, PARTICIPANT_MANAGER_LIST.lock().unwrap().len());

        assert!(participant1.enable().is_err());
        assert!(participant2.enable().is_ok());
    }
}
