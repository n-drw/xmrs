use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

/// Structure representing a pattern in a musical tracker format.
/// Note: The entire `Pattern` struct is limited to a maximum size of 0xFFFF (64 kilobytes).
#[derive(Deserialize, Debug, Default)]
#[repr(C)]
pub struct ItPattern {
    /// Length of the packed data in bytes.
    /// Length: 2 bytes
    pattern_length: u16,

    /// Number of rows in the pattern.
    /// Length: 2 bytes (signed)
    /// - IT format allows between 32 and 200 rows.
    /// - OpenMPT may support a larger row count.
    row_count: i16,

    /// Reserved bytes for future use.
    /// Length: 4 bytes
    reserved: u32,

    /// Packed data of the pattern.
    /// Length: Defined by `pattern_length`
    packed_data: Vec<u8>,
}

impl ItPattern {
    pub fn load(source: &[u8]) -> Self {
        let data = source;

        let pattern_length: u16 = u16::from_le_bytes(data[0..2].try_into().unwrap());
        let row_count: i16 = i16::from_le_bytes(data[2..4].try_into().unwrap());
        let reserved: u32 = u32::from_le_bytes(data[4..8].try_into().unwrap());

        if pattern_length == 0 {
            return Self {
                pattern_length,
                row_count,
                reserved,
                packed_data: vec![],
            };
        }

        let start = 8;
        let end = 8 + pattern_length as usize;
        return Self {
            pattern_length,
            row_count,
            reserved,
            packed_data: data[start..end].to_vec(),
        };
    }
}
