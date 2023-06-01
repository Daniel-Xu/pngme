use crate::chunk_type::ChunkType;
use anyhow::bail;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
#[allow(dead_code)]

// length: the data length (only the content data)
// chuck_type 4
// data => length
// crc32 for crc(chunk_type + data) -> 4 bytes
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl Chunk {
    fn generate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        return CASTAGNOLI.checksum(&bytes);
    }

    // data here is the real content
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Self::generate_crc(&chunk_type, &data);

        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn build(length: u32, chunk_type: ChunkType, data: Vec<u8>, crc: u32) -> Self {
        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> anyhow::Result<String> {
        // why do we need to use ? here
        // because if the from_utf8 returns error, we need to use the `?`'s From::from to convert
        // the error to anyhow error
        let s = String::from_utf8(self.data.clone())?;

        Ok(s)
    }

    #[allow(dead_code)]
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn read_chunk(reader: &mut BufReader<&[u8]>) -> anyhow::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let len = u32::from_be_bytes(buf);

        reader.read_exact(&mut buf)?;
        let chunk_type = ChunkType::try_from(buf)?;

        let mut data_buf = vec![0; len as usize];
        reader.read_exact(&mut data_buf)?;

        reader.read_exact(&mut buf)?;

        let read_crc = u32::from_be_bytes(buf);
        if Self::generate_crc(&chunk_type, &data_buf) != read_crc {
            bail!("invalid crc32");
        }

        Ok(Self::build(len, chunk_type, data_buf, read_crc))
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "length: {}, type: {}, data: {:?}, crc: {}",
            self.length(),
            self.chunk_type(),
            self.data(),
            self.crc()
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        let chunk = Self::read_chunk(&mut reader)?;

        if !reader.fill_buf()?.is_empty() {
            bail!("Invalid content, there's still data left");
        }

        Ok(chunk)
    }

    // ❌ initial implementation
    // fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    //     // validate total length
    //     if value.len() < 4 {
    //         bail!("not enough length");
    //     }
    //
    //     // let b = value[..1];
    //     let length = u32::from_be_bytes(<[u8; 4]>::try_from(&value[..4])?);
    //
    //     if 4 + 4 + length + 4 != value.len() as u32 {
    //         bail!("invalid length");
    //     }
    //
    //     let chunk_type = ChunkType::try_from(<[u8; 4]>::try_from(&value[4..8])?)?;
    //     // length: the data length (only the data)
    //
    //     // chuck_type 4
    //     // data => length
    //
    //     // crc32 for crc(chunk_type + data) -> 4 bytes
    //
    //     let messages = &value[8..(8 + length as usize)];
    //     let data = chunk_type
    //         .bytes()
    //         .iter()
    //         .chain(messages.iter())
    //         .copied()
    //         .collect::<Vec<u8>>();
    //
    //     let crc = CASTAGNOLI.checksum(data.as_ref());
    //     let input_crc = &value[(8 + length as usize)..];
    //
    //     if input_crc != crc.to_be_bytes() {
    //         bail!("invalid checksum");
    //     }
    //
    //     Ok(Self {
    //         length,
    //         chunk_type,
    //         data: messages.to_vec(),
    //         crc,
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();

        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
