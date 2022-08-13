use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub fn hmac_md5(key: &[u8], data: &[u8]) -> Vec<u8> {
    return hmac(key, data, Md5::new());
}

pub fn hmac_sha1(key: &[u8], data: &[u8]) -> Vec<u8> {
    return hmac(key, data, Sha1::new());
}

pub fn hmac<D: Digest>(key: &[u8], data: &[u8], d: D) -> Vec<u8> {
    let mut hmacker = Hmac::new(d, key);
    hmacker.input(data);
    return hmacker.result().code().to_vec();
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_hmac_md5() {
        assert_eq!(
            vec![
                0x2d, 0xc0, 0x9b, 0x8b, 0x35, 0xaf, 0x9c, 0x03, 0x6f, 0xc3,
                0xf2, 0x9c, 0xdb, 0xc0, 0x5f, 0xbb
            ],
            hmac_md5(&[0x61, 0x64, 0x6d, 0x69, 0x6e], &[])
        );
        assert_eq!(
            vec![
                0x9a, 0x06, 0x98, 0xf1, 0xb4, 0x8b, 0xc6, 0x4c, 0x95, 0xcf,
                0xf7, 0x4b, 0xf4, 0x69, 0x16, 0x39
            ],
            hmac_md5(
                &[0x61, 0x64, 0x6d, 0x69, 0x6e],
                &[0x61, 0x62, 0x63, 0x64]
            )
        );
        assert_eq!(
            vec![
                0x59, 0x51, 0xa7, 0xa9, 0x11, 0xb8, 0x9b, 0xfb, 0x36, 0x18,
                0x43, 0xbb, 0xa9, 0x8f, 0xfe, 0x54
            ],
            hmac_md5(
                &[0x61, 0x64, 0x6d, 0x69, 0x6e],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0xaa, 0x21, 0xa8, 0xa4, 0x23, 0xd6, 0x60, 0xa6, 0x58, 0xd6,
                0x1a, 0x86, 0xc8, 0xa9, 0x4e, 0xeb
            ],
            hmac_md5(
                &[0x61, 0x64, 0x6d, 0x69, 0x6e],
                &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b]
            )
        );
        assert_eq!(
            vec![
                0x22, 0x4d, 0xcb, 0x99, 0x1b, 0x06, 0x35, 0x5b, 0x82, 0x77,
                0x8d, 0x74, 0x18, 0xad, 0xd0, 0xcf
            ],
            hmac_md5(
                &[0x61, 0x64, 0x6d, 0x69, 0x6e],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
        assert_eq!(
            vec![
                0x6a, 0xeb, 0xb4, 0x1d, 0x43, 0x66, 0xc2, 0x79, 0x46, 0x87,
                0xe1, 0xd9, 0xe7, 0xae, 0xe3, 0x07
            ],
            hmac_md5(&[0x74, 0x65, 0x73, 0x74], &[])
        );
        assert_eq!(
            vec![
                0x80, 0x33, 0x46, 0x1a, 0xb9, 0x1d, 0xf1, 0x61, 0xb4, 0x06,
                0x62, 0x71, 0xd0, 0x2d, 0x3f, 0x82
            ],
            hmac_md5(&[0x74, 0x65, 0x73, 0x74], &[0x61, 0x62, 0x63, 0x64])
        );
        assert_eq!(
            vec![
                0x45, 0x25, 0xbd, 0x9d, 0xcc, 0x2a, 0xaa, 0xb4, 0x86, 0x60,
                0x4c, 0x46, 0x52, 0xf8, 0x6e, 0xc3
            ],
            hmac_md5(
                &[0x74, 0x65, 0x73, 0x74],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0xb4, 0xd5, 0x9f, 0xcd, 0x1e, 0xf1, 0xf1, 0x2a, 0x1a, 0xc5,
                0xa4, 0x11, 0x2d, 0x5e, 0x1e, 0xc1
            ],
            hmac_md5(
                &[0x74, 0x65, 0x73, 0x74],
                &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b]
            )
        );
        assert_eq!(
            vec![
                0xaf, 0x95, 0xcb, 0x8e, 0xf6, 0x07, 0x0d, 0x12, 0x03, 0x9c,
                0x68, 0xe2, 0xbe, 0xb5, 0xe2, 0xf2
            ],
            hmac_md5(
                &[0x74, 0x65, 0x73, 0x74],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
        assert_eq!(
            vec![
                0x16, 0xa9, 0x72, 0x2d, 0x9b, 0x34, 0x8e, 0xfe, 0x74, 0x46,
                0x23, 0xbd, 0x97, 0x1c, 0x35, 0x89
            ],
            hmac_md5(&[0x31, 0x33, 0x33, 0x37], &[])
        );
        assert_eq!(
            vec![
                0x79, 0x8a, 0xac, 0x10, 0xf4, 0xc3, 0x91, 0x86, 0x47, 0xea,
                0x92, 0x36, 0x73, 0x8b, 0xf3, 0x25
            ],
            hmac_md5(&[0x31, 0x33, 0x33, 0x37], &[0x61, 0x62, 0x63, 0x64])
        );
        assert_eq!(
            vec![
                0xbb, 0xd0, 0x9d, 0x84, 0xeb, 0x12, 0xcc, 0x2e, 0x4a, 0xa0,
                0x10, 0xea, 0x16, 0xa8, 0xc8, 0xa9
            ],
            hmac_md5(
                &[0x31, 0x33, 0x33, 0x37],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0x4a, 0x60, 0x81, 0x8f, 0x1a, 0x8e, 0xfa, 0x3a, 0x15, 0xbb,
                0x6c, 0x28, 0xf5, 0x75, 0x59, 0x43
            ],
            hmac_md5(
                &[0x31, 0x33, 0x33, 0x37],
                &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b]
            )
        );
        assert_eq!(
            vec![
                0x87, 0xa8, 0x59, 0x7a, 0x75, 0x0b, 0xc8, 0x7f, 0x58, 0x73,
                0xaa, 0xd6, 0x4a, 0x3c, 0xa0, 0x8f
            ],
            hmac_md5(
                &[0x31, 0x33, 0x33, 0x37],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
        assert_eq!(
            vec![
                0x74, 0xe6, 0xf7, 0x29, 0x8a, 0x9c, 0x2d, 0x16, 0x89, 0x35,
                0xf5, 0x8c, 0x00, 0x1b, 0xad, 0x88
            ],
            hmac_md5(&[], &[])
        );
        assert_eq!(
            vec![
                0xea, 0x1c, 0x52, 0x6c, 0x3d, 0x89, 0x5e, 0xeb, 0x84, 0x98,
                0x29, 0x8b, 0x13, 0xf1, 0x08, 0x96
            ],
            hmac_md5(&[], &[0x61, 0x62, 0x63, 0x64])
        );
        assert_eq!(
            vec![
                0x6c, 0xb9, 0x30, 0xb9, 0x66, 0x9a, 0x83, 0x2f, 0x2a, 0xce,
                0x4a, 0xeb, 0x03, 0xba, 0xfc, 0x42
            ],
            hmac_md5(
                &[],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0xcd, 0x2e, 0x36, 0xdc, 0x8a, 0xdd, 0x03, 0xc6, 0x97, 0xe9,
                0x31, 0x8d, 0x10, 0x9f, 0xb0, 0x9b
            ],
            hmac_md5(&[], &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b])
        );
        assert_eq!(
            vec![
                0xa9, 0x65, 0x79, 0x47, 0x7e, 0x2f, 0x69, 0x3f, 0xa4, 0x3d,
                0x1b, 0xc5, 0xa0, 0x59, 0x32, 0x62
            ],
            hmac_md5(
                &[],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
        assert_eq!(
            vec![
                0x06, 0x60, 0xbe, 0x5c, 0xda, 0xc0, 0x8e, 0x3f, 0xf3, 0xd4,
                0x94, 0x49, 0xce, 0xf9, 0xab, 0xd0
            ],
            hmac_md5(&[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38], &[])
        );
        assert_eq!(
            vec![
                0x62, 0x6a, 0x38, 0x15, 0xb0, 0x95, 0xa6, 0x8b, 0xf5, 0x7d,
                0x54, 0x39, 0xf6, 0x0c, 0x27, 0x6c
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38],
                &[0x61, 0x62, 0x63, 0x64]
            )
        );
        assert_eq!(
            vec![
                0x7e, 0x54, 0xa2, 0x51, 0x3d, 0x39, 0xb4, 0x9b, 0xfd, 0xc2,
                0xe8, 0x67, 0x21, 0xe3, 0x46, 0x48
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0x63, 0xb8, 0x7e, 0xa3, 0x2a, 0x07, 0xf8, 0x45, 0x66, 0xca,
                0xdf, 0xcb, 0x6d, 0xa0, 0x33, 0x9a
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38],
                &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b]
            )
        );
        assert_eq!(
            vec![
                0x05, 0x35, 0x0c, 0x93, 0xe5, 0xc3, 0xdc, 0x7f, 0xa5, 0xde,
                0x58, 0x54, 0x68, 0xa9, 0xa3, 0x50
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
        assert_eq!(
            vec![
                0x4c, 0x94, 0x72, 0x8a, 0x36, 0x46, 0xdd, 0x34, 0x68, 0x93,
                0x4e, 0xd1, 0xcc, 0xf6, 0x62, 0xca
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39],
                &[]
            )
        );
        assert_eq!(
            vec![
                0xa2, 0x75, 0x6b, 0x80, 0xaa, 0x4e, 0x84, 0xf0, 0xce, 0xe0,
                0x2e, 0xf5, 0x92, 0xba, 0x87, 0x27
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39],
                &[0x61, 0x62, 0x63, 0x64]
            )
        );
        assert_eq!(
            vec![
                0xf9, 0x9c, 0xd9, 0x33, 0xf7, 0x5e, 0xd5, 0x7d, 0x0c, 0xec,
                0x03, 0x1d, 0x2a, 0x18, 0xd7, 0xbc
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39],
                &[0x71, 0x77, 0x65, 0x72, 0x74, 0x79, 0x75, 0x69, 0x6f, 0x70]
            )
        );
        assert_eq!(
            vec![
                0x5b, 0xd2, 0xae, 0x8f, 0xee, 0x7c, 0xf9, 0xce, 0x22, 0x58,
                0x9c, 0x3f, 0xab, 0xc1, 0x84, 0xcf
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39],
                &[0x61, 0x73, 0x64, 0x66, 0x67, 0x68, 0x6a, 0x6b]
            )
        );
        assert_eq!(
            vec![
                0xef, 0x6a, 0x67, 0xfc, 0xa3, 0xca, 0x20, 0x5f, 0x3c, 0x30,
                0xfc, 0x1a, 0x04, 0x5a, 0xe5, 0x4b
            ],
            hmac_md5(
                &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39],
                &[0x7a, 0x78, 0x63, 0x76, 0x62, 0x6e, 0x6d, 0x2c, 0x2e]
            )
        );
    }

    #[test]
    fn test_hmac_sha1() {
        assert_eq!(
            vec![
                0x91, 0xf1, 0x85, 0x91, 0x4c, 0x1e, 0xa5, 0x02, 0xfb, 0x17,
                0x4d, 0x47, 0x3b, 0x51, 0x6a, 0x9a, 0x57, 0x07, 0xce, 0x66
            ],
            hmac_sha1("admin".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0x55, 0x7b, 0x01, 0xd4, 0x91, 0x88, 0x4f, 0x58, 0xcc, 0x92,
                0xcd, 0x8e, 0x43, 0x33, 0x8d, 0xe3, 0x1b, 0x22, 0xd1, 0x28
            ],
            hmac_sha1("admin".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0x8c, 0xc5, 0x32, 0x6b, 0xdd, 0x12, 0xe7, 0xb4, 0x3e, 0x4d,
                0x77, 0x1c, 0x8e, 0x89, 0x49, 0xe8, 0xae, 0x93, 0x8f, 0x13
            ],
            hmac_sha1("admin".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0x71, 0x32, 0x0d, 0x9c, 0x9a, 0xe0, 0xdb, 0x4b, 0x21, 0xda,
                0x2c, 0x5b, 0x7b, 0xf9, 0x66, 0x0b, 0x5c, 0xbd, 0x93, 0xed
            ],
            hmac_sha1("admin".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0x1c, 0x8c, 0x2c, 0xc8, 0x68, 0x66, 0xf8, 0x1b, 0x0b, 0xa5,
                0x83, 0x28, 0x7e, 0x2f, 0x66, 0x88, 0xe4, 0xeb, 0xda, 0xec
            ],
            hmac_sha1("admin".as_bytes(), "zxcvbnm,.".as_bytes())
        );
        assert_eq!(
            vec![
                0xfc, 0x85, 0x08, 0x74, 0x52, 0x69, 0x6e, 0x5b, 0xcb, 0xe3,
                0xb7, 0xa7, 0x1f, 0xde, 0x00, 0xe3, 0x20, 0xaf, 0x2c, 0xca
            ],
            hmac_sha1("test".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0xbc, 0xde, 0x77, 0x2e, 0xaf, 0xae, 0x46, 0x5f, 0xe0, 0x02,
                0xb2, 0xbb, 0xb5, 0xc6, 0x8d, 0x66, 0xda, 0x0d, 0x24, 0x1a
            ],
            hmac_sha1("test".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0x39, 0xe2, 0x3b, 0x43, 0xd6, 0xf4, 0xe2, 0xca, 0x5c, 0xa0,
                0x1c, 0x71, 0x12, 0xaa, 0x8c, 0x52, 0xe8, 0xd0, 0x0b, 0x89
            ],
            hmac_sha1("test".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0x70, 0x18, 0xc0, 0x78, 0xb7, 0xf7, 0xab, 0xa5, 0xf7, 0x62,
                0xe8, 0xf8, 0xad, 0xa8, 0x2b, 0xb5, 0x7d, 0x85, 0x23, 0x5e
            ],
            hmac_sha1("test".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0x06, 0x88, 0xdf, 0xf7, 0x7c, 0x04, 0x09, 0xae, 0x5e, 0xbd,
                0xb2, 0x02, 0x6d, 0xc9, 0x76, 0xdd, 0x5a, 0x37, 0x57, 0x91
            ],
            hmac_sha1("test".as_bytes(), "zxcvbnm,.".as_bytes())
        );
        assert_eq!(
            vec![
                0xc1, 0x11, 0x2b, 0x71, 0x05, 0x09, 0x5f, 0x6e, 0x9c, 0x7e,
                0x4f, 0x22, 0x96, 0x96, 0xa4, 0xce, 0x39, 0x3c, 0x6e, 0xf8
            ],
            hmac_sha1("1337".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0x8d, 0x8e, 0x83, 0x50, 0x86, 0x2f, 0x3a, 0xaa, 0x31, 0x4d,
                0xfd, 0xda, 0x16, 0x50, 0x0e, 0x3b, 0x82, 0xe0, 0x14, 0xd2
            ],
            hmac_sha1("1337".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0x3f, 0xbe, 0xbd, 0x31, 0x58, 0xea, 0x65, 0xd2, 0x0c, 0x71,
                0x2f, 0xe2, 0x1d, 0xf3, 0xbb, 0x7f, 0x3c, 0x1b, 0x6b, 0x25
            ],
            hmac_sha1("1337".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0x9c, 0xe4, 0xc3, 0x57, 0xe4, 0x5e, 0x99, 0xb3, 0xa9, 0x56,
                0x17, 0x14, 0x7a, 0x85, 0x60, 0x53, 0xd1, 0xaf, 0xb3, 0x49
            ],
            hmac_sha1("1337".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0x5d, 0x5d, 0x2a, 0x67, 0xe9, 0x79, 0x48, 0x4c, 0x76, 0x67,
                0x69, 0x03, 0x25, 0xac, 0xb5, 0x74, 0xcd, 0x5b, 0xc7, 0xef
            ],
            hmac_sha1("1337".as_bytes(), "zxcvbnm,.".as_bytes())
        );
        assert_eq!(
            vec![
                0xfb, 0xdb, 0x1d, 0x1b, 0x18, 0xaa, 0x6c, 0x08, 0x32, 0x4b,
                0x7d, 0x64, 0xb7, 0x1f, 0xb7, 0x63, 0x70, 0x69, 0x0e, 0x1d
            ],
            hmac_sha1("".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0xaf, 0xa2, 0x9a, 0xb8, 0x53, 0x44, 0x95, 0x25, 0x1a, 0xc8,
                0x34, 0x6a, 0x98, 0x57, 0x17, 0xc5, 0x4b, 0xc4, 0x9c, 0x26
            ],
            hmac_sha1("".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0x9d, 0x16, 0xc0, 0x21, 0x90, 0xe7, 0x4f, 0x9d, 0x66, 0x49,
                0xdb, 0xcb, 0xa9, 0xfa, 0xc1, 0x36, 0x91, 0x02, 0xbd, 0x21
            ],
            hmac_sha1("".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0xc2, 0x91, 0xa4, 0x35, 0xb8, 0xa5, 0x23, 0xa0, 0xff, 0x72,
                0xb7, 0x92, 0x9c, 0x3a, 0x17, 0x30, 0x88, 0x5e, 0x7b, 0x3d
            ],
            hmac_sha1("".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0x71, 0x46, 0x1b, 0x7f, 0x2d, 0x8c, 0xe9, 0xe1, 0xbc, 0xfa,
                0xbe, 0x80, 0xc7, 0x76, 0xea, 0xd0, 0xf5, 0x98, 0x24, 0x94
            ],
            hmac_sha1("".as_bytes(), "zxcvbnm,.".as_bytes())
        );
        assert_eq!(
            vec![
                0x6d, 0xe4, 0x20, 0x54, 0x0d, 0x0a, 0x34, 0xce, 0x16, 0xb4,
                0x27, 0xba, 0xf1, 0xf8, 0xf6, 0x0c, 0x49, 0xf7, 0x59, 0x65
            ],
            hmac_sha1("12345678".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0x34, 0xdc, 0x03, 0xf9, 0x16, 0x4a, 0x45, 0xf4, 0x8b, 0xaa,
                0x72, 0x6f, 0xa1, 0x6a, 0xc8, 0x0a, 0xf0, 0x3f, 0x85, 0x0a
            ],
            hmac_sha1("12345678".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0x97, 0xc7, 0x29, 0xb7, 0xc5, 0x9e, 0x97, 0xce, 0xb6, 0x42,
                0x26, 0x3c, 0xaf, 0x85, 0x75, 0xc8, 0x1f, 0xbd, 0xf5, 0xda
            ],
            hmac_sha1("12345678".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0x4d, 0xf8, 0x71, 0xac, 0x80, 0x6a, 0x70, 0x2b, 0x7d, 0xac,
                0xfa, 0x80, 0x93, 0xc9, 0x7a, 0x2d, 0x6f, 0xe9, 0x23, 0x70
            ],
            hmac_sha1("12345678".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0x11, 0x95, 0xb0, 0xa5, 0x31, 0x72, 0x16, 0x35, 0x05, 0x86,
                0x1d, 0xb2, 0x25, 0xe3, 0xa7, 0x9e, 0xaa, 0xcb, 0x2d, 0x63
            ],
            hmac_sha1("12345678".as_bytes(), "zxcvbnm,.".as_bytes())
        );
        assert_eq!(
            vec![
                0xed, 0xa2, 0x0f, 0xbb, 0x84, 0x5a, 0xbb, 0x1d, 0xb2, 0x50,
                0xdf, 0x4b, 0xb5, 0xc8, 0xc1, 0x5f, 0x20, 0xaf, 0x88, 0xaf
            ],
            hmac_sha1("123456789".as_bytes(), "".as_bytes())
        );
        assert_eq!(
            vec![
                0xa1, 0x0a, 0x96, 0x6b, 0x60, 0xd6, 0x05, 0x26, 0x89, 0xfd,
                0x41, 0x44, 0xad, 0xbc, 0xf1, 0xa3, 0x10, 0xf2, 0x4a, 0xa2
            ],
            hmac_sha1("123456789".as_bytes(), "abcd".as_bytes())
        );
        assert_eq!(
            vec![
                0xa4, 0x07, 0xab, 0x58, 0xe8, 0xbc, 0xf7, 0xd2, 0xfd, 0xaa,
                0xb7, 0x76, 0x0d, 0x25, 0x2a, 0xa5, 0x0c, 0x4d, 0xd5, 0x7b
            ],
            hmac_sha1("123456789".as_bytes(), "qwertyuiop".as_bytes())
        );
        assert_eq!(
            vec![
                0xfa, 0x89, 0x29, 0xf8, 0xd8, 0x9d, 0xd0, 0x4f, 0x92, 0xef,
                0x3d, 0x26, 0xfa, 0xb3, 0xeb, 0x9a, 0xc2, 0x2e, 0x26, 0x3a
            ],
            hmac_sha1("123456789".as_bytes(), "asdfghjk".as_bytes())
        );
        assert_eq!(
            vec![
                0xbd, 0x5d, 0x0d, 0x39, 0x46, 0xaa, 0x2c, 0x70, 0x41, 0xe3,
                0x16, 0x34, 0xe9, 0x90, 0x48, 0xc2, 0x54, 0xad, 0x30, 0x8c
            ],
            hmac_sha1("123456789".as_bytes(), "zxcvbnm,.".as_bytes())
        );
    }
}
