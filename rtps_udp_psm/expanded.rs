#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub mod submessages {
    pub mod ack_nack {
        use crate::submessage_elements::{CountUdp, EntityIdUdp, SequenceNumberSetUdp};
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader, Submessage};
        pub struct AckNackUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for AckNackUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    AckNackUdp {} => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "AckNackUdp");
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for AckNackUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for AckNackUdp {
            #[inline]
            fn eq(&self, other: &AckNackUdp) -> bool {
                match *other {
                    AckNackUdp {} => match *self {
                        AckNackUdp {} => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::AckNackSubmessage for AckNackUdp {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSetSubmessageElementType = SequenceNumberSetUdp;
            type CountSubmessageElementType = CountUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _final_flag: SubmessageFlag,
                _reader_id: EntityIdUdp,
                _writer_id: EntityIdUdp,
                _reader_sn_state: SequenceNumberSetUdp,
                _count: CountUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn final_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn reader_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn reader_sn_state(&self) -> &SequenceNumberSetUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn count(&self) -> &CountUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl Submessage for AckNackUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod data {
        use rust_rtps_pim::messages::{
            submessages::DataSubmessage,
            types::{SubmessageFlag},
            RtpsSubmessageHeader, Submessage,
        };
        use rust_serde_cdr::deserializer::{self, RtpsMessageDeserializer};
        use crate::{
            parameter_list::{ParameterListUdp},
            submessage_elements::{
                EntityIdUdp, SequenceNumberUdp, SerializedDataUdp, flags_to_byte, is_bit_set,
            },
            submessage_header::{DATA, SubmessageHeaderUdp},
        };
        pub struct DataSubmesageUdp<'a> {
            pub(crate) header: SubmessageHeaderUdp,
            extra_flags: u16,
            octets_to_inline_qos: u16,
            reader_id: EntityIdUdp,
            writer_id: EntityIdUdp,
            writer_sn: SequenceNumberUdp,
            inline_qos: ParameterListUdp<'a>,
            serialized_payload: SerializedDataUdp<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'a> ::core::fmt::Debug for DataSubmesageUdp<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DataSubmesageUdp {
                        header: ref __self_0_0,
                        extra_flags: ref __self_0_1,
                        octets_to_inline_qos: ref __self_0_2,
                        reader_id: ref __self_0_3,
                        writer_id: ref __self_0_4,
                        writer_sn: ref __self_0_5,
                        inline_qos: ref __self_0_6,
                        serialized_payload: ref __self_0_7,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "DataSubmesageUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "header",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "extra_flags",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "octets_to_inline_qos",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reader_id",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "writer_id",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "writer_sn",
                            &&(*__self_0_5),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "inline_qos",
                            &&(*__self_0_6),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "serialized_payload",
                            &&(*__self_0_7),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<'a> ::core::marker::StructuralPartialEq for DataSubmesageUdp<'a> {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'a> ::core::cmp::PartialEq for DataSubmesageUdp<'a> {
            #[inline]
            fn eq(&self, other: &DataSubmesageUdp<'a>) -> bool {
                match *other {
                    DataSubmesageUdp {
                        header: ref __self_1_0,
                        extra_flags: ref __self_1_1,
                        octets_to_inline_qos: ref __self_1_2,
                        reader_id: ref __self_1_3,
                        writer_id: ref __self_1_4,
                        writer_sn: ref __self_1_5,
                        inline_qos: ref __self_1_6,
                        serialized_payload: ref __self_1_7,
                    } => match *self {
                        DataSubmesageUdp {
                            header: ref __self_0_0,
                            extra_flags: ref __self_0_1,
                            octets_to_inline_qos: ref __self_0_2,
                            reader_id: ref __self_0_3,
                            writer_id: ref __self_0_4,
                            writer_sn: ref __self_0_5,
                            inline_qos: ref __self_0_6,
                            serialized_payload: ref __self_0_7,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                                && (*__self_0_6) == (*__self_1_6)
                                && (*__self_0_7) == (*__self_1_7)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &DataSubmesageUdp<'a>) -> bool {
                match *other {
                    DataSubmesageUdp {
                        header: ref __self_1_0,
                        extra_flags: ref __self_1_1,
                        octets_to_inline_qos: ref __self_1_2,
                        reader_id: ref __self_1_3,
                        writer_id: ref __self_1_4,
                        writer_sn: ref __self_1_5,
                        inline_qos: ref __self_1_6,
                        serialized_payload: ref __self_1_7,
                    } => match *self {
                        DataSubmesageUdp {
                            header: ref __self_0_0,
                            extra_flags: ref __self_0_1,
                            octets_to_inline_qos: ref __self_0_2,
                            reader_id: ref __self_0_3,
                            writer_id: ref __self_0_4,
                            writer_sn: ref __self_0_5,
                            inline_qos: ref __self_0_6,
                            serialized_payload: ref __self_0_7,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                                || (*__self_0_6) != (*__self_1_6)
                                || (*__self_0_7) != (*__self_1_7)
                        }
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::DataSubmessage<'a> for DataSubmesageUdp<'a> {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type ParameterListSubmessageElementType = ParameterListUdp<'a>;
            type SerializedDataSubmessageElementType = SerializedDataUdp<'a>;
            fn new(
                endianness_flag: SubmessageFlag,
                inline_qos_flag: SubmessageFlag,
                data_flag: SubmessageFlag,
                key_flag: SubmessageFlag,
                non_standard_payload_flag: SubmessageFlag,
                reader_id: Self::EntityIdSubmessageElementType,
                writer_id: Self::EntityIdSubmessageElementType,
                writer_sn: Self::SequenceNumberSubmessageElementType,
                inline_qos: Self::ParameterListSubmessageElementType,
                serialized_payload: Self::SerializedDataSubmessageElementType,
            ) -> Self {
                let flags = flags_to_byte([
                    endianness_flag,
                    inline_qos_flag,
                    data_flag,
                    key_flag,
                    non_standard_payload_flag,
                ]);
                let inline_qos_len = if inline_qos_flag { inline_qos.len() } else { 0 };
                let serialized_payload_len_padded = serialized_payload.len() + 3 & !3;
                let submessage_length = 20 + inline_qos_len + serialized_payload_len_padded;
                let header = SubmessageHeaderUdp {
                    submessage_id: DATA,
                    flags,
                    submessage_length: submessage_length as u16,
                };
                DataSubmesageUdp {
                    header,
                    extra_flags: 0b_0000_0000_0000_0000,
                    octets_to_inline_qos: 16,
                    reader_id,
                    writer_id,
                    writer_sn,
                    inline_qos,
                    serialized_payload,
                }
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 0)
            }
            fn inline_qos_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 1)
            }
            fn data_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 2)
            }
            fn key_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 3)
            }
            fn non_standard_payload_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 4)
            }
            fn reader_id(&self) -> &EntityIdUdp {
                &self.reader_id
            }
            fn writer_id(&self) -> &EntityIdUdp {
                &self.writer_id
            }
            fn writer_sn(&self) -> &SequenceNumberUdp {
                &self.writer_sn
            }
            fn inline_qos(&self) -> &Self::ParameterListSubmessageElementType {
                ::core::panicking::panic("not yet implemented")
            }
            fn serialized_payload(&self) -> &SerializedDataUdp<'a> {
                &self.serialized_payload
            }
        }
        impl<'a> Submessage for DataSubmesageUdp<'a> {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl<'a> serde::Serialize for DataSubmesageUdp<'a> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeStruct;
                let mut len = 6;
                if self.inline_qos_flag() {
                    len += 1
                };
                if self.data_flag() || self.key_flag() {
                    len += 1;
                };
                let mut state = serializer.serialize_struct("Data", len)?;
                state.serialize_field("header", &self.header)?;
                state.serialize_field("extra_flags", &self.extra_flags)?;
                state.serialize_field("octets_to_inline_qos", &self.octets_to_inline_qos)?;
                state.serialize_field("reader_id", &self.reader_id)?;
                state.serialize_field("writer_id", &self.writer_id)?;
                state.serialize_field("writer_sn", &self.writer_sn)?;
                if self.inline_qos_flag() {
                    state.serialize_field("inline_qos", &self.inline_qos)?;
                }
                if self.data_flag() || self.key_flag() {
                    state.serialize_field("serialized_payload", &self.serialized_payload)?;
                    let pad_length = (4 - self.serialized_payload.len() % 4) & 0b11;
                    let pad = ::alloc::vec::from_elem(0, pad_length as usize);
                    state.serialize_field("pad", &SerializedDataUdp(pad.as_slice()))?;
                }
                state.end()
            }
        }
        struct DataSubmesageVisitor<'a>(std::marker::PhantomData<&'a ()>);
        impl<'a, 'de: 'a> serde::de::Visitor<'de> for DataSubmesageVisitor<'a> {
            type Value = DataSubmesageUdp<'a>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataSubmesage")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let header: SubmessageHeaderUdp = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let inline_qos_flag = is_bit_set(header.flags, 1);
                let data_flag = is_bit_set(header.flags, 2);
                let key_flag = is_bit_set(header.flags, 3);
                let extra_flags: u16 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let octets_to_inline_qos: u16 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let reader_id: EntityIdUdp = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                let writer_id: EntityIdUdp = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;
                let writer_sn: SequenceNumberUdp = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(5, &self))?;
                let remaining_data: &[u8] = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(7, &self))?;
                let mut deserializer = RtpsMessageDeserializer {
                    reader: remaining_data,
                };
                let inline_qos: ParameterListUdp = if inline_qos_flag {
                    serde::de::Deserialize::deserialize(&mut deserializer).unwrap()
                } else {
                    ParameterListUdp {
                        parameter: ::alloc::vec::Vec::new(),
                    }
                };
                let inline_qos_len = if inline_qos_flag { inline_qos.len() } else { 0 };
                let serialized_payload: SerializedDataUdp = if data_flag || key_flag {
                    SerializedDataUdp(&remaining_data[inline_qos_len..])
                } else {
                    SerializedDataUdp(&[])
                };
                Ok(DataSubmesageUdp {
                    header,
                    extra_flags,
                    octets_to_inline_qos,
                    reader_id,
                    writer_id,
                    writer_sn,
                    inline_qos,
                    serialized_payload,
                })
            }
        }
        impl<'a, 'de: 'a> serde::Deserialize<'de> for DataSubmesageUdp<'a> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &'static [&'static str] = &[
                    "header",
                    "extra_flags",
                    "octets_to_inline_qos",
                    "reader_id",
                    "writer_id",
                    "writer_sn",
                    "inline_qos",
                    "serialized_payload",
                ];
                deserializer.deserialize_struct(
                    "DataSubmesage",
                    FIELDS,
                    DataSubmesageVisitor(std::marker::PhantomData),
                )
            }
        }
    }
    pub mod data_frag {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::{
            parameter_list::ParameterListUdp,
            submessage_elements::{
                EntityIdUdp, FragmentNumberUdp, SequenceNumberUdp, SerializedDataUdp, ULongUdp,
                UShortUdp,
            },
        };
        pub struct DataFragUdp<'a> {
            pub serialized_data: SerializedDataUdp<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'a> ::core::fmt::Debug for DataFragUdp<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DataFragUdp {
                        serialized_data: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "DataFragUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "serialized_data",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<'a> ::core::marker::StructuralPartialEq for DataFragUdp<'a> {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<'a> ::core::cmp::PartialEq for DataFragUdp<'a> {
            #[inline]
            fn eq(&self, other: &DataFragUdp<'a>) -> bool {
                match *other {
                    DataFragUdp {
                        serialized_data: ref __self_1_0,
                    } => match *self {
                        DataFragUdp {
                            serialized_data: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &DataFragUdp<'a>) -> bool {
                match *other {
                    DataFragUdp {
                        serialized_data: ref __self_1_0,
                    } => match *self {
                        DataFragUdp {
                            serialized_data: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::DataFragSubmessage for DataFragUdp<'a> {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type FragmentNumberSubmessageElementType = FragmentNumberUdp;
            type UShortSubmessageElementType = UShortUdp;
            type ULongSubmessageElementType = ULongUdp;
            type ParameterListSubmessageElementType = ParameterListUdp<'a>;
            type SerializedDataFragmentSubmessageElementType = SerializedDataUdp<'a>;
            fn new(
                _endianness_flag: SubmessageFlag,
                _inline_qos_flag: SubmessageFlag,
                _non_standard_payload_flag: SubmessageFlag,
                _key_flag: SubmessageFlag,
                _reader_id: Self::EntityIdSubmessageElementType,
                _writer_id: Self::EntityIdSubmessageElementType,
                _writer_sn: Self::SequenceNumberSubmessageElementType,
                _fragment_starting_num: Self::FragmentNumberSubmessageElementType,
                _fragments_in_submessage: Self::UShortSubmessageElementType,
                _data_size: Self::ULongSubmessageElementType,
                _fragment_size: Self::UShortSubmessageElementType,
                _inline_qos: Self::ParameterListSubmessageElementType,
                _serialized_payload: Self::SerializedDataFragmentSubmessageElementType,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn inline_qos_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn non_standard_payload_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn key_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn reader_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_sn(&self) -> &SequenceNumberUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn fragment_starting_num(&self) -> &FragmentNumberUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn fragments_in_submessage(&self) -> &UShortUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn data_size(&self) -> &ULongUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn fragment_size(&self) -> &UShortUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn inline_qos(&self) -> &Self::ParameterListSubmessageElementType {
                ::core::panicking::panic("not yet implemented")
            }
            fn serialized_payload(&self) -> &SerializedDataUdp<'a> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl<'a> rust_rtps_pim::messages::Submessage for DataFragUdp<'a> {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod gap {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::{
            submessage_elements::{
                flags_to_byte, is_bit_set, EntityIdUdp, SequenceNumberSetUdp, SequenceNumberUdp,
            },
            submessage_header::{SubmessageHeaderUdp, GAP},
        };
        pub struct GapSubmessageUdp {
            pub header: SubmessageHeaderUdp,
            reader_id: EntityIdUdp,
            writer_id: EntityIdUdp,
            gap_start: SequenceNumberUdp,
            gap_list: SequenceNumberSetUdp,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for GapSubmessageUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    GapSubmessageUdp {
                        header: ref __self_0_0,
                        reader_id: ref __self_0_1,
                        writer_id: ref __self_0_2,
                        gap_start: ref __self_0_3,
                        gap_list: ref __self_0_4,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "GapSubmessageUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "header",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reader_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "writer_id",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "gap_start",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "gap_list",
                            &&(*__self_0_4),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for GapSubmessageUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for GapSubmessageUdp {
            #[inline]
            fn eq(&self, other: &GapSubmessageUdp) -> bool {
                match *other {
                    GapSubmessageUdp {
                        header: ref __self_1_0,
                        reader_id: ref __self_1_1,
                        writer_id: ref __self_1_2,
                        gap_start: ref __self_1_3,
                        gap_list: ref __self_1_4,
                    } => match *self {
                        GapSubmessageUdp {
                            header: ref __self_0_0,
                            reader_id: ref __self_0_1,
                            writer_id: ref __self_0_2,
                            gap_start: ref __self_0_3,
                            gap_list: ref __self_0_4,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &GapSubmessageUdp) -> bool {
                match *other {
                    GapSubmessageUdp {
                        header: ref __self_1_0,
                        reader_id: ref __self_1_1,
                        writer_id: ref __self_1_2,
                        gap_start: ref __self_1_3,
                        gap_list: ref __self_1_4,
                    } => match *self {
                        GapSubmessageUdp {
                            header: ref __self_0_0,
                            reader_id: ref __self_0_1,
                            writer_id: ref __self_0_2,
                            gap_start: ref __self_0_3,
                            gap_list: ref __self_0_4,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                        }
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for GapSubmessageUdp {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "GapSubmessageUdp",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "header",
                        &self.header,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "reader_id",
                        &self.reader_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "writer_id",
                        &self.writer_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gap_start",
                        &self.gap_start,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gap_list",
                        &self.gap_list,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for GapSubmessageUdp {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "header" => _serde::__private::Ok(__Field::__field0),
                                "reader_id" => _serde::__private::Ok(__Field::__field1),
                                "writer_id" => _serde::__private::Ok(__Field::__field2),
                                "gap_start" => _serde::__private::Ok(__Field::__field3),
                                "gap_list" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"header" => _serde::__private::Ok(__Field::__field0),
                                b"reader_id" => _serde::__private::Ok(__Field::__field1),
                                b"writer_id" => _serde::__private::Ok(__Field::__field2),
                                b"gap_start" => _serde::__private::Ok(__Field::__field3),
                                b"gap_list" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<GapSubmessageUdp>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GapSubmessageUdp;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GapSubmessageUdp",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                SubmessageHeaderUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GapSubmessageUdp with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                EntityIdUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GapSubmessageUdp with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                EntityIdUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GapSubmessageUdp with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                SequenceNumberUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct GapSubmessageUdp with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                SequenceNumberSetUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct GapSubmessageUdp with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GapSubmessageUdp {
                                header: __field0,
                                reader_id: __field1,
                                writer_id: __field2,
                                gap_start: __field3,
                                gap_list: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<SubmessageHeaderUdp> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<EntityIdUdp> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<EntityIdUdp> =
                                _serde::__private::None;
                            let mut __field3: _serde::__private::Option<SequenceNumberUdp> =
                                _serde::__private::None;
                            let mut __field4: _serde::__private::Option<SequenceNumberSetUdp> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "header",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                SubmessageHeaderUdp,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "reader_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<EntityIdUdp>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "writer_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<EntityIdUdp>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gap_start",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                SequenceNumberUdp,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gap_list",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                SequenceNumberSetUdp,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("header") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("reader_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("writer_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gap_start") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gap_list") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(GapSubmessageUdp {
                                header: __field0,
                                reader_id: __field1,
                                writer_id: __field2,
                                gap_start: __field3,
                                gap_list: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["header", "reader_id", "writer_id", "gap_start", "gap_list"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GapSubmessageUdp",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GapSubmessageUdp>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl rust_rtps_pim::messages::submessages::GapSubmessage for GapSubmessageUdp {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type SequenceNumberSetSubmessageElementType = SequenceNumberSetUdp;
            fn new(
                endianness_flag: SubmessageFlag,
                reader_id: EntityIdUdp,
                writer_id: EntityIdUdp,
                gap_start: SequenceNumberUdp,
                gap_list: SequenceNumberSetUdp,
            ) -> Self {
                let flags = flags_to_byte([endianness_flag]);
                let submessage_length = 16 + gap_list.len();
                let header = SubmessageHeaderUdp {
                    submessage_id: GAP,
                    flags,
                    submessage_length,
                };
                Self {
                    header,
                    reader_id,
                    writer_id,
                    gap_start,
                    gap_list,
                }
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 0)
            }
            fn reader_id(&self) -> &EntityIdUdp {
                &self.reader_id
            }
            fn writer_id(&self) -> &EntityIdUdp {
                &self.writer_id
            }
            fn gap_start(&self) -> &SequenceNumberUdp {
                &self.gap_start
            }
            fn gap_list(&self) -> &SequenceNumberSetUdp {
                &self.gap_list
            }
        }
        impl rust_rtps_pim::messages::Submessage for GapSubmessageUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod heartbeat {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::{
            submessage_elements::{
                flags_to_byte, is_bit_set, CountUdp, EntityIdUdp, SequenceNumberUdp,
            },
            submessage_header::{SubmessageHeaderUdp, HEARTBEAT},
        };
        pub struct HeartbeatSubmessageUdp {
            pub header: SubmessageHeaderUdp,
            reader_id: EntityIdUdp,
            writer_id: EntityIdUdp,
            first_sn: SequenceNumberUdp,
            last_sn: SequenceNumberUdp,
            count: CountUdp,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for HeartbeatSubmessageUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    HeartbeatSubmessageUdp {
                        header: ref __self_0_0,
                        reader_id: ref __self_0_1,
                        writer_id: ref __self_0_2,
                        first_sn: ref __self_0_3,
                        last_sn: ref __self_0_4,
                        count: ref __self_0_5,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "HeartbeatSubmessageUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "header",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reader_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "writer_id",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "first_sn",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "last_sn",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "count",
                            &&(*__self_0_5),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for HeartbeatSubmessageUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for HeartbeatSubmessageUdp {
            #[inline]
            fn eq(&self, other: &HeartbeatSubmessageUdp) -> bool {
                match *other {
                    HeartbeatSubmessageUdp {
                        header: ref __self_1_0,
                        reader_id: ref __self_1_1,
                        writer_id: ref __self_1_2,
                        first_sn: ref __self_1_3,
                        last_sn: ref __self_1_4,
                        count: ref __self_1_5,
                    } => match *self {
                        HeartbeatSubmessageUdp {
                            header: ref __self_0_0,
                            reader_id: ref __self_0_1,
                            writer_id: ref __self_0_2,
                            first_sn: ref __self_0_3,
                            last_sn: ref __self_0_4,
                            count: ref __self_0_5,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &HeartbeatSubmessageUdp) -> bool {
                match *other {
                    HeartbeatSubmessageUdp {
                        header: ref __self_1_0,
                        reader_id: ref __self_1_1,
                        writer_id: ref __self_1_2,
                        first_sn: ref __self_1_3,
                        last_sn: ref __self_1_4,
                        count: ref __self_1_5,
                    } => match *self {
                        HeartbeatSubmessageUdp {
                            header: ref __self_0_0,
                            reader_id: ref __self_0_1,
                            writer_id: ref __self_0_2,
                            first_sn: ref __self_0_3,
                            last_sn: ref __self_0_4,
                            count: ref __self_0_5,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                        }
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for HeartbeatSubmessageUdp {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "HeartbeatSubmessageUdp",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "header",
                        &self.header,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "reader_id",
                        &self.reader_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "writer_id",
                        &self.writer_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "first_sn",
                        &self.first_sn,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "last_sn",
                        &self.last_sn,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "count",
                        &self.count,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl<'a> rust_rtps_pim::messages::submessages::HeartbeatSubmessage for HeartbeatSubmessageUdp {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type CountSubmessageElementType = CountUdp;
            fn new(
                endianness_flag: SubmessageFlag,
                final_flag: SubmessageFlag,
                liveliness_flag: SubmessageFlag,
                reader_id: EntityIdUdp,
                writer_id: EntityIdUdp,
                first_sn: SequenceNumberUdp,
                last_sn: SequenceNumberUdp,
                count: CountUdp,
            ) -> Self {
                let flags = flags_to_byte([endianness_flag, final_flag, liveliness_flag]);
                let submessage_length = 28;
                let header = SubmessageHeaderUdp {
                    submessage_id: HEARTBEAT,
                    flags,
                    submessage_length,
                };
                Self {
                    header,
                    reader_id,
                    writer_id,
                    first_sn,
                    last_sn,
                    count,
                }
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 0)
            }
            fn final_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 1)
            }
            fn liveliness_flag(&self) -> SubmessageFlag {
                is_bit_set(self.header.flags, 2)
            }
            fn reader_id(&self) -> &EntityIdUdp {
                &self.reader_id
            }
            fn writer_id(&self) -> &EntityIdUdp {
                &self.writer_id
            }
            fn first_sn(&self) -> &SequenceNumberUdp {
                &self.first_sn
            }
            fn last_sn(&self) -> &SequenceNumberUdp {
                &self.last_sn
            }
            fn count(&self) -> &CountUdp {
                &self.count
            }
        }
        impl rust_rtps_pim::messages::Submessage for HeartbeatSubmessageUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod heartbeat_frag {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::submessage_elements::{CountUdp, EntityIdUdp, FragmentNumberUdp, SequenceNumberUdp};
        pub struct HeartbeatFragUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for HeartbeatFragUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    HeartbeatFragUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "HeartbeatFragUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for HeartbeatFragUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for HeartbeatFragUdp {
            #[inline]
            fn eq(&self, other: &HeartbeatFragUdp) -> bool {
                match *other {
                    HeartbeatFragUdp => match *self {
                        HeartbeatFragUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::HeartbeatFragSubmessage for HeartbeatFragUdp {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type FragmentNumberSubmessageElementType = FragmentNumberUdp;
            type CountSubmessageElementType = CountUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _reader_id: EntityIdUdp,
                _writer_id: EntityIdUdp,
                _writer_sn: SequenceNumberUdp,
                _last_fragment_num: FragmentNumberUdp,
                _count: CountUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn reader_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_sn(&self) -> &SequenceNumberUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn last_fragment_num(&self) -> &FragmentNumberUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn count(&self) -> &CountUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for HeartbeatFragUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod info_destination {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::submessage_elements::GuidPrefixUdp;
        pub struct InfoDestinationUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InfoDestinationUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InfoDestinationUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InfoDestinationUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InfoDestinationUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InfoDestinationUdp {
            #[inline]
            fn eq(&self, other: &InfoDestinationUdp) -> bool {
                match *other {
                    InfoDestinationUdp => match *self {
                        InfoDestinationUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::InfoDestinationSubmessage for InfoDestinationUdp {
            type GuidPrefixSubmessageElementType = GuidPrefixUdp;
            fn new(_endianness_flag: SubmessageFlag, _guid_prefix: GuidPrefixUdp) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn guid_prefix(&self) -> &GuidPrefixUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for InfoDestinationUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod info_reply {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::submessage_elements::LocatorListUdp;
        pub struct InfoReplyUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InfoReplyUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InfoReplyUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InfoReplyUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InfoReplyUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InfoReplyUdp {
            #[inline]
            fn eq(&self, other: &InfoReplyUdp) -> bool {
                match *other {
                    InfoReplyUdp => match *self {
                        InfoReplyUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::InfoReplySubmessage for InfoReplyUdp {
            type LocatorListSubmessageElementType = LocatorListUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _multicast_flag: SubmessageFlag,
                _unicast_locator_list: LocatorListUdp,
                _multicast_locator_list: LocatorListUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn multicast_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn unicast_locator_list(&self) -> &LocatorListUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn multicast_locator_list(&self) -> &LocatorListUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for InfoReplyUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod info_source {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::submessage_elements::{GuidPrefixUdp, ProtocolVersionUdp, VendorIdUdp};
        pub struct InfoSourceUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InfoSourceUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InfoSourceUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InfoSourceUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InfoSourceUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InfoSourceUdp {
            #[inline]
            fn eq(&self, other: &InfoSourceUdp) -> bool {
                match *other {
                    InfoSourceUdp => match *self {
                        InfoSourceUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::InfoSourceSubmessage for InfoSourceUdp {
            type ProtocolVersionSubmessageElementType = ProtocolVersionUdp;
            type VendorIdSubmessageElementType = VendorIdUdp;
            type GuidPrefixSubmessageElementType = GuidPrefixUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _protocol_version: ProtocolVersionUdp,
                _vendor_id: VendorIdUdp,
                _guid_prefix: GuidPrefixUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn protocol_version(&self) -> &ProtocolVersionUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn vendor_id(&self) -> &VendorIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn guid_prefix(&self) -> &GuidPrefixUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for InfoSourceUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod info_timestamp {
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        use crate::submessage_elements::TimeUdp;
        pub struct InfoTimestampUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InfoTimestampUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InfoTimestampUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InfoTimestampUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InfoTimestampUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InfoTimestampUdp {
            #[inline]
            fn eq(&self, other: &InfoTimestampUdp) -> bool {
                match *other {
                    InfoTimestampUdp => match *self {
                        InfoTimestampUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::InfoTimestampSubmessage for InfoTimestampUdp {
            type TimestampSubmessageElementType = TimeUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _invalidate_flag: SubmessageFlag,
                _timestamp: TimeUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn invalidate_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn timestamp(&self) -> &TimeUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for InfoTimestampUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod nack_frag {
        use crate::submessage_elements::{
            CountUdp, EntityIdUdp, FragmentNumberSetUdp, SequenceNumberUdp,
        };
        use rust_rtps_pim::messages::{types::SubmessageFlag, RtpsSubmessageHeader};
        pub struct NackFragUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for NackFragUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    NackFragUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "NackFragUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for NackFragUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for NackFragUdp {
            #[inline]
            fn eq(&self, other: &NackFragUdp) -> bool {
                match *other {
                    NackFragUdp => match *self {
                        NackFragUdp => true,
                    },
                }
            }
        }
        impl<'a> rust_rtps_pim::messages::submessages::NackFragSubmessage for NackFragUdp {
            type EntityIdSubmessageElementType = EntityIdUdp;
            type SequenceNumberSubmessageElementType = SequenceNumberUdp;
            type FragmentNumberSetSubmessageElementType = FragmentNumberSetUdp;
            type CountSubmessageElementType = CountUdp;
            fn new(
                _endianness_flag: SubmessageFlag,
                _reader_id: EntityIdUdp,
                _writer_id: EntityIdUdp,
                _writer_sn: SequenceNumberUdp,
                _fragment_number_state: FragmentNumberSetUdp,
                _count: CountUdp,
            ) -> Self {
                ::core::panicking::panic("not yet implemented")
            }
            fn endianness_flag(&self) -> SubmessageFlag {
                ::core::panicking::panic("not yet implemented")
            }
            fn reader_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_id(&self) -> &EntityIdUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn writer_sn(&self) -> &SequenceNumberUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn fragment_number_state(&self) -> &FragmentNumberSetUdp {
                ::core::panicking::panic("not yet implemented")
            }
            fn count(&self) -> &CountUdp {
                ::core::panicking::panic("not yet implemented")
            }
        }
        impl rust_rtps_pim::messages::Submessage for NackFragUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    pub mod pad {
        use rust_rtps_pim::messages::RtpsSubmessageHeader;
        pub struct PadUdp;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for PadUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    PadUdp => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "PadUdp");
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for PadUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for PadUdp {
            #[inline]
            fn eq(&self, other: &PadUdp) -> bool {
                match *other {
                    PadUdp => match *self {
                        PadUdp => true,
                    },
                }
            }
        }
        impl rust_rtps_pim::messages::submessages::PadSubmessage for PadUdp {}
        impl rust_rtps_pim::messages::Submessage for PadUdp {
            fn submessage_header(&self) -> RtpsSubmessageHeader {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
}
pub mod psm {
    use rust_rtps_pim::messages::submessages::RtpsSubmessagePIM;
    use crate::submessages::{
        ack_nack::AckNackUdp, data::DataSubmesageUdp, data_frag::DataFragUdp,
        gap::GapSubmessageUdp, heartbeat::HeartbeatSubmessageUdp, heartbeat_frag::HeartbeatFragUdp,
        info_destination::InfoDestinationUdp, info_reply::InfoReplyUdp, info_source::InfoSourceUdp,
        info_timestamp::InfoTimestampUdp, nack_frag::NackFragUdp, pad::PadUdp,
    };
    pub struct RtpsUdpPsm;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RtpsUdpPsm {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RtpsUdpPsm => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "RtpsUdpPsm");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for RtpsUdpPsm {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for RtpsUdpPsm {
        #[inline]
        fn eq(&self, other: &RtpsUdpPsm) -> bool {
            match *other {
                RtpsUdpPsm => match *self {
                    RtpsUdpPsm => true,
                },
            }
        }
    }
    impl<'a> RtpsSubmessagePIM<'a> for RtpsUdpPsm {
        type AckNackSubmessageType = AckNackUdp;
        type DataSubmessageType = DataSubmesageUdp<'a>;
        type DataFragSubmessageType = DataFragUdp<'a>;
        type GapSubmessageType = GapSubmessageUdp;
        type HeartbeatSubmessageType = HeartbeatSubmessageUdp;
        type HeartbeatFragSubmessageType = HeartbeatFragUdp;
        type InfoDestinationSubmessageType = InfoDestinationUdp;
        type InfoReplySubmessageType = InfoReplyUdp;
        type InfoSourceSubmessageType = InfoSourceUdp;
        type InfoTimestampSubmessageType = InfoTimestampUdp;
        type NackFragSubmessageType = NackFragUdp;
        type PadSubmessageType = PadUdp;
    }
}
pub mod submessage_elements {
    use rust_rtps_pim::{
        messages::types::{Count, FragmentNumber, SubmessageFlag, Time},
        structure::types::{EntityKind, Locator, ProtocolVersion},
    };
    use serde::ser::SerializeStruct;
    pub fn is_bit_set(value: u8, index: usize) -> bool {
        value & (0b_0000_0001 << index) != 0
    }
    pub fn flags_to_byte<const N: usize>(value: [SubmessageFlag; N]) -> u8 {
        let mut flags = 0b_0000_0000_u8;
        for (i, &item) in value.iter().enumerate() {
            if item {
                flags |= 0b_0000_0001 << i
            }
        }
        flags
    }
    pub struct UShortUdp(pub(crate) u16);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for UShortUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                UShortUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "UShortUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for UShortUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for UShortUdp {
        #[inline]
        fn eq(&self, other: &UShortUdp) -> bool {
            match *other {
                UShortUdp(ref __self_1_0) => match *self {
                    UShortUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UShortUdp) -> bool {
            match *other {
                UShortUdp(ref __self_1_0) => match *self {
                    UShortUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl rust_rtps_pim::messages::submessage_elements::UShortSubmessageElementType for UShortUdp {
        fn new(value: &u16) -> Self {
            Self(*value)
        }
        fn value(&self) -> u16 {
            self.0
        }
    }
    pub struct LongUdp(pub(crate) i32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LongUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LongUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "LongUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for LongUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for LongUdp {
        #[inline]
        fn eq(&self, other: &LongUdp) -> bool {
            match *other {
                LongUdp(ref __self_1_0) => match *self {
                    LongUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LongUdp) -> bool {
            match *other {
                LongUdp(ref __self_1_0) => match *self {
                    LongUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for LongUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "LongUdp", &self.0)
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::LongSubmessageElementType for LongUdp {
        fn new(value: &i32) -> Self {
            Self(*value)
        }
        fn value(&self) -> i32 {
            self.0
        }
    }
    pub struct ULongUdp(pub(crate) u32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ULongUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ULongUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ULongUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ULongUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ULongUdp {
        #[inline]
        fn eq(&self, other: &ULongUdp) -> bool {
            match *other {
                ULongUdp(ref __self_1_0) => match *self {
                    ULongUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ULongUdp) -> bool {
            match *other {
                ULongUdp(ref __self_1_0) => match *self {
                    ULongUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ULongUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "ULongUdp", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ULongUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ULongUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ULongUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ULongUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: u32 = match <u32 as _serde::Deserialize>::deserialize(__e) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(ULongUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct ULongUdp with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ULongUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ULongUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ULongUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::ULongSubmessageElementType for ULongUdp {
        fn new(value: &u32) -> Self {
            Self(*value)
        }
        fn value(&self) -> u32 {
            self.0
        }
    }
    pub struct GuidPrefixUdp(pub(crate) [u8; 12]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for GuidPrefixUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                GuidPrefixUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "GuidPrefixUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for GuidPrefixUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for GuidPrefixUdp {
        #[inline]
        fn eq(&self, other: &GuidPrefixUdp) -> bool {
            match *other {
                GuidPrefixUdp(ref __self_1_0) => match *self {
                    GuidPrefixUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &GuidPrefixUdp) -> bool {
            match *other {
                GuidPrefixUdp(ref __self_1_0) => match *self {
                    GuidPrefixUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for GuidPrefixUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "GuidPrefixUdp", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for GuidPrefixUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GuidPrefixUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GuidPrefixUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct GuidPrefixUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: [u8; 12] =
                            match <[u8; 12] as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(GuidPrefixUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<[u8; 12]>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct GuidPrefixUdp with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(GuidPrefixUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "GuidPrefixUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GuidPrefixUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::GuidPrefixSubmessageElementType
        for GuidPrefixUdp
    {
        fn new(value: &rust_rtps_pim::structure::types::GuidPrefix) -> Self {
            Self(value.clone())
        }
        fn value(&self) -> rust_rtps_pim::structure::types::GuidPrefix {
            self.0
        }
    }
    pub struct EntityIdUdp {
        pub entity_key: [u8; 3],
        pub entity_kind: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for EntityIdUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                EntityIdUdp {
                    entity_key: ref __self_0_0,
                    entity_kind: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "EntityIdUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "entity_key",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "entity_kind",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for EntityIdUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for EntityIdUdp {
        #[inline]
        fn eq(&self, other: &EntityIdUdp) -> bool {
            match *other {
                EntityIdUdp {
                    entity_key: ref __self_1_0,
                    entity_kind: ref __self_1_1,
                } => match *self {
                    EntityIdUdp {
                        entity_key: ref __self_0_0,
                        entity_kind: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &EntityIdUdp) -> bool {
            match *other {
                EntityIdUdp {
                    entity_key: ref __self_1_0,
                    entity_kind: ref __self_1_1,
                } => match *self {
                    EntityIdUdp {
                        entity_key: ref __self_0_0,
                        entity_kind: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EntityIdUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EntityIdUdp",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "entity_key",
                    &self.entity_key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "entity_kind",
                    &self.entity_kind,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EntityIdUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "entity_key" => _serde::__private::Ok(__Field::__field0),
                            "entity_kind" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"entity_key" => _serde::__private::Ok(__Field::__field0),
                            b"entity_kind" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EntityIdUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EntityIdUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct EntityIdUdp")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<[u8; 3]>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct EntityIdUdp with 2 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct EntityIdUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(EntityIdUdp {
                            entity_key: __field0,
                            entity_kind: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<[u8; 3]> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u8> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "entity_key",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<[u8; 3]>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "entity_kind",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("entity_key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("entity_kind") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EntityIdUdp {
                            entity_key: __field0,
                            entity_kind: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["entity_key", "entity_kind"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EntityIdUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EntityIdUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::EntityIdSubmessageElementType for EntityIdUdp {
        fn new(value: &rust_rtps_pim::structure::types::EntityId) -> Self {
            Self {
                entity_key: [
                    value.entity_key[0],
                    value.entity_key[1],
                    value.entity_key[2],
                ],
                entity_kind: entity_kind_into_u8(value.entity_kind),
            }
        }
        fn value(&self) -> rust_rtps_pim::structure::types::EntityId {
            rust_rtps_pim::structure::types::EntityId {
                entity_key: [self.entity_key[0], self.entity_key[1], self.entity_key[2]],
                entity_kind: u8_into_entity_kind(self.entity_kind),
            }
        }
    }
    fn entity_kind_into_u8(value: EntityKind) -> u8 {
        match value {
            EntityKind::UserDefinedUnknown => 0x00,
            EntityKind::BuiltInUnknown => 0xc0,
            EntityKind::BuiltInParticipant => 0xc1,
            EntityKind::UserDefinedWriterWithKey => 0x02,
            EntityKind::BuiltInWriterWithKey => 0xc2,
            EntityKind::UserDefinedWriterNoKey => 0x03,
            EntityKind::BuiltInWriterNoKey => 0xc3,
            EntityKind::UserDefinedReaderWithKey => 0x07,
            EntityKind::BuiltInReaderWithKey => 0xc7,
            EntityKind::UserDefinedReaderNoKey => 0x04,
            EntityKind::BuiltInReaderNoKey => 0xc4,
            EntityKind::UserDefinedWriterGroup => 0x08,
            EntityKind::BuiltInWriterGroup => 0xc8,
            EntityKind::UserDefinedReaderGroup => 0x09,
            EntityKind::BuiltInReaderGroup => 0xc9,
        }
    }
    fn u8_into_entity_kind(_value: u8) -> EntityKind {
        ::core::panicking::panic("not yet implemented")
    }
    pub struct SequenceNumberUdp {
        pub(crate) high: i32,
        pub(crate) low: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SequenceNumberUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SequenceNumberUdp {
                    high: ref __self_0_0,
                    low: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SequenceNumberUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "high",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "low",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SequenceNumberUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SequenceNumberUdp {
        #[inline]
        fn eq(&self, other: &SequenceNumberUdp) -> bool {
            match *other {
                SequenceNumberUdp {
                    high: ref __self_1_0,
                    low: ref __self_1_1,
                } => match *self {
                    SequenceNumberUdp {
                        high: ref __self_0_0,
                        low: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SequenceNumberUdp) -> bool {
            match *other {
                SequenceNumberUdp {
                    high: ref __self_1_0,
                    low: ref __self_1_1,
                } => match *self {
                    SequenceNumberUdp {
                        high: ref __self_0_0,
                        low: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for SequenceNumberUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SequenceNumberUdp {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<i32>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SequenceNumberUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SequenceNumberUdp",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "high",
                    &self.high,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "low",
                    &self.low,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SequenceNumberUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "high" => _serde::__private::Ok(__Field::__field0),
                            "low" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"high" => _serde::__private::Ok(__Field::__field0),
                            b"low" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SequenceNumberUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SequenceNumberUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SequenceNumberUdp",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SequenceNumberUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SequenceNumberUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(SequenceNumberUdp {
                            high: __field0,
                            low: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "high",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "low",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("high") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("low") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(SequenceNumberUdp {
                            high: __field0,
                            low: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["high", "low"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SequenceNumberUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SequenceNumberUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl From<&SequenceNumberUdp> for i64 {
        fn from(value: &SequenceNumberUdp) -> Self {
            ((value.high as i64) << 32) + value.low as i64
        }
    }
    impl From<&i64> for SequenceNumberUdp {
        fn from(value: &i64) -> Self {
            Self {
                high: (value >> 32) as i32,
                low: *value as u32,
            }
        }
    }
    impl rust_rtps_pim::messages::submessage_elements::SequenceNumberSubmessageElementType
        for SequenceNumberUdp
    {
        fn new(value: &rust_rtps_pim::structure::types::SequenceNumber) -> Self {
            value.into()
        }
        fn value(&self) -> rust_rtps_pim::structure::types::SequenceNumber {
            self.into()
        }
    }
    pub struct SequenceNumberSetUdp {
        base: SequenceNumberUdp,
        num_bits: u32,
        bitmap: [i32; 8],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SequenceNumberSetUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SequenceNumberSetUdp {
                    base: ref __self_0_0,
                    num_bits: ref __self_0_1,
                    bitmap: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SequenceNumberSetUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "base",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_bits",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "bitmap",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SequenceNumberSetUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SequenceNumberSetUdp {
        #[inline]
        fn eq(&self, other: &SequenceNumberSetUdp) -> bool {
            match *other {
                SequenceNumberSetUdp {
                    base: ref __self_1_0,
                    num_bits: ref __self_1_1,
                    bitmap: ref __self_1_2,
                } => match *self {
                    SequenceNumberSetUdp {
                        base: ref __self_0_0,
                        num_bits: ref __self_0_1,
                        bitmap: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SequenceNumberSetUdp) -> bool {
            match *other {
                SequenceNumberSetUdp {
                    base: ref __self_1_0,
                    num_bits: ref __self_1_1,
                    bitmap: ref __self_1_2,
                } => match *self {
                    SequenceNumberSetUdp {
                        base: ref __self_0_0,
                        num_bits: ref __self_0_1,
                        bitmap: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl SequenceNumberSetUdp {
        pub fn len(&self) -> u16 {
            let number_of_bitmap_elements = ((self.num_bits + 31) / 32) as usize;
            12 + 4 * number_of_bitmap_elements as u16
        }
    }
    impl serde::Serialize for SequenceNumberSetUdp {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let len = 2 + self.bitmap.len();
            let mut state = serializer.serialize_struct("SequenceNumberSet", len)?;
            state.serialize_field("bitmapBase", &self.base)?;
            state.serialize_field("numBits", &self.num_bits)?;
            const BITMAP_NAMES: [&str; 8] = [
                "bitmap[0]",
                "bitmap[1]",
                "bitmap[2]",
                "bitmap[3]",
                "bitmap[4]",
                "bitmap[5]",
                "bitmap[6]",
                "bitmap[7]",
            ];
            let number_of_bitmap_elements = ((self.num_bits + 31) / 32) as usize;
            for i in 0..number_of_bitmap_elements {
                state.serialize_field(BITMAP_NAMES[i], &self.bitmap[i])?;
            }
            state.end()
        }
    }
    struct SequenceNumberSetVisitor;
    impl<'de> serde::de::Visitor<'de> for SequenceNumberSetVisitor {
        type Value = SequenceNumberSetUdp;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("SequenceNumberSet Submessage Element")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let base: SequenceNumberUdp = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
            let num_bits = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
            let num_bitmaps = (num_bits + 31) / 32;
            let mut bitmap = [0; 8];
            for i in 0..num_bitmaps as usize {
                let bitmap_i = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(i + 2, &self))?;
                bitmap[i] = bitmap_i;
            }
            Ok(SequenceNumberSetUdp {
                base,
                num_bits,
                bitmap,
            })
        }
    }
    impl<'de> serde::Deserialize<'de> for SequenceNumberSetUdp {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const MAX_BITMAPS: usize = 8;
            const OTHER_FIELDS: usize = 2;
            const MAX_FIELDS: usize = MAX_BITMAPS + OTHER_FIELDS;
            deserializer.deserialize_tuple(MAX_FIELDS, SequenceNumberSetVisitor)
        }
    }
    impl rust_rtps_pim::messages::submessage_elements::SequenceNumberSetSubmessageElementType
        for SequenceNumberSetUdp
    {
        type IntoIter = std::vec::IntoIter<rust_rtps_pim::structure::types::SequenceNumber>;
        fn new(
            base: &rust_rtps_pim::structure::types::SequenceNumber,
            set: &[rust_rtps_pim::structure::types::SequenceNumber],
        ) -> Self {
            let mut bitmap = [0; 8];
            let mut num_bits = 0;
            for sequence_number in set.iter() {
                let delta_n = (sequence_number - base) as u32;
                let bitmap_num = delta_n / 32;
                bitmap[bitmap_num as usize] |= 1 << (31 - delta_n % 32);
                if delta_n + 1 > num_bits {
                    num_bits = delta_n + 1;
                }
            }
            Self {
                base: base.into(),
                num_bits,
                bitmap,
            }
        }
        fn base(&self) -> rust_rtps_pim::structure::types::SequenceNumber {
            (&self.base).into()
        }
        fn set(&self) -> Self::IntoIter {
            let mut set = ::alloc::vec::Vec::new();
            for delta_n in 0..self.num_bits as usize {
                if (self.bitmap[delta_n / 32] & (1 << (31 - delta_n % 32)))
                    == (1 << (31 - delta_n % 32))
                {
                    let seq_num =
                        Into::<rust_rtps_pim::structure::types::SequenceNumber>::into(&self.base)
                            + delta_n as rust_rtps_pim::structure::types::SequenceNumber;
                    set.push(seq_num);
                }
            }
            set.into_iter()
        }
    }
    pub type InstanceHandleUdp = i32;
    pub struct ProtocolVersionUdp {
        pub major: u8,
        pub minor: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ProtocolVersionUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ProtocolVersionUdp {
                    major: ref __self_0_0,
                    minor: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ProtocolVersionUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "major",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "minor",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ProtocolVersionUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ProtocolVersionUdp {
        #[inline]
        fn eq(&self, other: &ProtocolVersionUdp) -> bool {
            match *other {
                ProtocolVersionUdp {
                    major: ref __self_1_0,
                    minor: ref __self_1_1,
                } => match *self {
                    ProtocolVersionUdp {
                        major: ref __self_0_0,
                        minor: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProtocolVersionUdp) -> bool {
            match *other {
                ProtocolVersionUdp {
                    major: ref __self_1_0,
                    minor: ref __self_1_1,
                } => match *self {
                    ProtocolVersionUdp {
                        major: ref __self_0_0,
                        minor: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ProtocolVersionUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ProtocolVersionUdp",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "major",
                    &self.major,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "minor",
                    &self.minor,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ProtocolVersionUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "major" => _serde::__private::Ok(__Field::__field0),
                            "minor" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"major" => _serde::__private::Ok(__Field::__field0),
                            b"minor" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ProtocolVersionUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ProtocolVersionUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ProtocolVersionUdp",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ProtocolVersionUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ProtocolVersionUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ProtocolVersionUdp {
                            major: __field0,
                            minor: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u8> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "major",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "minor",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("major") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("minor") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ProtocolVersionUdp {
                            major: __field0,
                            minor: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["major", "minor"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ProtocolVersionUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ProtocolVersionUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::ProtocolVersionSubmessageElementType
        for ProtocolVersionUdp
    {
        fn new(value: &ProtocolVersion) -> Self {
            Self {
                major: value.major,
                minor: value.minor,
            }
        }
        fn value(&self) -> ProtocolVersion {
            ProtocolVersion {
                major: self.major,
                minor: self.minor,
            }
        }
    }
    pub struct SerializedDataUdp<'a>(pub &'a [u8]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for SerializedDataUdp<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SerializedDataUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "SerializedDataUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a> ::core::marker::StructuralPartialEq for SerializedDataUdp<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::PartialEq for SerializedDataUdp<'a> {
        #[inline]
        fn eq(&self, other: &SerializedDataUdp<'a>) -> bool {
            match *other {
                SerializedDataUdp(ref __self_1_0) => match *self {
                    SerializedDataUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SerializedDataUdp<'a>) -> bool {
            match *other {
                SerializedDataUdp(ref __self_1_0) => match *self {
                    SerializedDataUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de: 'a, 'a> _serde::Deserialize<'de> for SerializedDataUdp<'a> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de: 'a, 'a> {
                    marker: _serde::__private::PhantomData<SerializedDataUdp<'a>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                    type Value = SerializedDataUdp<'a>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct SerializedDataUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: &'a [u8] =
                            match <&'a [u8] as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(SerializedDataUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<&'a [u8]>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct SerializedDataUdp with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(SerializedDataUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "SerializedDataUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SerializedDataUdp<'a>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<'a> SerializedDataUdp<'a> {
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }
    impl<'a> rust_rtps_pim::messages::submessage_elements::SerializedDataSubmessageElementType<'a>
        for SerializedDataUdp<'a>
    {
        type Value = &'a [u8];
        fn new(value: &Self::Value) -> Self {
            SerializedDataUdp(value)
        }
        fn value(&self) -> Self::Value {
            self.0
        }
    }
    impl<'a> serde::Serialize for SerializedDataUdp<'a> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_bytes(self.0)
        }
    }
    impl<'a>
        rust_rtps_pim::messages::submessage_elements::SerializedDataFragmentSubmessageElementType
        for SerializedDataUdp<'a>
    {
        type Value = &'a [u8];
        fn new<T: Into<Self::Value>>(value: T) -> Self {
            Self(value.into())
        }
        fn value(&self) -> &[u8] {
            self.0
        }
    }
    pub struct VendorIdUdp(pub(crate) [u8; 2]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for VendorIdUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                VendorIdUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "VendorIdUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for VendorIdUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for VendorIdUdp {
        #[inline]
        fn eq(&self, other: &VendorIdUdp) -> bool {
            match *other {
                VendorIdUdp(ref __self_1_0) => match *self {
                    VendorIdUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VendorIdUdp) -> bool {
            match *other {
                VendorIdUdp(ref __self_1_0) => match *self {
                    VendorIdUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for VendorIdUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "VendorIdUdp", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VendorIdUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<VendorIdUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = VendorIdUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct VendorIdUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: [u8; 2] =
                            match <[u8; 2] as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(VendorIdUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<[u8; 2]>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct VendorIdUdp with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(VendorIdUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "VendorIdUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<VendorIdUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::VendorIdSubmessageElementType for VendorIdUdp {
        fn new(value: &rust_rtps_pim::structure::types::VendorId) -> Self {
            Self(value.clone())
        }
        fn value(&self) -> rust_rtps_pim::structure::types::VendorId {
            self.0
        }
    }
    pub struct TimeUdp {
        pub seconds: u32,
        pub fraction: u32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TimeUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TimeUdp {
                    seconds: ref __self_0_0,
                    fraction: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TimeUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "seconds",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "fraction",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for TimeUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TimeUdp {
        #[inline]
        fn eq(&self, other: &TimeUdp) -> bool {
            match *other {
                TimeUdp {
                    seconds: ref __self_1_0,
                    fraction: ref __self_1_1,
                } => match *self {
                    TimeUdp {
                        seconds: ref __self_0_0,
                        fraction: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TimeUdp) -> bool {
            match *other {
                TimeUdp {
                    seconds: ref __self_1_0,
                    fraction: ref __self_1_1,
                } => match *self {
                    TimeUdp {
                        seconds: ref __self_0_0,
                        fraction: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl<'a> rust_rtps_pim::messages::submessage_elements::TimestampSubmessageElementType for TimeUdp {
        fn new(_value: &Time) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn value(&self) -> Time {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub struct CountUdp(pub(crate) i32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for CountUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                CountUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "CountUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for CountUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for CountUdp {
        #[inline]
        fn eq(&self, other: &CountUdp) -> bool {
            match *other {
                CountUdp(ref __self_1_0) => match *self {
                    CountUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &CountUdp) -> bool {
            match *other {
                CountUdp(ref __self_1_0) => match *self {
                    CountUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CountUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "CountUdp", &self.0)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CountUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CountUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CountUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct CountUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: i32 = match <i32 as _serde::Deserialize>::deserialize(__e) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(CountUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct CountUdp with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(CountUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "CountUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CountUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<'a> rust_rtps_pim::messages::submessage_elements::CountSubmessageElementType for CountUdp {
        fn new(value: &Count) -> Self {
            Self(value.0)
        }
        fn value(&self) -> Count {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub struct FragmentNumberUdp(pub(crate) u32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for FragmentNumberUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                FragmentNumberUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "FragmentNumberUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for FragmentNumberUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for FragmentNumberUdp {
        #[inline]
        fn eq(&self, other: &FragmentNumberUdp) -> bool {
            match *other {
                FragmentNumberUdp(ref __self_1_0) => match *self {
                    FragmentNumberUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &FragmentNumberUdp) -> bool {
            match *other {
                FragmentNumberUdp(ref __self_1_0) => match *self {
                    FragmentNumberUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl rust_rtps_pim::messages::submessage_elements::FragmentNumberSubmessageElementType
        for FragmentNumberUdp
    {
        fn new(value: &FragmentNumber) -> Self {
            Self(value.0)
        }
        fn value(&self) -> FragmentNumber {
            FragmentNumber(self.0)
        }
    }
    impl From<u32> for FragmentNumberUdp {
        fn from(_: u32) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl Into<u32> for FragmentNumberUdp {
        fn into(self) -> u32 {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub struct FragmentNumberSetUdp(Vec<FragmentNumberUdp>);
    impl rust_rtps_pim::messages::submessage_elements::FragmentNumberSetSubmessageElementType
        for FragmentNumberSetUdp
    {
        type IntoIter = Vec<FragmentNumber>;
        fn new(_base: &FragmentNumber, _set: &[FragmentNumber]) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn base(&self) -> FragmentNumber {
            ::core::panicking::panic("not yet implemented")
        }
        fn set(&self) -> Self::IntoIter {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub type GroupDigestUdp = [u8; 4];
    pub struct LocatorUdp {
        kind: i32,
        port: u32,
        address: [u8; 16],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LocatorUdp {
        #[inline]
        fn clone(&self) -> LocatorUdp {
            {
                let _: ::core::clone::AssertParamIsClone<i32>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<[u8; 16]>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for LocatorUdp {}
    impl ::core::marker::StructuralPartialEq for LocatorUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for LocatorUdp {
        #[inline]
        fn eq(&self, other: &LocatorUdp) -> bool {
            match *other {
                LocatorUdp {
                    kind: ref __self_1_0,
                    port: ref __self_1_1,
                    address: ref __self_1_2,
                } => match *self {
                    LocatorUdp {
                        kind: ref __self_0_0,
                        port: ref __self_0_1,
                        address: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LocatorUdp) -> bool {
            match *other {
                LocatorUdp {
                    kind: ref __self_1_0,
                    port: ref __self_1_1,
                    address: ref __self_1_2,
                } => match *self {
                    LocatorUdp {
                        kind: ref __self_0_0,
                        port: ref __self_0_1,
                        address: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LocatorUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LocatorUdp {
                    kind: ref __self_0_0,
                    port: ref __self_0_1,
                    address: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "LocatorUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "kind",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "port",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "address",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for LocatorUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "LocatorUdp",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kind",
                    &self.kind,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "port",
                    &self.port,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for LocatorUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kind" => _serde::__private::Ok(__Field::__field0),
                            "port" => _serde::__private::Ok(__Field::__field1),
                            "address" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kind" => _serde::__private::Ok(__Field::__field0),
                            b"port" => _serde::__private::Ok(__Field::__field1),
                            b"address" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<LocatorUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LocatorUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct LocatorUdp")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct LocatorUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct LocatorUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<[u8; 16]>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct LocatorUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(LocatorUdp {
                            kind: __field0,
                            port: __field1,
                            address: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<[u8; 16]> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kind",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "port",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<[u8; 16]>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("kind") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("port") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("address") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(LocatorUdp {
                            kind: __field0,
                            port: __field1,
                            address: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["kind", "port", "address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "LocatorUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<LocatorUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl LocatorUdp {
        pub fn new(locator: &Locator) -> Self {
            Self {
                kind: *locator.kind(),
                port: *locator.port(),
                address: *locator.address(),
            }
        }
        pub fn value(&self) -> Locator {
            Locator::new(self.kind, self.port, self.address)
        }
    }
    pub struct LocatorListUdp(pub Vec<LocatorUdp>);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for LocatorListUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<LocatorListUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = LocatorListUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct LocatorListUdp",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<LocatorUdp> =
                            match <Vec<LocatorUdp> as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(LocatorListUdp(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<LocatorUdp>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct LocatorListUdp with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(LocatorListUdp(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "LocatorListUdp",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<LocatorListUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl rust_rtps_pim::messages::submessage_elements::LocatorListSubmessageElementType
        for LocatorListUdp
    {
        type IntoIter = Vec<rust_rtps_pim::structure::types::Locator>;
        fn new(value: &[rust_rtps_pim::structure::types::Locator]) -> Self {
            let mut locator_list = Vec::new();
            for locator in value {
                locator_list.push(LocatorUdp::new(locator));
            }
            Self(locator_list)
        }
        fn value(&self) -> Self::IntoIter {
            let mut locator_list = Vec::new();
            for locator_udp in &self.0 {
                let locator = rust_rtps_pim::structure::types::Locator::new(
                    locator_udp.kind,
                    locator_udp.port,
                    locator_udp.address,
                );
                locator_list.push(locator);
            }
            locator_list
        }
    }
}
pub mod message {
    use std::io::BufRead;
    use rust_rtps_pim::messages::{submessages::RtpsSubmessageType, RtpsMessageHeader};
    use rust_serde_cdr::{deserializer::RtpsMessageDeserializer, error::Error};
    use serde::{de::Deserialize, ser::SerializeStruct};
    use crate::{
        message_header::RTPSMessageHeaderUdp,
        psm::RtpsUdpPsm,
        submessage_header::{SubmessageHeaderUdp, DATA, GAP},
    };
    pub struct RTPSMessageUdp<'a> {
        header: RTPSMessageHeaderUdp,
        submessages: Vec<RtpsSubmessageType<'a, RtpsUdpPsm>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for RTPSMessageUdp<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RTPSMessageUdp {
                    header: ref __self_0_0,
                    submessages: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "RTPSMessageUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "header",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "submessages",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a> ::core::marker::StructuralPartialEq for RTPSMessageUdp<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::PartialEq for RTPSMessageUdp<'a> {
        #[inline]
        fn eq(&self, other: &RTPSMessageUdp<'a>) -> bool {
            match *other {
                RTPSMessageUdp {
                    header: ref __self_1_0,
                    submessages: ref __self_1_1,
                } => match *self {
                    RTPSMessageUdp {
                        header: ref __self_0_0,
                        submessages: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &RTPSMessageUdp<'a>) -> bool {
            match *other {
                RTPSMessageUdp {
                    header: ref __self_1_0,
                    submessages: ref __self_1_1,
                } => match *self {
                    RTPSMessageUdp {
                        header: ref __self_0_0,
                        submessages: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl<'a> rust_rtps_pim::messages::RTPSMessage for RTPSMessageUdp<'a> {
        type SubmessageType = RtpsSubmessageType<'a, RtpsUdpPsm>;
        fn new<T: IntoIterator<Item = Self::SubmessageType>>(
            header: &RtpsMessageHeader,
            submessages: T,
        ) -> Self {
            Self {
                header: header.into(),
                submessages: submessages.into_iter().collect(),
            }
        }
        fn header(&self) -> RtpsMessageHeader {
            (&self.header).into()
        }
        fn submessages(&self) -> &[Self::SubmessageType] {
            &self.submessages
        }
    }
    impl<'a> serde::Serialize for RTPSMessageUdp<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let len = self.submessages.len() + 1;
            let mut state = serializer.serialize_struct("RTPSMessage", len)?;
            state.serialize_field("header", &self.header)?;
            for submessage in &self.submessages {
                match submessage {
                    RtpsSubmessageType::Data(submessage) => {
                        state.serialize_field("submessage", submessage)?
                    }
                    RtpsSubmessageType::Gap(submessage) => {
                        state.serialize_field("submessage", submessage)?
                    }
                    _ => ::core::panicking::panic("not yet implemented"),
                }
            }
            state.end()
        }
    }
    impl<'a> RTPSMessageUdp<'a> {
        pub fn from_bytes(buf: &'a [u8]) -> Result<Self, Error> {
            let mut submessages = ::alloc::vec::Vec::new();
            let mut header_deserializer = RtpsMessageDeserializer { reader: buf };
            let header: RTPSMessageHeaderUdp = Deserialize::deserialize(&mut header_deserializer)?;
            const MAX_SUBMESSAGES: usize = 2 ^ 16;
            for _ in 0..MAX_SUBMESSAGES {
                if header_deserializer.reader.len() < 1 {
                    break;
                };
                let mut deserializer = RtpsMessageDeserializer {
                    reader: &header_deserializer.reader,
                };
                let submessage_header: SubmessageHeaderUdp =
                    Deserialize::deserialize(&mut header_deserializer)?;
                let submessage_length = submessage_header.submessage_length as usize;
                let typed_submessage = match submessage_header.submessage_id.into() {
                    GAP => Some(RtpsSubmessageType::Gap(Deserialize::deserialize(
                        &mut deserializer,
                    )?)),
                    DATA => Some(RtpsSubmessageType::Data(Deserialize::deserialize(
                        &mut deserializer,
                    )?)),
                    _ => None,
                };
                if let Some(typed_submessage) = typed_submessage {
                    submessages.push(typed_submessage);
                }
                header_deserializer.reader.consume(submessage_length);
            }
            Ok(RTPSMessageUdp {
                header,
                submessages,
            })
        }
    }
}
pub mod submessage_header {
    use rust_rtps_pim::messages::types::SubmessageKind;
    pub const DATA: u8 = 0x15;
    pub const GAP: u8 = 0x08;
    pub const HEARTBEAT: u8 = 0x07;
    pub const ACKNACK: u8 = 0x06;
    pub const PAD: u8 = 0x01;
    pub const INFO_TS: u8 = 0x09;
    pub const INFO_REPLY: u8 = 0x0f;
    pub const INFO_DST: u8 = 0x0e;
    pub const INFO_SRC: u8 = 0x0c;
    pub const DATA_FRAG: u8 = 0x16;
    pub const NACK_FRAG: u8 = 0x12;
    pub const HEARTBEAT_FRAG: u8 = 0x13;
    pub fn submessage_kind_into_byte(value: SubmessageKind) -> u8 {
        match value {
            SubmessageKind::DATA => DATA,
            SubmessageKind::GAP => GAP,
            SubmessageKind::HEARTBEAT => HEARTBEAT,
            SubmessageKind::ACKNACK => ACKNACK,
            SubmessageKind::PAD => PAD,
            SubmessageKind::INFO_TS => INFO_TS,
            SubmessageKind::INFO_REPLY => INFO_REPLY,
            SubmessageKind::INFO_DST => INFO_DST,
            SubmessageKind::INFO_SRC => INFO_SRC,
            SubmessageKind::DATA_FRAG => DATA_FRAG,
            SubmessageKind::NACK_FRAG => NACK_FRAG,
            SubmessageKind::HEARTBEAT_FRAG => HEARTBEAT_FRAG,
        }
    }
    pub struct SubmessageHeaderUdp {
        pub(crate) submessage_id: u8,
        pub(crate) flags: u8,
        pub(crate) submessage_length: u16,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SubmessageHeaderUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SubmessageHeaderUdp {
                    submessage_id: ref __self_0_0,
                    flags: ref __self_0_1,
                    submessage_length: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SubmessageHeaderUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "submessage_id",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "flags",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "submessage_length",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SubmessageHeaderUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SubmessageHeaderUdp {
        #[inline]
        fn eq(&self, other: &SubmessageHeaderUdp) -> bool {
            match *other {
                SubmessageHeaderUdp {
                    submessage_id: ref __self_1_0,
                    flags: ref __self_1_1,
                    submessage_length: ref __self_1_2,
                } => match *self {
                    SubmessageHeaderUdp {
                        submessage_id: ref __self_0_0,
                        flags: ref __self_0_1,
                        submessage_length: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SubmessageHeaderUdp) -> bool {
            match *other {
                SubmessageHeaderUdp {
                    submessage_id: ref __self_1_0,
                    flags: ref __self_1_1,
                    submessage_length: ref __self_1_2,
                } => match *self {
                    SubmessageHeaderUdp {
                        submessage_id: ref __self_0_0,
                        flags: ref __self_0_1,
                        submessage_length: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SubmessageHeaderUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "SubmessageHeaderUdp",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "submessage_id",
                    &self.submessage_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "flags",
                    &self.flags,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "submessage_length",
                    &self.submessage_length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SubmessageHeaderUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "submessage_id" => _serde::__private::Ok(__Field::__field0),
                            "flags" => _serde::__private::Ok(__Field::__field1),
                            "submessage_length" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"submessage_id" => _serde::__private::Ok(__Field::__field0),
                            b"flags" => _serde::__private::Ok(__Field::__field1),
                            b"submessage_length" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SubmessageHeaderUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SubmessageHeaderUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SubmessageHeaderUdp",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SubmessageHeaderUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SubmessageHeaderUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<u16>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct SubmessageHeaderUdp with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(SubmessageHeaderUdp {
                            submessage_id: __field0,
                            flags: __field1,
                            submessage_length: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<u16> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "submessage_id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "flags",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "submessage_length",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u16>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("submessage_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("flags") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("submessage_length") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(SubmessageHeaderUdp {
                            submessage_id: __field0,
                            flags: __field1,
                            submessage_length: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["submessage_id", "flags", "submessage_length"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SubmessageHeaderUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SubmessageHeaderUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl SubmessageHeaderUdp {
        pub const fn number_of_bytes(&self) -> usize {
            4
        }
    }
}
pub mod message_header {
    use rust_rtps_pim::{messages::RtpsMessageHeader, structure::types::ProtocolVersion};
    use crate::submessage_elements::{GuidPrefixUdp, ProtocolVersionUdp, VendorIdUdp};
    pub type ProtocolIdUdp = [u8; 4];
    pub struct RTPSMessageHeaderUdp {
        pub(crate) protocol: ProtocolIdUdp,
        pub(crate) version: ProtocolVersionUdp,
        pub(crate) vendor_id: VendorIdUdp,
        pub(crate) guid_prefix: GuidPrefixUdp,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RTPSMessageHeaderUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RTPSMessageHeaderUdp {
                    protocol: ref __self_0_0,
                    version: ref __self_0_1,
                    vendor_id: ref __self_0_2,
                    guid_prefix: ref __self_0_3,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "RTPSMessageHeaderUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "protocol",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "version",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "vendor_id",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "guid_prefix",
                        &&(*__self_0_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for RTPSMessageHeaderUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for RTPSMessageHeaderUdp {
        #[inline]
        fn eq(&self, other: &RTPSMessageHeaderUdp) -> bool {
            match *other {
                RTPSMessageHeaderUdp {
                    protocol: ref __self_1_0,
                    version: ref __self_1_1,
                    vendor_id: ref __self_1_2,
                    guid_prefix: ref __self_1_3,
                } => match *self {
                    RTPSMessageHeaderUdp {
                        protocol: ref __self_0_0,
                        version: ref __self_0_1,
                        vendor_id: ref __self_0_2,
                        guid_prefix: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &RTPSMessageHeaderUdp) -> bool {
            match *other {
                RTPSMessageHeaderUdp {
                    protocol: ref __self_1_0,
                    version: ref __self_1_1,
                    vendor_id: ref __self_1_2,
                    guid_prefix: ref __self_1_3,
                } => match *self {
                    RTPSMessageHeaderUdp {
                        protocol: ref __self_0_0,
                        version: ref __self_0_1,
                        vendor_id: ref __self_0_2,
                        guid_prefix: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RTPSMessageHeaderUdp {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "RTPSMessageHeaderUdp",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "protocol",
                    &self.protocol,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "vendor_id",
                    &self.vendor_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "guid_prefix",
                    &self.guid_prefix,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RTPSMessageHeaderUdp {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "protocol" => _serde::__private::Ok(__Field::__field0),
                            "version" => _serde::__private::Ok(__Field::__field1),
                            "vendor_id" => _serde::__private::Ok(__Field::__field2),
                            "guid_prefix" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"protocol" => _serde::__private::Ok(__Field::__field0),
                            b"version" => _serde::__private::Ok(__Field::__field1),
                            b"vendor_id" => _serde::__private::Ok(__Field::__field2),
                            b"guid_prefix" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RTPSMessageHeaderUdp>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RTPSMessageHeaderUdp;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RTPSMessageHeaderUdp",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ProtocolIdUdp,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct RTPSMessageHeaderUdp with 4 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            ProtocolVersionUdp,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct RTPSMessageHeaderUdp with 4 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<VendorIdUdp>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct RTPSMessageHeaderUdp with 4 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            GuidPrefixUdp,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct RTPSMessageHeaderUdp with 4 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(RTPSMessageHeaderUdp {
                            protocol: __field0,
                            version: __field1,
                            vendor_id: __field2,
                            guid_prefix: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<ProtocolIdUdp> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ProtocolVersionUdp> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<VendorIdUdp> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<GuidPrefixUdp> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "protocol",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ProtocolIdUdp>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ProtocolVersionUdp>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "vendor_id",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<VendorIdUdp>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "guid_prefix",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<GuidPrefixUdp>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("protocol") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("version") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("vendor_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("guid_prefix") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(RTPSMessageHeaderUdp {
                            protocol: __field0,
                            version: __field1,
                            vendor_id: __field2,
                            guid_prefix: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["protocol", "version", "vendor_id", "guid_prefix"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RTPSMessageHeaderUdp",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RTPSMessageHeaderUdp>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl RTPSMessageHeaderUdp {
        pub const fn number_of_bytes(&self) -> usize {
            20
        }
    }
    impl From<&RTPSMessageHeaderUdp> for RtpsMessageHeader {
        fn from(header: &RTPSMessageHeaderUdp) -> Self {
            Self {
                protocol: rust_rtps_pim::messages::types::ProtocolId::PROTOCOL_RTPS,
                version: ProtocolVersion {
                    major: header.version.major,
                    minor: header.version.minor,
                },
                vendor_id: header.vendor_id.0,
                guid_prefix: header.guid_prefix.0,
            }
        }
    }
    impl From<&RtpsMessageHeader> for RTPSMessageHeaderUdp {
        fn from(header: &RtpsMessageHeader) -> Self {
            Self {
                protocol: [b'R', b'T', b'P', b'S'],
                version: ProtocolVersionUdp {
                    major: header.version.major,
                    minor: header.version.minor,
                },
                vendor_id: VendorIdUdp(header.vendor_id),
                guid_prefix: GuidPrefixUdp(header.guid_prefix),
            }
        }
    }
}
pub mod parameter_list {
    use std::{collections::HashMap, io::BufRead, marker::PhantomData};
    use rust_rtps_pim::messages::types::ParameterId;
    use rust_serde_cdr::deserializer::RtpsMessageDeserializer;
    use serde::ser::SerializeStruct;
    pub struct VectorUdp(pub Vec<u8>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for VectorUdp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                VectorUdp(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "VectorUdp");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for VectorUdp {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for VectorUdp {
        #[inline]
        fn eq(&self, other: &VectorUdp) -> bool {
            match *other {
                VectorUdp(ref __self_1_0) => match *self {
                    VectorUdp(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VectorUdp) -> bool {
            match *other {
                VectorUdp(ref __self_1_0) => match *self {
                    VectorUdp(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl serde::Serialize for VectorUdp {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_bytes(self.0.as_slice())
        }
    }
    pub struct ParameterUdp<'a> {
        pub parameter_id: u16,
        pub length: i16,
        #[serde(with = "serde_bytes")]
        pub value: &'a [u8],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for ParameterUdp<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ParameterUdp {
                    parameter_id: ref __self_0_0,
                    length: ref __self_0_1,
                    value: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ParameterUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "parameter_id",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "length",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "value",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a> ::core::marker::StructuralPartialEq for ParameterUdp<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::PartialEq for ParameterUdp<'a> {
        #[inline]
        fn eq(&self, other: &ParameterUdp<'a>) -> bool {
            match *other {
                ParameterUdp {
                    parameter_id: ref __self_1_0,
                    length: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    ParameterUdp {
                        parameter_id: ref __self_0_0,
                        length: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ParameterUdp<'a>) -> bool {
            match *other {
                ParameterUdp {
                    parameter_id: ref __self_1_0,
                    length: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    ParameterUdp {
                        parameter_id: ref __self_0_0,
                        length: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'a> _serde::Serialize for ParameterUdp<'a> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ParameterUdp",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parameter_id",
                    &self.parameter_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "length",
                    &self.length,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "value", {
                    struct __SerializeWith<'__a, 'a: '__a> {
                        values: (&'__a &'a [u8],),
                        phantom: _serde::__private::PhantomData<ParameterUdp<'a>>,
                    }
                    impl<'__a, 'a: '__a> _serde::Serialize for __SerializeWith<'__a, 'a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            serde_bytes::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.value,),
                        phantom: _serde::__private::PhantomData::<ParameterUdp<'a>>,
                    }
                }) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl<'a> ParameterUdp<'a> {
        pub fn new(parameter_id: u16, value: &'a [u8]) -> Self {
            let length = value.len() as i16;
            Self {
                parameter_id,
                length,
                value,
            }
        }
        pub fn len(&self) -> usize {
            4 + self.value.len()
        }
    }
    struct ParameterVisitor<'a>(PhantomData<&'a ()>);
    impl<'a, 'de: 'a> serde::de::Visitor<'de> for ParameterVisitor<'a> {
        type Value = ParameterUdp<'a>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Parameter of the ParameterList Submessage Element")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let parameter_id: u16 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
            let length: i16 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
            let data: &[u8] = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
            Ok(ParameterUdp {
                parameter_id,
                length,
                value: &data[..length as usize],
            })
        }
    }
    impl<'a, 'de: 'a> serde::Deserialize<'de> for ParameterUdp<'a> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["parameter_id", "length", "value"];
            deserializer.deserialize_struct("Parameter", FIELDS, ParameterVisitor(PhantomData))
        }
    }
    const PID_SENTINEL: u16 = 1;
    static SENTINEL: ParameterUdp = ParameterUdp {
        parameter_id: PID_SENTINEL,
        length: 0,
        value: &[],
    };
    impl<'a> rust_rtps_pim::messages::submessage_elements::ParameterListSubmessageElementType<'a>
        for ParameterListUdp<'a>
    {
        type IntoIter = Vec<rust_rtps_pim::messages::submessage_elements::Parameter<'a>>;
        fn new(parameter: &[rust_rtps_pim::messages::submessage_elements::Parameter]) -> Self {
            ::core::panicking::panic("not yet implemented")
        }
        fn parameter(&'a self) -> Self::IntoIter {
            self.parameter
                .iter()
                .map(
                    |x| rust_rtps_pim::messages::submessage_elements::Parameter {
                        parameter_id: ParameterId(x.parameter_id),
                        length: x.value.len() as i16,
                        value: x.value,
                    },
                )
                .collect()
        }
    }
    pub struct ParameterListUdp<'a> {
        pub parameter: Vec<ParameterUdp<'a>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for ParameterListUdp<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ParameterListUdp {
                    parameter: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ParameterListUdp");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "parameter",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a> ::core::marker::StructuralPartialEq for ParameterListUdp<'a> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::cmp::PartialEq for ParameterListUdp<'a> {
        #[inline]
        fn eq(&self, other: &ParameterListUdp<'a>) -> bool {
            match *other {
                ParameterListUdp {
                    parameter: ref __self_1_0,
                } => match *self {
                    ParameterListUdp {
                        parameter: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ParameterListUdp<'a>) -> bool {
            match *other {
                ParameterListUdp {
                    parameter: ref __self_1_0,
                } => match *self {
                    ParameterListUdp {
                        parameter: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl<'a> ParameterListUdp<'a> {
        pub fn len(&self) -> usize {
            self.parameter.iter().map(|p| p.len()).sum::<usize>() + 4
        }
        pub fn get<'b: 'de, 'de, T: serde::Deserialize<'de>>(
            &'b self,
            parameter_id: u16,
        ) -> Option<T> {
            let map = self.as_map();
            let bytes = map.get(&parameter_id)?;
            rust_serde_cdr::deserializer::from_bytes(bytes).ok()
        }
        pub fn get_list<'b: 'de, 'de, T: serde::Deserialize<'de>>(
            &'b self,
            parameter_id: u16,
        ) -> Vec<T> {
            let mut list = Vec::new();
            for parameter in &self.parameter {
                if parameter.parameter_id == parameter_id {
                    let value = rust_serde_cdr::deserializer::from_bytes(parameter.value).unwrap();
                    list.push(value);
                }
            }
            list
        }
        fn as_map(&self) -> HashMap<u16, &[u8]> {
            let mut map = HashMap::new();
            for parameter_i in &self.parameter {
                map.insert(parameter_i.parameter_id, parameter_i.value);
            }
            map
        }
    }
    impl<'a> serde::Serialize for ParameterListUdp<'a> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let len = self.parameter.len() + 1;
            let mut state = serializer.serialize_struct("ParameterList", len)?;
            for parameter in &self.parameter {
                state.serialize_field("parameter", &parameter)?;
            }
            state.serialize_field("sentinel", &SENTINEL)?;
            state.end()
        }
    }
    struct ParameterListVisitor<'a>(PhantomData<&'a ()>);
    impl<'a, 'de: 'a> serde::de::Visitor<'de> for ParameterListVisitor<'a> {
        type Value = ParameterListUdp<'a>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("ParameterList Submessage Element")
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            const MAX_PARAMETERS: usize = 2_usize.pow(16);
            let buf: &[u8] = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
            let mut parameter = ::alloc::vec::Vec::new();
            let mut de = RtpsMessageDeserializer { reader: buf };
            for _ in 0..MAX_PARAMETERS {
                let parameter_i: ParameterUdp =
                    serde::de::Deserialize::deserialize(&mut de).unwrap();
                if parameter_i == SENTINEL {
                    return Ok(ParameterListUdp { parameter });
                } else {
                    parameter.push(parameter_i);
                }
            }
            Ok(ParameterListUdp { parameter })
        }
    }
    impl<'a, 'de: 'a> serde::Deserialize<'de> for ParameterListUdp<'a> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &'static [&'static str] = &["parameter"];
            deserializer.deserialize_struct(
                "ParameterList",
                FIELDS,
                ParameterListVisitor(PhantomData),
            )
        }
    }
}
pub mod builtin_endpoints {
    pub mod data {
        use crate::{
            builtin_endpoints::parameterid_list::PID_DOMAIN_ID,
            parameter_list::{ParameterListUdp, ParameterUdp},
            submessage_elements::{
                CountUdp, EntityIdUdp, GuidPrefixUdp, LocatorUdp, ProtocolVersionUdp, VendorIdUdp,
            },
        };
        use rust_rtps_pim::{
            behavior::types::Duration,
            discovery::{
                spdp::spdp_discovered_participant_data::SPDPdiscoveredParticipantData,
                types::{BuiltinEndpointQos, BuiltinEndpointSet, DomainId},
            },
            messages::{
                submessage_elements::{
                    CountSubmessageElementType, EntityIdSubmessageElementType,
                    GuidPrefixSubmessageElementType, ProtocolVersionSubmessageElementType,
                    VendorIdSubmessageElementType,
                },
                types::Count,
            },
            structure::types::{GuidPrefix, Locator, ProtocolVersion, VendorId, GUID},
        };
        use super::parameterid_list::{
            PID_BUILTIN_ENDPOINT_QOS, PID_BUILTIN_ENDPOINT_SET, PID_DEFAULT_MULTICAST_LOCATOR,
            PID_DEFAULT_UNICAST_LOCATOR, PID_DOMAIN_TAG, PID_EXPECTS_INLINE_QOS,
            PID_METATRAFFIC_MULTICAST_LOCATOR, PID_METATRAFFIC_UNICAST_LOCATOR,
            PID_PARTICIPANT_GUID, PID_PARTICIPANT_LEASE_DURATION,
            PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT, PID_PROTOCOL_VERSION, PID_VENDORID,
        };
        const PL_CDR_LE: [u8; 4] = [0x00, 0x03, 0x00, 0x00];
        struct GUIDUdp {
            prefix: GuidPrefixUdp,
            entity_id: EntityIdUdp,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for GUIDUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    GUIDUdp {
                        prefix: ref __self_0_0,
                        entity_id: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "GUIDUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "prefix",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "entity_id",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for GUIDUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for GUIDUdp {
            #[inline]
            fn eq(&self, other: &GUIDUdp) -> bool {
                match *other {
                    GUIDUdp {
                        prefix: ref __self_1_0,
                        entity_id: ref __self_1_1,
                    } => match *self {
                        GUIDUdp {
                            prefix: ref __self_0_0,
                            entity_id: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &GUIDUdp) -> bool {
                match *other {
                    GUIDUdp {
                        prefix: ref __self_1_0,
                        entity_id: ref __self_1_1,
                    } => match *self {
                        GUIDUdp {
                            prefix: ref __self_0_0,
                            entity_id: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for GUIDUdp {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "GUIDUdp",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "prefix",
                        &self.prefix,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "entity_id",
                        &self.entity_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for GUIDUdp {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "prefix" => _serde::__private::Ok(__Field::__field0),
                                "entity_id" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"prefix" => _serde::__private::Ok(__Field::__field0),
                                b"entity_id" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<GUIDUdp>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GUIDUdp;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct GUIDUdp")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                GuidPrefixUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GUIDUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                EntityIdUdp,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GUIDUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GUIDUdp {
                                prefix: __field0,
                                entity_id: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<GuidPrefixUdp> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<EntityIdUdp> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "prefix",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<GuidPrefixUdp>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "entity_id",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<EntityIdUdp>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("prefix") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("entity_id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(GUIDUdp {
                                prefix: __field0,
                                entity_id: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["prefix", "entity_id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GUIDUdp",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GUIDUdp>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl GUIDUdp {
            pub fn new(guid: &GUID) -> Self {
                Self {
                    prefix: GuidPrefixUdp::new(guid.prefix()),
                    entity_id: EntityIdUdp::new(guid.entity_id()),
                }
            }
            pub fn value(&self) -> GUID {
                GUID::new(self.prefix.value(), self.entity_id.value())
            }
        }
        pub struct DurationUdp {
            seconds: i32,
            fraction: u32,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for DurationUdp {
            #[inline]
            fn clone(&self) -> DurationUdp {
                {
                    let _: ::core::clone::AssertParamIsClone<i32>;
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for DurationUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DurationUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DurationUdp {
                        seconds: ref __self_0_0,
                        fraction: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "DurationUdp");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "seconds",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "fraction",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for DurationUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for DurationUdp {
            #[inline]
            fn eq(&self, other: &DurationUdp) -> bool {
                match *other {
                    DurationUdp {
                        seconds: ref __self_1_0,
                        fraction: ref __self_1_1,
                    } => match *self {
                        DurationUdp {
                            seconds: ref __self_0_0,
                            fraction: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &DurationUdp) -> bool {
                match *other {
                    DurationUdp {
                        seconds: ref __self_1_0,
                        fraction: ref __self_1_1,
                    } => match *self {
                        DurationUdp {
                            seconds: ref __self_0_0,
                            fraction: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DurationUdp {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DurationUdp",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "seconds",
                        &self.seconds,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "fraction",
                        &self.fraction,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DurationUdp {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "seconds" => _serde::__private::Ok(__Field::__field0),
                                "fraction" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"seconds" => _serde::__private::Ok(__Field::__field0),
                                b"fraction" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<DurationUdp>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DurationUdp;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DurationUdp",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DurationUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DurationUdp with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DurationUdp {
                                seconds: __field0,
                                fraction: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "seconds",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "fraction",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("seconds") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("fraction") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DurationUdp {
                                seconds: __field0,
                                fraction: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["seconds", "fraction"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DurationUdp",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<DurationUdp>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl DurationUdp {
            pub fn new(duration: &Duration) -> Self {
                Self {
                    seconds: duration.seconds,
                    fraction: duration.fraction,
                }
            }
            pub fn value(&self) -> Duration {
                Duration {
                    seconds: self.seconds,
                    fraction: self.fraction,
                }
            }
        }
        struct ParticipantProxy {
            domain_id: u32,
            domain_tag: String,
            protocol_version: ProtocolVersionUdp,
            guid: GUIDUdp,
            vendor_id: VendorIdUdp,
            expects_inline_qos: bool,
            metatraffic_unicast_locator_list: Vec<LocatorUdp>,
            metatraffic_multicast_locator_list: Vec<LocatorUdp>,
            default_unicast_locator_list: Vec<LocatorUdp>,
            default_multicast_locator_list: Vec<LocatorUdp>,
            available_builtin_endpoints: u32,
            manual_liveliness_count: CountUdp,
            builtin_endpoint_qos: u32,
        }
        impl ::core::marker::StructuralPartialEq for ParticipantProxy {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ParticipantProxy {
            #[inline]
            fn eq(&self, other: &ParticipantProxy) -> bool {
                match *other {
                    ParticipantProxy {
                        domain_id: ref __self_1_0,
                        domain_tag: ref __self_1_1,
                        protocol_version: ref __self_1_2,
                        guid: ref __self_1_3,
                        vendor_id: ref __self_1_4,
                        expects_inline_qos: ref __self_1_5,
                        metatraffic_unicast_locator_list: ref __self_1_6,
                        metatraffic_multicast_locator_list: ref __self_1_7,
                        default_unicast_locator_list: ref __self_1_8,
                        default_multicast_locator_list: ref __self_1_9,
                        available_builtin_endpoints: ref __self_1_10,
                        manual_liveliness_count: ref __self_1_11,
                        builtin_endpoint_qos: ref __self_1_12,
                    } => match *self {
                        ParticipantProxy {
                            domain_id: ref __self_0_0,
                            domain_tag: ref __self_0_1,
                            protocol_version: ref __self_0_2,
                            guid: ref __self_0_3,
                            vendor_id: ref __self_0_4,
                            expects_inline_qos: ref __self_0_5,
                            metatraffic_unicast_locator_list: ref __self_0_6,
                            metatraffic_multicast_locator_list: ref __self_0_7,
                            default_unicast_locator_list: ref __self_0_8,
                            default_multicast_locator_list: ref __self_0_9,
                            available_builtin_endpoints: ref __self_0_10,
                            manual_liveliness_count: ref __self_0_11,
                            builtin_endpoint_qos: ref __self_0_12,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                                && (*__self_0_6) == (*__self_1_6)
                                && (*__self_0_7) == (*__self_1_7)
                                && (*__self_0_8) == (*__self_1_8)
                                && (*__self_0_9) == (*__self_1_9)
                                && (*__self_0_10) == (*__self_1_10)
                                && (*__self_0_11) == (*__self_1_11)
                                && (*__self_0_12) == (*__self_1_12)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &ParticipantProxy) -> bool {
                match *other {
                    ParticipantProxy {
                        domain_id: ref __self_1_0,
                        domain_tag: ref __self_1_1,
                        protocol_version: ref __self_1_2,
                        guid: ref __self_1_3,
                        vendor_id: ref __self_1_4,
                        expects_inline_qos: ref __self_1_5,
                        metatraffic_unicast_locator_list: ref __self_1_6,
                        metatraffic_multicast_locator_list: ref __self_1_7,
                        default_unicast_locator_list: ref __self_1_8,
                        default_multicast_locator_list: ref __self_1_9,
                        available_builtin_endpoints: ref __self_1_10,
                        manual_liveliness_count: ref __self_1_11,
                        builtin_endpoint_qos: ref __self_1_12,
                    } => match *self {
                        ParticipantProxy {
                            domain_id: ref __self_0_0,
                            domain_tag: ref __self_0_1,
                            protocol_version: ref __self_0_2,
                            guid: ref __self_0_3,
                            vendor_id: ref __self_0_4,
                            expects_inline_qos: ref __self_0_5,
                            metatraffic_unicast_locator_list: ref __self_0_6,
                            metatraffic_multicast_locator_list: ref __self_0_7,
                            default_unicast_locator_list: ref __self_0_8,
                            default_multicast_locator_list: ref __self_0_9,
                            available_builtin_endpoints: ref __self_0_10,
                            manual_liveliness_count: ref __self_0_11,
                            builtin_endpoint_qos: ref __self_0_12,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                                || (*__self_0_6) != (*__self_1_6)
                                || (*__self_0_7) != (*__self_1_7)
                                || (*__self_0_8) != (*__self_1_8)
                                || (*__self_0_9) != (*__self_1_9)
                                || (*__self_0_10) != (*__self_1_10)
                                || (*__self_0_11) != (*__self_1_11)
                                || (*__self_0_12) != (*__self_1_12)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ParticipantProxy {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ParticipantProxy {
                        domain_id: ref __self_0_0,
                        domain_tag: ref __self_0_1,
                        protocol_version: ref __self_0_2,
                        guid: ref __self_0_3,
                        vendor_id: ref __self_0_4,
                        expects_inline_qos: ref __self_0_5,
                        metatraffic_unicast_locator_list: ref __self_0_6,
                        metatraffic_multicast_locator_list: ref __self_0_7,
                        default_unicast_locator_list: ref __self_0_8,
                        default_multicast_locator_list: ref __self_0_9,
                        available_builtin_endpoints: ref __self_0_10,
                        manual_liveliness_count: ref __self_0_11,
                        builtin_endpoint_qos: ref __self_0_12,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "ParticipantProxy");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "domain_id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "domain_tag",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "protocol_version",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "guid",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "vendor_id",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "expects_inline_qos",
                            &&(*__self_0_5),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "metatraffic_unicast_locator_list",
                            &&(*__self_0_6),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "metatraffic_multicast_locator_list",
                            &&(*__self_0_7),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "default_unicast_locator_list",
                            &&(*__self_0_8),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "default_multicast_locator_list",
                            &&(*__self_0_9),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "available_builtin_endpoints",
                            &&(*__self_0_10),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "manual_liveliness_count",
                            &&(*__self_0_11),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "builtin_endpoint_qos",
                            &&(*__self_0_12),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        pub struct SPDPdiscoveredParticipantDataUdp {
            participant_proxy: ParticipantProxy,
            lease_duration: DurationUdp,
        }
        impl ::core::marker::StructuralPartialEq for SPDPdiscoveredParticipantDataUdp {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SPDPdiscoveredParticipantDataUdp {
            #[inline]
            fn eq(&self, other: &SPDPdiscoveredParticipantDataUdp) -> bool {
                match *other {
                    SPDPdiscoveredParticipantDataUdp {
                        participant_proxy: ref __self_1_0,
                        lease_duration: ref __self_1_1,
                    } => match *self {
                        SPDPdiscoveredParticipantDataUdp {
                            participant_proxy: ref __self_0_0,
                            lease_duration: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SPDPdiscoveredParticipantDataUdp) -> bool {
                match *other {
                    SPDPdiscoveredParticipantDataUdp {
                        participant_proxy: ref __self_1_0,
                        lease_duration: ref __self_1_1,
                    } => match *self {
                        SPDPdiscoveredParticipantDataUdp {
                            participant_proxy: ref __self_0_0,
                            lease_duration: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SPDPdiscoveredParticipantDataUdp {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    SPDPdiscoveredParticipantDataUdp {
                        participant_proxy: ref __self_0_0,
                        lease_duration: ref __self_0_1,
                    } => {
                        let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(
                            f,
                            "SPDPdiscoveredParticipantDataUdp",
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "participant_proxy",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "lease_duration",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl SPDPdiscoveredParticipantDataUdp {
            const _DEFAULT_DOMAIN_TAG: String = String::new();
            const DEFAULT_EXPECTS_INLINE_QOS: bool = false;
            const _DEFAULT_PARTICIPANT_LEASE_DURATION: DurationUdp = DurationUdp {
                seconds: 100,
                fraction: 0,
            };
            pub fn new(
                domain_id: &DomainId,
                domain_tag: &str,
                protocol_version: &ProtocolVersion,
                guid: &GUID,
                vendor_id: &VendorId,
                expects_inline_qos: &bool,
                metatraffic_unicast_locator_list: &[Locator],
                metatraffic_multicast_locator_list: &[Locator],
                default_unicast_locator_list: &[Locator],
                default_multicast_locator_list: &[Locator],
                available_builtin_endpoints: &BuiltinEndpointSet,
                manual_liveliness_count: &Count,
                builtin_endpoint_qos: &BuiltinEndpointQos,
                lease_duration: &Duration,
            ) -> Self {
                Self {
                    participant_proxy: ParticipantProxy {
                        domain_id: *domain_id,
                        domain_tag: domain_tag.to_owned(),
                        protocol_version: ProtocolVersionUdp::new(protocol_version),
                        guid: GUIDUdp::new(guid),
                        vendor_id: VendorIdUdp::new(vendor_id),
                        expects_inline_qos: *expects_inline_qos,
                        metatraffic_unicast_locator_list: metatraffic_unicast_locator_list
                            .iter()
                            .map(|x| LocatorUdp::new(x))
                            .collect(),
                        metatraffic_multicast_locator_list: metatraffic_multicast_locator_list
                            .iter()
                            .map(|x| LocatorUdp::new(x))
                            .collect(),
                        default_unicast_locator_list: default_unicast_locator_list
                            .iter()
                            .map(|x| LocatorUdp::new(x))
                            .collect(),
                        default_multicast_locator_list: default_multicast_locator_list
                            .iter()
                            .map(|x| LocatorUdp::new(x))
                            .collect(),
                        available_builtin_endpoints: available_builtin_endpoints.0,
                        manual_liveliness_count: CountUdp::new(manual_liveliness_count),
                        builtin_endpoint_qos: builtin_endpoint_qos.0,
                    },
                    lease_duration: DurationUdp::new(lease_duration),
                }
            }
            pub fn to_bytes(&self) -> Result<Vec<u8>, rust_serde_cdr::error::Error> {
                let mut parameter = Vec::new();
                let value =
                    &rust_serde_cdr::serializer::to_bytes(&self.participant_proxy.domain_id)
                        .unwrap();
                parameter.push(ParameterUdp {
                    parameter_id: PID_DOMAIN_ID,
                    length: 4,
                    value,
                });
                let mut bytes = PL_CDR_LE.to_vec();
                rust_serde_cdr::serializer::serialize_into(
                    &ParameterListUdp { parameter },
                    &mut bytes,
                )
                .unwrap();
                Ok(bytes)
            }
            pub fn from_bytes(buf: &[u8]) -> Result<Self, rust_serde_cdr::error::Error> {
                let _representation: [u8; 4] =
                    rust_serde_cdr::deserializer::from_bytes(&buf[0..4])?;
                let parameter_list: ParameterListUdp =
                    rust_serde_cdr::deserializer::from_bytes(&buf[4..])?;
                let domain_id = parameter_list.get(PID_DOMAIN_ID).unwrap();
                let domain_tag = parameter_list.get(PID_DOMAIN_TAG).unwrap();
                let protocol_version: ProtocolVersionUdp =
                    parameter_list.get(PID_PROTOCOL_VERSION).unwrap();
                let guid: GUIDUdp = parameter_list.get(PID_PARTICIPANT_GUID).unwrap();
                let vendor_id: VendorIdUdp = parameter_list.get(PID_VENDORID).unwrap();
                let expects_inline_qos = parameter_list
                    .get(PID_EXPECTS_INLINE_QOS)
                    .unwrap_or(Self::DEFAULT_EXPECTS_INLINE_QOS);
                let metatraffic_unicast_locator_list =
                    parameter_list.get_list(PID_METATRAFFIC_UNICAST_LOCATOR);
                let metatraffic_multicast_locator_list =
                    parameter_list.get_list(PID_METATRAFFIC_MULTICAST_LOCATOR);
                let default_unicast_locator_list =
                    parameter_list.get_list(PID_DEFAULT_UNICAST_LOCATOR);
                let default_multicast_locator_list =
                    parameter_list.get_list(PID_DEFAULT_MULTICAST_LOCATOR);
                let available_builtin_endpoints =
                    parameter_list.get(PID_BUILTIN_ENDPOINT_SET).unwrap();
                let manual_liveliness_count = parameter_list
                    .get(PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT)
                    .unwrap();
                let builtin_endpoint_qos =
                    parameter_list.get(PID_BUILTIN_ENDPOINT_QOS).unwrap_or(0);
                let lease_duration = parameter_list.get(PID_PARTICIPANT_LEASE_DURATION).unwrap();
                let participant_proxy = ParticipantProxy {
                    domain_id,
                    domain_tag,
                    protocol_version,
                    guid,
                    vendor_id,
                    expects_inline_qos,
                    metatraffic_unicast_locator_list,
                    metatraffic_multicast_locator_list,
                    default_unicast_locator_list,
                    default_multicast_locator_list,
                    available_builtin_endpoints,
                    manual_liveliness_count,
                    builtin_endpoint_qos,
                };
                Ok(Self {
                    participant_proxy: participant_proxy,
                    lease_duration: lease_duration,
                })
            }
        }
        impl SPDPdiscoveredParticipantData for SPDPdiscoveredParticipantDataUdp {
            type LocatorListType = Vec<Locator>;
            fn domain_id(&self) -> DomainId {
                self.participant_proxy.domain_id
            }
            fn domain_tag(&self) -> &str {
                &self.participant_proxy.domain_tag
            }
            fn protocol_version(&self) -> ProtocolVersion {
                self.participant_proxy.protocol_version.value()
            }
            fn guid_prefix(&self) -> GuidPrefix {
                *self.participant_proxy.guid.value().prefix()
            }
            fn vendor_id(&self) -> VendorId {
                self.participant_proxy.vendor_id.value()
            }
            fn expects_inline_qos(&self) -> bool {
                self.participant_proxy.expects_inline_qos
            }
            fn metatraffic_unicast_locator_list(&self) -> Self::LocatorListType {
                self.participant_proxy
                    .metatraffic_unicast_locator_list
                    .iter()
                    .map(|x| x.value())
                    .collect()
            }
            fn metatraffic_multicast_locator_list(&self) -> Self::LocatorListType {
                self.participant_proxy
                    .metatraffic_multicast_locator_list
                    .iter()
                    .map(|x| x.value())
                    .collect()
            }
            fn default_unicast_locator_list(&self) -> Self::LocatorListType {
                self.participant_proxy
                    .default_unicast_locator_list
                    .iter()
                    .map(|x| x.value())
                    .collect()
            }
            fn default_multicast_locator_list(&self) -> Self::LocatorListType {
                self.participant_proxy
                    .default_multicast_locator_list
                    .iter()
                    .map(|x| x.value())
                    .collect()
            }
            fn available_builtin_endpoints(&self) -> BuiltinEndpointSet {
                BuiltinEndpointSet(self.participant_proxy.available_builtin_endpoints)
            }
            fn manual_liveliness_count(&self) -> Count {
                self.participant_proxy.manual_liveliness_count.value()
            }
            fn builtin_endpoint_qos(&self) -> BuiltinEndpointQos {
                BuiltinEndpointQos(self.participant_proxy.builtin_endpoint_qos)
            }
        }
    }
    pub mod parameterid_list {
        pub const PID_PAD: u16 = 0x0000;
        pub const PID_SENTINEL: u16 = 0x0001;
        pub const PID_USER_DATA: u16 = 0x002c;
        pub const PID_TOPIC_NAME: u16 = 0x0005;
        pub const PID_TYPE_NAME: u16 = 0x0007;
        pub const PID_GROUP_DATA: u16 = 0x002d;
        pub const PID_TOPIC_DATA: u16 = 0x002e;
        pub const PID_DURABILITY: u16 = 0x001d;
        pub const PID_DURABILITY_SERVICE: u16 = 0x001e;
        pub const PID_DEADLINE: u16 = 0x0023;
        pub const PID_LATENCY_BUDGET: u16 = 0x0027;
        pub const PID_LIVELINESS: u16 = 0x001b;
        pub const PID_RELIABILITY: u16 = 0x001a;
        pub const PID_LIFESPAN: u16 = 0x002b;
        pub const PID_DESTINATION_ORDER: u16 = 0x0025;
        pub const PID_HISTORY: u16 = 0x0040;
        pub const PID_RESOURCE_LIMITS: u16 = 0x0041;
        pub const PID_OWNERSHIP: u16 = 0x001f;
        pub const PID_OWNERSHIP_STRENGTH: u16 = 0x0006;
        pub const PID_PRESENTATION: u16 = 0x0021;
        pub const PID_PARTITION: u16 = 0x0029;
        pub const PID_TIME_BASED_FILTER: u16 = 0x0004;
        pub const PID_TRANSPORT_PRIORITY: u16 = 0x0049;
        pub const PID_DOMAIN_ID: u16 = 0x000f;
        pub const PID_DOMAIN_TAG: u16 = 0x4014;
        pub const PID_PROTOCOL_VERSION: u16 = 0x0015;
        pub const PID_VENDORID: u16 = 0x0016;
        pub const PID_UNICAST_LOCATOR: u16 = 0x002f;
        pub const PID_MULTICAST_LOCATOR: u16 = 0x0030;
        pub const PID_DEFAULT_UNICAST_LOCATOR: u16 = 0x0031;
        pub const PID_DEFAULT_MULTICAST_LOCATOR: u16 = 0x0048;
        pub const PID_METATRAFFIC_UNICAST_LOCATOR: u16 = 0x0032;
        pub const PID_METATRAFFIC_MULTICAST_LOCATOR: u16 = 0x0033;
        pub const PID_EXPECTS_INLINE_QOS: u16 = 0x0043;
        pub const PID_PARTICIPANT_MANUAL_LIVELINESS_COUNT: u16 = 0x0034;
        pub const PID_PARTICIPANT_LEASE_DURATION: u16 = 0x0002;
        pub const PID_CONTENT_FILTER_PROPERTY: u16 = 0x0035;
        pub const PID_PARTICIPANT_GUID: u16 = 0x0050;
        pub const PID_GROUP_GUID: u16 = 0x0052;
        pub const PID_BUILTIN_ENDPOINT_SET: u16 = 0x0058;
        pub const PID_BUILTIN_ENDPOINT_QOS: u16 = 0x0077;
        pub const PID_PROPERTY_LIST: u16 = 0x0059;
        pub const PID_TYPE_MAX_SIZE_SERIALIZED: u16 = 0x0060;
        pub const PID_ENTITY_NAME: u16 = 0x0062;
        pub const PID_ENDPOINT_GUID: u16 = 0x005a;
    }
}
