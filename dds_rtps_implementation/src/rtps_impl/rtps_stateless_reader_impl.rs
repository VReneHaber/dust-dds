use rust_rtps_pim::{
    behavior::{
        reader::{
            reader::RtpsReaderAttributes,
            stateless_reader::{RtpsStatelessReaderAttributes, RtpsStatelessReaderConstructor},
        },
        stateless_reader_behavior::BestEffortStatelessReaderBehavior,
        types::Duration,
    },
    structure::{
        endpoint::RtpsEndpointAttributes,
        entity::RtpsEntityAttributes,
        history_cache::RtpsHistoryCacheConstructor,
        types::{Guid, Locator, ReliabilityKind, TopicKind},
    },
};

use super::rtps_reader_history_cache_impl::ReaderHistoryCache;

pub struct RtpsStatelessReaderImpl {
    guid: Guid,
    topic_kind: TopicKind,
    reliability_level: ReliabilityKind,
    unicast_locator_list: Vec<Locator>,
    multicast_locator_list: Vec<Locator>,
    heartbeat_response_delay: Duration,
    heartbeat_supression_duration: Duration,
    reader_cache: ReaderHistoryCache,
    expects_inline_qos: bool,
}

impl RtpsStatelessReaderConstructor for RtpsStatelessReaderImpl {
    fn new(
        guid: Guid,
        topic_kind: TopicKind,
        reliability_level: ReliabilityKind,
        unicast_locator_list: &[Locator],
        multicast_locator_list: &[Locator],
        heartbeat_response_delay: Duration,
        heartbeat_supression_duration: Duration,
        expects_inline_qos: bool,
    ) -> Self {
        Self {
            guid,
            topic_kind,
            reliability_level,
            unicast_locator_list: unicast_locator_list.to_vec(),
            multicast_locator_list: multicast_locator_list.to_vec(),
            heartbeat_response_delay,
            heartbeat_supression_duration,
            reader_cache: ReaderHistoryCache::new(),
            expects_inline_qos,
        }
    }
}

impl RtpsStatelessReaderAttributes for RtpsStatelessReaderImpl {}

impl RtpsReaderAttributes for RtpsStatelessReaderImpl {
    type ReaderHistoryCacheType = ReaderHistoryCache;

    fn heartbeat_response_delay(&self) -> &Duration {
        &self.heartbeat_response_delay
    }

    fn heartbeat_supression_duration(&self) -> &Duration {
        &self.heartbeat_supression_duration
    }

    fn reader_cache(&self) -> &Self::ReaderHistoryCacheType {
        &self.reader_cache
    }

    fn expects_inline_qos(&self) -> &bool {
        &self.expects_inline_qos
    }
}

impl RtpsEndpointAttributes for RtpsStatelessReaderImpl {
    fn topic_kind(&self) -> &TopicKind {
        &self.topic_kind
    }

    fn reliability_level(&self) -> &ReliabilityKind {
        &self.reliability_level
    }

    fn unicast_locator_list(&self) -> &[Locator] {
        self.unicast_locator_list.as_slice()
    }

    fn multicast_locator_list(&self) -> &[Locator] {
        &self.multicast_locator_list.as_slice()
    }
}

impl RtpsEntityAttributes for RtpsStatelessReaderImpl {
    fn guid(&self) -> &Guid {
        &self.guid
    }
}

impl<'a> IntoIterator for &'a mut RtpsStatelessReaderImpl {
    type Item = BestEffortStatelessReaderBehavior<'a, ReaderHistoryCache>;
    type IntoIter =
        std::option::IntoIter<BestEffortStatelessReaderBehavior<'a, ReaderHistoryCache>>;

    fn into_iter(self) -> Self::IntoIter {
        Some(BestEffortStatelessReaderBehavior {
            reader_guid: &self.guid,
            reader_cache: &mut self.reader_cache,
        })
        .into_iter()
    }
}
