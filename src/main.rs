fn main() {
    println!("Hello, world!");
}
fn encode_bytes(bytes: &mut Vec<u8>) {
    for byte in bytes.iter_mut() {
        *byte = (*byte >> 4) | (*byte << 4);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
