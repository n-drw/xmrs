use alloc::vec;
use alloc::vec::Vec;
use bincode::error::DecodeError;
use bincode::Decode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Decode)]
#[repr(C)]
pub struct ItEditHistoryEntry {
    // 2 bytes (Microsoft FAT date format)
    fat_date: u16,

    // 2 bytes (Microsoft FAT time format)
    fat_time: u16,

    // 4 bytes (Runtime in MS-DOS ticks, 1/18.2 second)
    run_time: u32,
}

#[derive(Deserialize, Debug, Default)]
#[repr(C)]
pub struct ItEditHistory;

impl ItEditHistory {
    pub fn load(source: &[u8]) -> Result<(Option<Vec<ItEditHistoryEntry>>, usize), DecodeError> {
        let data = source;

        if data.len() < 2 {
            return Err(DecodeError::LimitExceeded);
        }

        let edit_history_number: u16 = u16::from_le_bytes(data[0..2].try_into().unwrap());
        if edit_history_number == 0 {
            return Ok((None, 2));
        }

        let struct_size = core::mem::size_of::<ItEditHistoryEntry>();
        let total_size = struct_size * edit_history_number as usize;
        let data = &data[2..2 + total_size];

        if data.len() < total_size {
            return Err(DecodeError::LimitExceeded);
        }

        let mut edit_histories: Vec<ItEditHistoryEntry> = vec![];
        for i in 0..edit_history_number {
            let start = struct_size * i as usize;
            let end = start + struct_size;
            let src = &data[start..end];
            let edit_history_entry: (ItEditHistoryEntry, usize) =
                bincode::decode_from_slice::<ItEditHistoryEntry, _>(src, bincode::config::legacy())
                    .unwrap();
            edit_histories.push(edit_history_entry.0);
        }

        Ok((Some(edit_histories), 2 + total_size))
    }
}
