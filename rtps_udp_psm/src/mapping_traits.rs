use std::io::{Error, Write};

use byteorder::{ByteOrder, LittleEndian};

pub trait MappingWrite {
    fn mapping_write<W: Write>(&self, writer: W) -> Result<(), Error>;
}

pub trait MappingWriteByteOrdered {
    fn mapping_write_byte_ordered<W: Write, B: ByteOrder>(&self, writer: W) -> Result<(), Error>;
}

pub trait NumberOfBytes {
    fn number_of_bytes(&self) -> usize;
}

pub trait MappingRead<'de>: Sized {
    fn mapping_read(buf: &mut &'de [u8]) -> Result<Self, Error>;
}

pub trait MappingReadByteOrdered<'de>: Sized {
    fn mapping_read_byte_ordered<B>(buf: &mut &'de [u8]) -> Result<Self, Error>
    where
        B: ByteOrder;
}

pub fn to_bytes_le<S: MappingWriteByteOrdered>(value: &S) -> Result<Vec<u8>, Error> {
    let mut writer = Vec::<u8>::new();
    value.mapping_write_byte_ordered::<_, LittleEndian>(&mut writer)?;
    Ok(writer)
}

pub fn to_bytes<S: MappingWrite>(value: &S) -> Result<Vec<u8>, Error> {
    let mut writer = Vec::<u8>::new();
    value.mapping_write(&mut writer)?;
    Ok(writer)
}

pub fn from_bytes_le<'de, D: MappingReadByteOrdered<'de>>(mut buf: &'de [u8]) -> Result<D, Error> {
    D::mapping_read_byte_ordered::<LittleEndian>(&mut buf)
}

pub fn from_bytes<'de, D: MappingRead<'de>>(mut buf: &'de [u8]) -> Result<D, Error> {
    D::mapping_read(&mut buf)
}
