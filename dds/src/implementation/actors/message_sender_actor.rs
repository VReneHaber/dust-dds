use dust_dds_derive::actor_interface;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};

use crate::rtps::{
    messages::overall_structure::{RtpsMessageHeader, RtpsMessageWrite, Submessage},
    types::{
        GuidPrefix, Locator, ProtocolVersion, VendorId, LOCATOR_KIND_UDP_V4, LOCATOR_KIND_UDP_V6,
    },
};

pub struct MessageSenderActor {
    socket: std::net::UdpSocket,
    protocol_version: ProtocolVersion,
    vendor_id: VendorId,
    guid_prefix: GuidPrefix,
}

impl MessageSenderActor {
    pub fn new(
        socket: std::net::UdpSocket,
        protocol_version: ProtocolVersion,
        vendor_id: VendorId,
        guid_prefix: GuidPrefix,
    ) -> Self {
        Self {
            socket,
            protocol_version,
            vendor_id,
            guid_prefix,
        }
    }
}

#[actor_interface]
impl MessageSenderActor {
    pub fn write(
        &self,
        submessages: Vec<Box<dyn Submessage + Send>>,
        destination_locator_list: Vec<Locator>,
    ) {
        let header =
            RtpsMessageHeader::new(self.protocol_version, self.vendor_id, self.guid_prefix);
        let message = RtpsMessageWrite::new(&header, &submessages);
        let buf = message.buffer();

        for destination_locator in destination_locator_list {
            if UdpLocator(destination_locator).is_multicast() {
                let socket2: socket2::Socket = self.socket.try_clone().unwrap().into();
                let interface_addresses = NetworkInterface::show();
                let interface_addresses: Vec<_> = interface_addresses
                    .expect("Could not scan interfaces")
                    .into_iter()
                    .flat_map(|i| {
                        i.addr.into_iter().filter_map(|a| match a {
                            Addr::V4(v4) => Some(v4.ip),
                            _ => None,
                        })
                    })
                    .collect();
                for address in interface_addresses {
                    if socket2.set_multicast_if_v4(&address).is_ok() {
                        self.socket
                            .send_to(buf, UdpLocator(destination_locator))
                            .ok();
                    }
                }
            } else {
                self.socket
                    .send_to(buf, UdpLocator(destination_locator))
                    .ok();
            }
        }
    }
}

struct UdpLocator(Locator);

impl ToSocketAddrs for UdpLocator {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        let locator_address = self.0.address();
        match self.0.kind() {
            LOCATOR_KIND_UDP_V4 => {
                let address = SocketAddrV4::new(
                    Ipv4Addr::new(
                        locator_address[12],
                        locator_address[13],
                        locator_address[14],
                        locator_address[15],
                    ),
                    self.0.port() as u16,
                );
                Ok(Some(SocketAddr::V4(address)).into_iter())
            }
            LOCATOR_KIND_UDP_V6 => todo!(),
            _ => Err(std::io::ErrorKind::InvalidInput.into()),
        }
    }
}

impl UdpLocator {
    fn is_multicast(&self) -> bool {
        let locator_address = self.0.address();
        match self.0.kind() {
            LOCATOR_KIND_UDP_V4 => Ipv4Addr::new(
                locator_address[12],
                locator_address[13],
                locator_address[14],
                locator_address[15],
            )
            .is_multicast(),
            LOCATOR_KIND_UDP_V6 => Ipv6Addr::from(locator_address).is_multicast(),
            _ => false,
        }
    }
}
