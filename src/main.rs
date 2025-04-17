fn main() {
    println!("Hello, world!");
}
fn encode_bytes(bytes: &mut Vec<u8>) {
    for byte in bytes.iter_mut() {
        *byte = (*byte >> 4) | (*byte << 4);
    }
}

fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let input = input.as_ref();
    let mut output = String::new();

    let mut i = 0;
    while i < input.len() {
        let b1 = input[i];
        let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        // Encode 6-bit groups
        output.push(BASE64_CHARS[(b1 >> 2) as usize] as char);
        output.push(BASE64_CHARS[(((b1 & 0b0000_0011) << 4) | (b2 >> 4)) as usize] as char);
        if i + 1 < input.len() {
            output.push(BASE64_CHARS[(((b2 & 0b0000_1111) << 2) | (b3 >> 6)) as usize] as char);
        }
        if i + 2 < input.len() {
            output.push(BASE64_CHARS[(b3 & 0b0011_1111) as usize] as char);
        }

        i += 3;
    }

    output
}

fn base64_encode_using_library<T: AsRef<[u8]>>(input: T) -> String {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;

    URL_SAFE_NO_PAD.encode(input.as_ref())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(&[0b1010_1010, 0b1111_0000, 0b0000_1111]), "qvAP");
        assert_eq!(base64_encode(b"Hello, World!"), "SGVsbG8sIFdvcmxkIQ");
    }

    #[test]
    fn test_base64_encode_using_library() {
        assert_eq!(base64_encode_using_library(&[0b1010_1010, 0b1111_0000, 0b0000_1111]), "qvAP");
        assert_eq!(base64_encode_using_library(b"Hello, World!"), "SGVsbG8sIFdvcmxkIQ");
    }

    #[test]
    fn test_base64_encode_empty_input() {
        assert_eq!(base64_encode(&[]), "");
        assert_eq!(base64_encode_using_library(&[]), "");
    }

    #[test]
    fn test_base64_encode_single_byte() {
        assert_eq!(base64_encode(&[0b0100_0001]), "QQ");
        assert_eq!(base64_encode_using_library(&[0b0100_0001]), "QQ");
    }

    #[test]
    fn test_base64_encode_two_bytes() {
        assert_eq!(base64_encode(&[0b0100_0001, 0b0100_0010]), "QUI");
        assert_eq!(base64_encode_using_library(&[0b0100_0001, 0b0100_0010]), "QUI");
    }

    #[test]
    fn test_base64_encode_three_bytes() {
        assert_eq!(base64_encode(&[0b0100_0001, 0b0100_0010, 0b0100_0011]), "QUJD");
        assert_eq!(base64_encode_using_library(&[0b0100_0001, 0b0100_0010, 0b0100_0011]), "QUJD");
    }

    #[test]
    fn test_base64_encode_non_ascii_characters() {
        assert_eq!(base64_encode("你好".as_bytes()), "5L2g5aW9");
        assert_eq!(base64_encode_using_library("你好".as_bytes()), "5L2g5aW9");
    }

    #[test]
    fn test_base64_encode_all_zero_bytes() {
        let input = vec![0x00, 0x00, 0x00];
        assert_eq!(base64_encode(&input), "AAAA");
        assert_eq!(base64_encode_using_library(&input), "AAAA");
    }

    #[test]
    fn test_base64_encode_all_one_bytes() {
        let input = vec![0xFF, 0xFF, 0xFF];
        assert_eq!(base64_encode(&input), "____");
        assert_eq!(base64_encode_using_library(&input), "____");
    }

    #[test]
    fn test_base64_encode_mixed_bytes() {
        let input = vec![0xAB, 0xCD, 0xEF];
        assert_eq!(base64_encode(&input), "q83v");
        assert_eq!(base64_encode_using_library(&input), "q83v");
    }

    #[test]
    fn test_base64_encode_partial_padding_one_byte() {
        let input = vec![0b0100_0001];
        assert_eq!(base64_encode(&input), "QQ");
        assert_eq!(base64_encode_using_library(&input), "QQ");
    }

    #[test]
    fn test_base64_encode_partial_padding_two_bytes() {
        let input = vec![0b0100_0001, 0b0100_0010];
        assert_eq!(base64_encode(&input), "QUI");
        assert_eq!(base64_encode_using_library(&input), "QUI");
    }

    #[test]
    fn test_base64_encode_special_characters() {
        assert_eq!(base64_encode(b"!@#$%^&*()"), "IUAjJCVeJiooKQ");
        assert_eq!(base64_encode_using_library(b"!@#$%^&*()"), "IUAjJCVeJiooKQ");
    }

    #[test]
    fn test_base64_encode_numeric_input() {
        assert_eq!(base64_encode(b"1234567890"), "MTIzNDU2Nzg5MA");
        assert_eq!(base64_encode_using_library(b"1234567890"), "MTIzNDU2Nzg5MA");
    }

    #[test]
    fn test_base64_encode_alphanumeric_input() {
        assert_eq!(base64_encode(b"abc123XYZ"), "YWJjMTIzWFla");
        assert_eq!(base64_encode_using_library(b"abc123XYZ"), "YWJjMTIzWFla");
    }

    #[test]
    fn test_base64_encode_long_string() {
        let input = b"The quick brown fox jumps over the lazy dog.";
        assert_eq!(base64_encode(input), "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4");
        assert_eq!(base64_encode_using_library(input), "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4");
    }

    #[test]
    fn test_encode_bytes() {
        let mut bytes = vec![0b1111_0000, 0b0000_1111, 0xFF, 0xAB];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b0000_1111, 0b1111_0000, 0xFF, 0xBA]);
    }

    #[test]
    fn test_encode_bytes_basic() {
        let mut bytes = vec![0b1111_0000, 0b0000_1111];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b0000_1111, 0b1111_0000]);
    }

    #[test]
    fn test_encode_bytes_all_zeros() {
        let mut bytes = vec![0b0000_0000];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b0000_0000]);
    }

    #[test]
    fn test_encode_bytes_all_ones() {
        let mut bytes = vec![0b1111_1111];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b1111_1111]);
    }

    #[test]
    fn test_encode_bytes_mixed_values() {
        let mut bytes = vec![0b1010_0101, 0b0101_1010];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b0101_1010, 0b1010_0101]);
    }

    #[test]
    fn test_encode_bytes_single_byte() {
        let mut bytes = vec![0b1001_0110];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0b0110_1001]);
    }

    #[test]
    fn test_encode_bytes_empty_vector() {
        let mut bytes: Vec<u8> = vec![];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![]);
    }

    #[test]
    fn test_encode_bytes_large_vector() {
        let mut bytes = vec![0b1111_0000; 1000];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![0b0000_1111; 1000]);
    }

    #[test]
    fn test_encode_bytes_edge_values() {
        let mut bytes = vec![0x00, 0xFF, 0x80, 0x7F];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0x00, 0xFF, 0x08, 0xF7]);
    }

    #[test]
    fn test_encode_bytes_random_values() {
        let mut bytes = vec![0xAB, 0xCD, 0xEF];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, &[0xBA, 0xDC, 0xFE]);
    }
}
