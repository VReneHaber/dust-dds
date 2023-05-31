use crate::implementation::{
    rtps::messages::overall_structure::SubmessageHeaderWrite,
    rtps_udp_psm::mapping_traits::MappingWriteByteOrderInfoInData,
};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::{Error, Write};

pub trait MappingWriteSubmessage {
    fn submessage_header(&self) -> SubmessageHeaderWrite;
    fn mapping_write_submessage_elements<W: Write, B: ByteOrder>(
        &self,
        writer: W,
    ) -> Result<(), Error>;
}

impl<T> MappingWriteByteOrderInfoInData for T
where
    T: MappingWriteSubmessage,
{
    fn mapping_write_byte_order_info_in_data<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        self.submessage_header()
            .mapping_write_byte_order_info_in_data(&mut writer)?;
        if self.submessage_header().flags[0] {
            self.mapping_write_submessage_elements::<_, LittleEndian>(&mut writer)
        } else {
            self.mapping_write_submessage_elements::<_, BigEndian>(&mut writer)
        }
    }
}
