use std::{array::TryFromSliceError, fmt, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type_array = ChunkType { chunk_type: value };
        return Ok(chunk_type_array);
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Result<[u8; 4], TryFromSliceError> = s.as_bytes()[..4].try_into();
        match s {
            Ok(value) => {
                for byte in value {
                    if !byte.is_ascii_alphabetic() {
                        return Err("Not valid byte chunk");
                    }
                }
                return Ok(ChunkType { chunk_type: value });
            }
            Err(_) => {
                return Err("string slice failed to convert to bytes");
            }
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chunk_type_string = std::str::from_utf8(&self.chunk_type).unwrap();
        write!(f, "{}", chunk_type_string)
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        return self.chunk_type.clone();
    }
    fn is_valid(&self) -> bool {
        let third_byte_fifth_bit = self.chunk_type[2];
        return third_byte_fifth_bit.is_ascii_uppercase();
    }
    fn is_critical(&self) -> bool {
        let first_byte_fifth_bit = self.chunk_type[0];
        return first_byte_fifth_bit.is_ascii_uppercase();
    }
    fn is_public(&self) -> bool {
        let second_byte_fifth_bit = self.chunk_type[1];
        return second_byte_fifth_bit.is_ascii_uppercase();
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let third_byte_fifth_bit = self.chunk_type[2];
        return third_byte_fifth_bit.is_ascii_uppercase();
    }
    fn is_safe_to_copy(&self) -> bool {
        let forth_byte_fifth_bit = self.chunk_type[3];
        return !forth_byte_fifth_bit.is_ascii_uppercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

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

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

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
