use std::collections::HashMap;

use crate::cache::{HistoryCache, ReaderCacheChange, ReaderHistoryCache};
use crate::endpoint::Endpoint;
use crate::parser::{InlineQosParameter, Payload};
use crate::proxy::WriterProxy;
use crate::types::{
    ChangeKind, Duration, EntityId, InlineQosParameterList, LocatorList, ParameterList,
    ReliabilityKind, SequenceNumber, TopicKind, GUID,
};

/// Specialization of RTPS Reader. The RTPS StatefulReader keeps state on each matched RTPS Writer.
/// The state kept on each writer is maintained in the RTPS WriterProxy class.
pub struct StatefulReader<'a> {
    reader: Reader,
    matched_writers: HashMap<GUID, WriterProxy<'a>>,
}

impl<'a> StatefulReader<'a> {
    /// This operation creates a new RTPS StatefulReader. The newly-created stateful reader is initialized with
    /// an empty list of matched writers
    pub fn new(
        endpoint: Endpoint,
        heartbeat_response_delay: Duration,
        heartbeat_suppression_duration: Duration,
        expects_inline_qos: bool,
    ) -> Self {
        StatefulReader {
            reader: Reader::new(
                endpoint,
                heartbeat_response_delay,
                heartbeat_suppression_duration,
                expects_inline_qos,
            ),
            matched_writers: HashMap::new(),
        }
    }

    /// This operation adds the WriterProxy a_writer_proxy to the StatefulReader::matched_writers.
    pub fn matched_writer_add(&mut self, a_writer_proxy: WriterProxy<'a>) {
        self.matched_writers
            .insert(a_writer_proxy.remote_writer_guid(), a_writer_proxy);
    }

    /// This operation removes the WriterProxy a_writer_proxy from the set StatefulReader::matched_writers.
    pub fn matched_writer_remove(&mut self, a_writer_proxy: WriterProxy) {
        self.matched_writers
            .remove(&a_writer_proxy.remote_writer_guid());
    }

    /// This operation finds the WriterProxy with GUID_t a_writer_guid from the set StatefulReader::matched_writers.
    /// If the writer GUID does not exist in the list of matched writers returns None
    pub fn matched_writer_lookup(&self, a_writer_guid: GUID) -> Option<&WriterProxy> {
        self.matched_writers.get(&a_writer_guid)
    }
}

pub struct Reader {
    endpoint: Endpoint,
    pub heartbeat_response_delay: Duration,
    pub heartbeat_suppression_duration: Duration,
    pub reader_cache: ReaderHistoryCache,
    expects_inline_qos: bool,
}

impl Reader {
    pub fn new(
        endpoint: Endpoint,
        heartbeat_response_delay: Duration,
        heartbeat_suppression_duration: Duration,
        expects_inline_qos: bool,
    ) -> Self {
        Reader {
            endpoint,
            heartbeat_response_delay,
            heartbeat_suppression_duration,
            reader_cache: ReaderHistoryCache::new(),
            expects_inline_qos,
        }
    }

    pub fn read_data(
        &self,
        writer_guid: GUID,
        sequence_number: SequenceNumber,
        inline_qos: Option<InlineQosParameterList>,
        serialized_payload: Payload,
    ) {
        println!("Reader is processing data");

        if let Payload::Data(data) = serialized_payload {
            if let Some(inline_qos_list) = inline_qos {
                let key_hash_parameter = inline_qos_list.iter().find(|&x| x.is_key_hash());
                if let Some(InlineQosParameter::KeyHash(instance_handle)) = key_hash_parameter {
                    let rcc = ReaderCacheChange::new(
                        ChangeKind::Alive,
                        writer_guid,
                        *instance_handle,
                        sequence_number,
                        None, /*inline_qos*/
                        Some(data),
                    );
                    self.reader_cache.add_change(rcc);
                }
            }
        } else if let Payload::Key(key) = serialized_payload {
            if let Some(inline_qos_list) = inline_qos {
                let status_info_parameter = inline_qos_list.iter().find(|&x| x.is_status_info());
                if let Some(InlineQosParameter::StatusInfo(status_info)) = status_info_parameter {
                    // TODO: Check the liveliness changes to the entity
                }
            }
        } else {
            // TODO: Either no payload or non standardized payload. In either case, not implemented yet
        }
    }
}
