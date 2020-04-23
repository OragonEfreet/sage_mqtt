use crate::{Error, ReadTwoByteInteger, Result as SageResult};
use std::io::{Error as IOError, ErrorKind, Read, Write};

pub trait WriteBinaryData {
    fn write_binary_data<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteBinaryData for Vec<u8> {
    fn write_binary_data<W>(self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let len = self.len();
        if len > i16::max_value() as usize {
            return Err(IOError::new(ErrorKind::InvalidData, "ERROR_MSG_DATA_TOO_LONG").into());
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(&self)?;
        Ok(2 + len)
    }
}

pub trait ReadBinaryData: Sized {
    fn read_binary_data<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadBinaryData for Vec<u8> {
    fn read_binary_data<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size = u16::read_two_byte_integer(&mut chunk)? as usize;

        let mut data_buffer: Vec<u8> = Vec::with_capacity(size);
        if size > 0 {
            let mut chunk = reader.take(size as u64);
            match chunk.read_to_end(&mut data_buffer) {
                Ok(n) if n == size => Ok(data_buffer),
                _ => Err(Error::MalformedPacket),
            }
        } else {
            Ok(Default::default())
        }
    }
}

#[cfg(test)]
mod unit_codec {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode_binarydata() {
        let mut result = Vec::new();
        assert_eq!(
            Vec::from("A𪛔".as_bytes())
                .write_binary_data(&mut result)
                .unwrap(),
            7
        );
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_binarydata_empty() {
        let mut result = Vec::new();
        assert_eq!(Vec::new().write_binary_data(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[test]
    fn decode_binary_data() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            Vec::read_binary_data(&mut test_stream).unwrap(),
            Vec::from("A𪛔".as_bytes())
        );
    }

    #[test]
    fn decode_binary_data_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(Vec::read_binary_data(&mut test_stream).unwrap(), Vec::new());
    }

    #[test]
    fn decode_binary_data_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            Vec::read_binary_data(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
