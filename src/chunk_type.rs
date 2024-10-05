use std::convert::TryFrom;
use std::str::FromStr;
use::std::fmt::Display;
use::std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    chunk_type: [u8; 4]
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        for &byte in &value {
            if !byte.is_ascii_alphabetic() {
                return Err(format!("Invalid byte: {}", byte).into());
            }
        }
        Ok(ChunkType { chunk_type: value })
    }
}

impl FromStr for ChunkType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err("Chunk type must be exactly 4 bytes.".into());
        }
        let array: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        ChunkType::try_from(array)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.chunk_type).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }
    pub fn is_critical(&self) -> bool {
        self.chunk_type[0].is_ascii_uppercase()
    }
    pub fn is_valid(&self) -> bool {
        self.chunk_type.iter().all(|&b| b.is_ascii_alphabetic()) 
    }
    pub fn is_public(&self) -> bool {
        self.chunk_type[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        !self.chunk_type[3].is_ascii_uppercase()
    }
}

pub fn test(){
   let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
   println!("{:?}", expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    // use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        println!("{:?}", expected);

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());

    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

