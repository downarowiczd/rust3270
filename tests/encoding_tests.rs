#[cfg(test)]
mod tests {
    use rust3270::encoding::{Encoding, decode_to_ascii, encode_ascii_to};

    fn encoding_under_test() -> Encoding {
        Encoding::CP037
    }

    fn check_ascii_letters_encoding(encoding: &Encoding) {
        let input = ['A', 'Z', 'a', 'z'];
        let expected = [0xC1, 0xE9, 0x81, 0xA9];
        let result: Vec<u8> = encode_ascii_to(input.iter().copied(), encoding).collect();
        assert_eq!(result, expected);
    }

    fn check_ascii_letters_decoding(encoding: &Encoding) {
        let input = [0xC1, 0xE9, 0x81, 0xA9];
        let expected = ['A', 'Z', 'a', 'z'];
        let result: Vec<char> = decode_to_ascii(input.iter().copied(), encoding).collect();
        assert_eq!(result, expected);
    }

    fn check_unmappable_char(encoding: &Encoding) {
        let input = ['☃', 'Ł', '€'];
        let result: Vec<u8> = encode_ascii_to(input.iter().copied(), encoding).collect();
        assert!(result.iter().all(|&b| b == 0x40));
    }

    fn check_encode_decode_round_trip_ascii(encoding: &Encoding) {
        for ch in 0x20u8..=0x7Eu8 {
            let c = ch as char;
            let encoded: Vec<u8> = encode_ascii_to(std::iter::once(c), encoding).collect();
            let decoded: Vec<char> = decode_to_ascii(encoded.iter().copied(), encoding).collect();
            assert_eq!(decoded[0], c, "Round-trip failed for '{}'", c);
        }
    }

    fn check_encode_table_consistency(encoding: &Encoding) {
        let tbl = encoding.encode_table();
        for (i, &b) in tbl.iter().enumerate() {
            if i >= 0x20 && i <= 0x7E {
                let c = i as u8 as char;
                let encoded: Vec<u8> = encode_ascii_to(std::iter::once(c), encoding).collect();
                assert_eq!(encoded[0], b, "Encoding mismatch for '{}'", c);
            }
        }
    }

    fn check_decode_table_consistency(encoding: &Encoding) {
        let tbl = encoding.decode_table();
        for (i, &b) in tbl.iter().enumerate() {
            let decoded: Vec<char> = decode_to_ascii(std::iter::once(i as u8), encoding).collect();
            assert_eq!(decoded[0] as u8, b, "Decoding mismatch for byte 0x{:02X}", i);
        }
    }

    fn check_decode_invalid_bytes(encoding: &Encoding) {
        let invalid_bytes = [0x00, 0xFF, 0x9C, 0x9D];
        let decoded: Vec<char> = decode_to_ascii(invalid_bytes.iter().copied(), encoding).collect();
        assert_eq!(decoded.len(), invalid_bytes.len());
    }

    #[test]
    fn test_ascii_letters_encoding() {
        check_ascii_letters_encoding(&encoding_under_test());
    }

    #[test]
    fn test_ascii_letters_decoding() {
        check_ascii_letters_decoding(&encoding_under_test());
    }

    #[test]
    fn test_unmappable_char() {
        check_unmappable_char(&encoding_under_test());
    }

    #[test]
    fn test_encode_decode_round_trip_ascii() {
        check_encode_decode_round_trip_ascii(&encoding_under_test());
    }

    #[test]
    fn test_encode_table_consistency() {
        check_encode_table_consistency(&encoding_under_test());
    }

    #[test]
    fn test_decode_table_consistency() {
        check_decode_table_consistency(&encoding_under_test());
    }

    #[test]
    fn test_decode_invalid_bytes() {
        check_decode_invalid_bytes(&encoding_under_test());
    }
}
