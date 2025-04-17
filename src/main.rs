fn main() {
    println!("Hello, world!");
}
fn encode_bytes(bytes: &mut Vec<u8>) -> () {
    for byte in bytes.iter_mut() {
        *byte = byte.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bytes() {
        let mut bytes = b"Hello, World!".to_vec();
        encode_bytes(&mut bytes);
        assert_eq!(bytes, b"Ifmmp-!Xpsme\"");
    }

    #[test]
    fn test_empty_vector() {
        let mut bytes: Vec<u8> = vec![];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![]);
    }

    #[test]
    fn test_all_zeros() {
        let mut bytes = vec![0, 0, 0];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![1, 1, 1]);
    }

    #[test]
    fn test_all_max_values() {
        let mut bytes = vec![255, 255, 255];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![0, 0, 0]);
    }

    #[test]
    fn test_mixed_values() {
        let mut bytes = vec![0, 127, 255];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![1, 128, 0]);
    }

    #[test]
    fn test_single_byte() {
        let mut bytes = vec![42];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![43]);
    }

    #[test]
    fn test_encode_bytes_extra() {
        // Non-ASCII characters
        let mut bytes = vec![200, 201, 202];
        encode_bytes(&mut bytes);
        assert_eq!(bytes, vec![201, 202, 203]);

        // Large input
        let mut large_bytes = vec![1; 10_000];
        encode_bytes(&mut large_bytes);
        assert_eq!(large_bytes, vec![2; 10_000]);

        // Repeated patterns
        let mut pattern_bytes = vec![1, 2, 3, 1, 2, 3];
        encode_bytes(&mut pattern_bytes);
        assert_eq!(pattern_bytes, vec![2, 3, 4, 2, 3, 4]);
    }
}
