use std::io::{Error, Write};

use byteorder::ByteOrder;
use dds_transport::messages::submessage_elements::SequenceNumberSubmessageElement;

use crate::mapping_traits::{MappingReadByteOrdered, MappingWriteByteOrdered};

impl MappingWriteByteOrdered for SequenceNumberSubmessageElement {
    fn mapping_write_byte_ordered<W: Write, B: ByteOrder>(
        &self,
        mut writer: W,
    ) -> Result<(), Error> {
        let high = (self.value >> 32) as i32;
        let low = self.value as i32;
        high.mapping_write_byte_ordered::<_, B>(&mut writer)?;
        low.mapping_write_byte_ordered::<_, B>(&mut writer)?;
        Ok(())
    }
}

impl<'de> MappingReadByteOrdered<'de> for SequenceNumberSubmessageElement {
    fn mapping_read_byte_ordered<B: ByteOrder>(buf: &mut &'de [u8]) -> Result<Self, Error> {
        let high: i32 = MappingReadByteOrdered::mapping_read_byte_ordered::<B>(buf)?;
        let low: i32 = MappingReadByteOrdered::mapping_read_byte_ordered::<B>(buf)?;
        let value = ((high as i64) << 32) + low as i64;
        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping_traits::{from_bytes_le, to_bytes_le};

    #[test]
    fn serialize_sequence_number() {
        let data = SequenceNumberSubmessageElement { value: 7 };
        assert_eq!(
            to_bytes_le(&data).unwrap(),
            vec![
                0, 0, 0, 0, // high (long)
                7, 0, 0, 0, // low (unsigned long)
            ]
        );
    }

    #[test]
    fn deserialize_sequence_number() {
        let expected = SequenceNumberSubmessageElement { value: 7 };
        assert_eq!(
            expected,
            from_bytes_le(&[
                0, 0, 0, 0, // high (long)
                7, 0, 0, 0, // low (unsigned long)
            ])
            .unwrap()
        );
    }
}
