//! Contains useful common functionality and utilities

/// Create a new u16 from two bytes
/// Byte order is assumed to be big-endian
pub fn to_u16(bytes: &[u8]) -> u16 {
    assert!(
        bytes.len() == 2,
        "Expected 2 bytes, got {} bytes",
        bytes.len()
    );

    u16::from_be_bytes([bytes[0], bytes[1]])
}

/// Create a new u32 from four bytes
/// Byte order is assumed to be big-endian
pub fn to_u32(bytes: &[u8]) -> u32 {
    assert!(
        bytes.len() == 4,
        "Expected 4 bytes, got {} bytes",
        bytes.len()
    );

    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Create a new i32 from four bytes
/// Byte order is assumed to be big-endian
pub fn to_i32(bytes: &[u8]) -> i32 {
    assert!(
        bytes.len() == 4,
        "Expected 4 bytes, got {} bytes",
        bytes.len()
    );

    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Create a new i64 from four bytes
/// Byte order is assumed to be big-endian
pub fn to_i64(bytes: &[u8]) -> i64 {
    assert!(
        bytes.len() == 8,
        "Expected 8 bytes, got {} bytes",
        bytes.len()
    );

    i64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

/// Create a new f32 from four bytes
/// Byte order is assumed to be big-endian
pub fn to_f32(bytes: &[u8]) -> f32 {
    assert!(
        bytes.len() == 4,
        "Expected 4 bytes, got {} bytes",
        bytes.len()
    );

    f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Create a new i64 from four bytes
/// Byte order is assumed to be big-endian
pub fn to_f64(bytes: &[u8]) -> f64 {
    assert!(
        bytes.len() == 8,
        "Expected 8 bytes, got {} bytes",
        bytes.len()
    );

    f64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

/// Checks if the specified bitmask is set
pub fn bitmask_matches(value: u16, bitmask: u16) -> bool {
    value & bitmask == bitmask
}

#[cfg(test)]
mod tests {
    use super::{bitmask_matches, to_f32, to_f64, to_i32, to_i64, to_u16, to_u32};

    #[test]
    fn test_to_u16_valid_args() {
        to_u16(&[1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_u16_invalid_args() {
        to_u16(&[1]);
        to_u16(&[1, 1, 1]);
    }

    #[test]
    fn test_to_u32_valid_args() {
        to_u32(&[1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_u32_invalid_args() {
        to_u32(&[1]);
        to_u32(&[1, 1]);
        to_u32(&[1, 1, 1]);
        to_u32(&[1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_to_i32_valid_args() {
        to_i32(&[1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_i32_invalid_args() {
        to_i32(&[1]);
        to_i32(&[1, 1]);
        to_i32(&[1, 1, 1]);
        to_i32(&[1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_to_i64_valid_args() {
        to_i64(&[1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_i64_invalid_args() {
        to_i64(&[1]);
        to_i64(&[1, 1]);
        to_i64(&[1, 1, 1]);
        to_i64(&[1, 1, 1, 1]);
        to_i64(&[1, 1, 1, 1, 1]);
        to_i64(&[1, 1, 1, 1, 1, 1]);
        to_i64(&[1, 1, 1, 1, 1, 1, 1]);
        to_i64(&[1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_to_f32_valid_args() {
        to_f32(&[1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_f32_invalid_args() {
        to_f32(&[1]);
        to_f32(&[1, 1]);
        to_f32(&[1, 1, 1]);
        to_f32(&[1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_to_f64_valid_args() {
        to_f64(&[1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    #[should_panic]
    fn test_to_f64_invalid_args() {
        to_f64(&[1]);
        to_f64(&[1, 1]);
        to_f64(&[1, 1, 1]);
        to_f64(&[1, 1, 1, 1]);
        to_f64(&[1, 1, 1, 1, 1]);
        to_f64(&[1, 1, 1, 1, 1, 1]);
        to_f64(&[1, 1, 1, 1, 1, 1, 1]);
        to_f64(&[1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_bitmask_check() {
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0000_0001),
            true,
            "Bit 0 should be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0000_0010),
            true,
            "Bit 1 should be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0000_0100),
            false,
            "Bit 2 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0000_1000),
            false,
            "Bit 3 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0001_0000),
            false,
            "Bit 4 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0010_0000),
            true,
            "Bit 5 should be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_0100_0000),
            false,
            "Bit 6 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0000_1000_0000),
            false,
            "Bit 7 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0001_0000_0000),
            false,
            "Bit 8 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0010_0000_0000),
            true,
            "Bit 9 should be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_0100_0000_0000),
            false,
            "Bit 10 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0000_1000_0000_0000),
            false,
            "Bit 11 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0001_0000_0000_0000),
            false,
            "Bit 12 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0010_0000_0000_0000),
            false,
            "Bit 13 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_0100_0000_0000_0000),
            false,
            "Bit 14 should not be set"
        );
        assert_eq!(
            bitmask_matches(33315, 0b_1000_0000_0000_0000),
            true,
            "Bit 15 should be set"
        );

        assert_eq!(
            bitmask_matches(33315, 0b_1000_0010_0010_0011),
            true,
            "Bits 0, 1, 5, 9, and 15 should be set"
        );
    }
}
