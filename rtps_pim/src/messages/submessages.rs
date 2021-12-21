use super::{
    submessage_elements::{
        CountSubmessageElement, EntityIdSubmessageElement, FragmentNumberSetSubmessageElement,
        FragmentNumberSubmessageElement, GuidPrefixSubmessageElement, LocatorListSubmessageElement,
        ParameterListSubmessageElement, ProtocolVersionSubmessageElement,
        SequenceNumberSetSubmessageElement, SequenceNumberSubmessageElement,
        SerializedDataFragmentSubmessageElement, SerializedDataSubmessageElement,
        TimestampSubmessageElement, ULongSubmessageElement, UShortSubmessageElement,
        VendorIdSubmessageElement,
    },
    types::SubmessageFlag,
};

#[derive(Debug, PartialEq)]
pub struct AckNackSubmessage<S> {
    pub endianness_flag: SubmessageFlag,
    pub final_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub reader_sn_state: SequenceNumberSetSubmessageElement<S>,
    pub count: CountSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct DataSubmessage<P, D> {
    pub endianness_flag: SubmessageFlag,
    pub inline_qos_flag: SubmessageFlag,
    pub data_flag: SubmessageFlag,
    pub key_flag: SubmessageFlag,
    pub non_standard_payload_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub writer_sn: SequenceNumberSubmessageElement,
    pub inline_qos: ParameterListSubmessageElement<P>,
    pub serialized_payload: SerializedDataSubmessageElement<D>,
}

pub trait DataSubmessageConstructor {
    type EntityIdType;
    type SequenceNumberType;
    type ParameterListType;
    type SerializedDataType;

    fn new(
        endianness_flag: SubmessageFlag,
        inline_qos_flag: SubmessageFlag,
        data_flag: SubmessageFlag,
        key_flag: SubmessageFlag,
        non_standard_payload_flag: SubmessageFlag,
        reader_id: Self::EntityIdType,
        writer_id: Self::EntityIdType,
        writer_sn: Self::SequenceNumberType,
        inline_qos: Self::ParameterListType,
        serialized_payload: Self::SerializedDataType,
    ) -> Self;
}

pub trait DataSubmessageAttributes {
    type EntityIdSubmessageElementType;
    type SequenceNumberSubmessageElementType;
    type ParameterListSubmessageElementType;
    type SerializedDataSubmessageElementType;

    fn endianness_flag(&self) -> &SubmessageFlag;
    fn inline_qos_flag(&self) -> &SubmessageFlag;
    fn data_flag(&self) -> &SubmessageFlag;
    fn key_flag(&self) -> &SubmessageFlag;
    fn non_standard_payload_flag(&self) -> &SubmessageFlag;
    fn reader_id(&self) -> &Self::EntityIdSubmessageElementType;
    fn writer_id(&self) -> &Self::EntityIdSubmessageElementType;
    fn writer_sn(&self) -> &Self::SequenceNumberSubmessageElementType;
    fn inline_qos(&self) -> &Self::ParameterListSubmessageElementType;
    fn serialized_payload(&self) -> &Self::SerializedDataSubmessageElementType;
}

#[derive(Debug, PartialEq)]
pub struct DataFragSubmessage<P, D> {
    pub endianness_flag: SubmessageFlag,
    pub inline_qos_flag: SubmessageFlag,
    pub non_standard_payload_flag: SubmessageFlag,
    pub key_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub writer_sn: SequenceNumberSubmessageElement,
    pub fragment_starting_num: FragmentNumberSubmessageElement,
    pub fragments_in_submessage: UShortSubmessageElement,
    pub data_size: ULongSubmessageElement,
    pub fragment_size: UShortSubmessageElement,
    pub inline_qos: ParameterListSubmessageElement<P>,
    pub serialized_payload: SerializedDataFragmentSubmessageElement<D>,
}

#[derive(Debug, PartialEq)]
pub struct GapSubmessage<S> {
    pub endianness_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub gap_start: SequenceNumberSubmessageElement,
    pub gap_list: SequenceNumberSetSubmessageElement<S>,
    // gap_start_gsn: submessage_elements::SequenceNumber,
    // gap_end_gsn: submessage_elements::SequenceNumber,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeartbeatSubmessage {
    pub endianness_flag: SubmessageFlag,
    pub final_flag: SubmessageFlag,
    pub liveliness_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub first_sn: SequenceNumberSubmessageElement,
    pub last_sn: SequenceNumberSubmessageElement,
    pub count: CountSubmessageElement,
    // current_gsn: submessage_elements::SequenceNumber,
    // first_gsn: submessage_elements::SequenceNumber,
    // last_gsn: submessage_elements::SequenceNumber,
    // writer_set: submessage_elements::GroupDigest,
    // secure_writer_set: submessage_elements::GroupDigest,
}

#[derive(Debug, PartialEq)]
pub struct HeartbeatFragSubmessage {
    pub endianness_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub writer_sn: SequenceNumberSubmessageElement,
    pub last_fragment_num: FragmentNumberSubmessageElement,
    pub count: CountSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct InfoDestinationSubmessage {
    pub endianness_flag: SubmessageFlag,
    pub guid_prefix: GuidPrefixSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct InfoReplySubmessage<L> {
    pub endianness_flag: SubmessageFlag,
    pub multicast_flag: SubmessageFlag,
    pub unicast_locator_list: LocatorListSubmessageElement<L>,
    pub multicast_locator_list: LocatorListSubmessageElement<L>,
}

#[derive(Debug, PartialEq)]
pub struct InfoSourceSubmessage {
    pub endianness_flag: SubmessageFlag,
    pub protocol_version: ProtocolVersionSubmessageElement,
    pub vendor_id: VendorIdSubmessageElement,
    pub guid_prefix: GuidPrefixSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct InfoTimestampSubmessage {
    pub endianness_flag: SubmessageFlag,
    pub invalidate_flag: SubmessageFlag,
    pub timestamp: TimestampSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct NackFragSubmessage<F> {
    pub endianness_flag: SubmessageFlag,
    pub reader_id: EntityIdSubmessageElement,
    pub writer_id: EntityIdSubmessageElement,
    pub writer_sn: SequenceNumberSubmessageElement,
    pub fragment_number_state: FragmentNumberSetSubmessageElement<F>,
    pub count: CountSubmessageElement,
}

#[derive(Debug, PartialEq)]
pub struct PadSubmessage {}
