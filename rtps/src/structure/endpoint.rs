use crate::types::{Locator, ReliabilityKind, TopicKind};

use super::RTPSEntity;

pub trait RTPSEndpoint: RTPSEntity {
    fn unicast_locator_list(&self) -> &[Locator];
    fn multicast_locator_list(&self) -> &[Locator];
    fn topic_kind(&self) -> TopicKind;
    fn reliability_level(&self) -> ReliabilityKind;
}
