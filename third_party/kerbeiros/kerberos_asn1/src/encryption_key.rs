use crate::Int32;
use red_asn1::{Asn1Object, OctetString};
use red_asn1_derive::Sequence;

/// (*EncryptionKey*) To represent the key of an encryption algorithm.
/// ```asn1
/// EncryptionKey   ::= SEQUENCE {
///         keytype         [0] Int32 -- actually encryption type --,
///         keyvalue        [1] OCTET STRING
/// }
/// ```
///

#[derive(Sequence, Default, Debug, PartialEq, Clone)]
pub struct EncryptionKey {
    #[seq_field(context_tag = 0)]
    pub keytype: Int32,
    #[seq_field(context_tag = 1)]
    pub keyvalue: OctetString,
}

impl EncryptionKey {
    pub fn new(keytype: Int32, keyvalue: OctetString) -> Self {
        return Self { keytype, keyvalue };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use kerberos_constants::etypes::*;

    #[test]
    fn test_parse_encryption_key() {
        let encryption_key = EncryptionKey::new(
            AES256_CTS_HMAC_SHA1_96,
            vec![
                0x63, 0x7b, 0x4d, 0x21, 0x38, 0x22, 0x5a, 0x3a, 0x0a, 0xd7,
                0x93, 0x5a, 0xf3, 0x31, 0x22, 0x68, 0x50, 0xeb, 0x53, 0x1d,
                0x2d, 0x40, 0xf2, 0x19, 0x19, 0xd0, 0x08, 0x41, 0x91, 0x72,
                0x17, 0xff,
            ],
        );

        assert_eq!(
            encryption_key,
            EncryptionKey::parse(&[
                0x30, 0x29, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x22, 0x04,
                0x20, 0x63, 0x7b, 0x4d, 0x21, 0x38, 0x22, 0x5a, 0x3a, 0x0a,
                0xd7, 0x93, 0x5a, 0xf3, 0x31, 0x22, 0x68, 0x50, 0xeb, 0x53,
                0x1d, 0x2d, 0x40, 0xf2, 0x19, 0x19, 0xd0, 0x08, 0x41, 0x91,
                0x72, 0x17, 0xff,
            ])
            .unwrap()
            .1
        );
    }


    #[test]
    fn test_build_encryption_key() {
        let encryption_key = EncryptionKey::new(
            AES256_CTS_HMAC_SHA1_96,
            vec![
                0x63, 0x7b, 0x4d, 0x21, 0x38, 0x22, 0x5a, 0x3a, 0x0a, 0xd7,
                0x93, 0x5a, 0xf3, 0x31, 0x22, 0x68, 0x50, 0xeb, 0x53, 0x1d,
                0x2d, 0x40, 0xf2, 0x19, 0x19, 0xd0, 0x08, 0x41, 0x91, 0x72,
                0x17, 0xff,
            ],
        );

        assert_eq!(
            vec![
                0x30, 0x29, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x22, 0x04,
                0x20, 0x63, 0x7b, 0x4d, 0x21, 0x38, 0x22, 0x5a, 0x3a, 0x0a,
                0xd7, 0x93, 0x5a, 0xf3, 0x31, 0x22, 0x68, 0x50, 0xeb, 0x53,
                0x1d, 0x2d, 0x40, 0xf2, 0x19, 0x19, 0xd0, 0x08, 0x41, 0x91,
                0x72, 0x17, 0xff,
            ],
            encryption_key.build()
        );
    }
}
