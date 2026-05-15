#![forbid(unsafe_code)]

//! Reusable computing constants expressed as plain integer values.

/// Exact computing constant for the number of bits in one byte.
pub const BITS_PER_BYTE: usize = 8;

/// Exact computing constant for the number of bits in one nibble.
pub const NIBBLE_BITS: usize = 4;

/// Exact computing constant for the number of distinct values in one byte.
pub const BYTE_VALUES: usize = 256;

/// Binary storage constant for one kibibyte, in bytes.
pub const KIBIBYTE: usize = 1024;

/// Binary storage constant for one mebibyte, in bytes.
pub const MEBIBYTE: usize = 1024 * KIBIBYTE;

/// Binary storage constant for one gibibyte, in bytes.
pub const GIBIBYTE: usize = 1024 * MEBIBYTE;

/// Binary storage constant for one tebibyte, in bytes.
pub const TEBIBYTE: usize = 1024 * GIBIBYTE;

/// Decimal storage constant for one kilobyte, in bytes.
pub const KILOBYTE: usize = 1000;

/// Decimal storage constant for one megabyte, in bytes.
pub const MEGABYTE: usize = 1000 * KILOBYTE;

/// Decimal storage constant for one gigabyte, in bytes.
pub const GIGABYTE: usize = 1000 * MEGABYTE;

/// Decimal storage constant for one terabyte, in bytes.
pub const TERABYTE: usize = 1000 * GIGABYTE;

#[cfg(test)]
mod tests {
    use super::{BITS_PER_BYTE, BYTE_VALUES, KIBIBYTE, KILOBYTE, MEBIBYTE, MEGABYTE, NIBBLE_BITS};

    #[test]
    fn byte_size_relationships_hold() {
        assert_eq!(BYTE_VALUES, 1usize << BITS_PER_BYTE);
        assert_eq!(NIBBLE_BITS * 2, BITS_PER_BYTE);
    }

    #[test]
    fn binary_storage_relationships_hold() {
        assert_eq!(MEBIBYTE, 1024 * KIBIBYTE);
    }

    #[test]
    fn decimal_storage_relationships_hold() {
        assert_eq!(MEGABYTE, 1000 * KILOBYTE);
    }
}
