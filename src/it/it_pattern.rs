use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

use crate::prelude::PatternSlot;

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

    pub fn unpack(&self) -> Vec<Vec<PatternSlot>> {
        let mut result = vec![vec![PatternSlot::default(); 64]; self.row_count as usize];
        let mut last_mask_vars = vec![0u8; 64];
        let mut data_iter = self.packed_data.iter();

        for row in 0..self.row_count as usize {
            let mut channel_mask = match data_iter.next() {
                Some(&mask) => mask,
                None => break,
            };

            while channel_mask > 0 {
                let channel = (channel_mask - 1) & 63;

                let mask_variable = if channel_mask & 0x80 != 0 {
                    let var = *data_iter.next().unwrap();
                    last_mask_vars[channel as usize] = var;
                    var
                } else {
                    last_mask_vars[channel as usize]
                };

                let mut slot = PatternSlot::default();

                if mask_variable & 0x01 != 0 {
                    let n = *data_iter.next().unwrap();
                    slot.note = n.try_into().unwrap();
                } else if mask_variable & 0x10 != 0 && row > 0 {
                    slot.note = result[row - 1][channel as usize].note;
                }

                if mask_variable & 0x02 != 0 {
                    slot.instrument = *data_iter.next().unwrap();
                } else if mask_variable & 0x20 != 0 && row > 0 {
                    slot.instrument = result[row - 1][channel as usize].instrument;
                }

                if mask_variable & 0x04 != 0 {
                    slot.volume = *data_iter.next().unwrap();
                } else if mask_variable & 0x40 != 0 && row > 0 {
                    slot.volume = result[row - 1][channel as usize].volume;
                }

                if mask_variable & 0x08 != 0 {
                    slot.effect_type = *data_iter.next().unwrap();
                    slot.effect_parameter = *data_iter.next().unwrap();
                } else if mask_variable & 0x80 != 0 && row > 0 {
                    slot.effect_type = result[row - 1][channel as usize].effect_type;
                    slot.effect_parameter = result[row - 1][channel as usize].effect_parameter;
                }

                result[row][channel as usize] = slot;

                channel_mask = match data_iter.next() {
                    Some(&mask) => mask,
                    None => break,
                };
            }
        }
        result
    }
}
