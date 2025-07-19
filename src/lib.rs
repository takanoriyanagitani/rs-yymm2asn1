use std::io;

use der::Choice;
use der::Encode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Choice)]
pub enum DateYyMmCompactInfer {
    #[asn1(tag_mode = "IMPLICIT", context_specific = "1")]
    Jan(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "2")]
    Feb(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "3")]
    Mar(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "4")]
    Apr(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "5")]
    May(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "6")]
    Jun(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "7")]
    Jul(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "8")]
    Aug(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "9")]
    Sep(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "10")]
    Oct(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "11")]
    Nov(u8),
    #[asn1(tag_mode = "IMPLICIT", context_specific = "12")]
    Dec(u8),
}

impl DateYyMmCompactInfer {
    pub fn to_der_bytes(&self) -> Result<Vec<u8>, io::Error> {
        self.to_der().map_err(io::Error::other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use der::Decode;

    #[test]
    fn test_jan_serialization() {
        let original = DateYyMmCompactInfer::Jan(25);
        let der_bytes = original.to_der_bytes().unwrap();
        assert_eq!(der_bytes, &[0x81, 0x01, 0x19]);

        let decoded = DateYyMmCompactInfer::from_der(&der_bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_dec_serialization() {
        let original = DateYyMmCompactInfer::Dec(99);
        let der_bytes = original.to_der_bytes().unwrap();
        assert_eq!(der_bytes, &[0x8C, 0x01, 0x63]);

        let decoded = DateYyMmCompactInfer::from_der(&der_bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
