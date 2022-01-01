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

#[cfg(test)]
mod tests {
    use super::{to_f32, to_f64, to_i32, to_i64, to_u16, to_u32};

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
}
