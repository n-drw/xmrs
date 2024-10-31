use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[repr(C)]
struct ItXNames;

impl ItXNames {
    pub fn load(source: &[u8]) -> (Vec<String>, usize) {
        let data = source;

        let length: u16 = u16::from_le_bytes(data[0..2].try_into().unwrap());
        if length == 0 {
            return (vec![], 2);
        }

        let total_size = 32 * length as usize;
        let data = &data[2..total_size];

        let vec_u8: Vec<[u8; 32]> =
            bincode::decode_from_slice::<Vec<[u8; 32]>, _>(data, bincode::config::legacy())
                .unwrap()
                .0;

        let vec_string: Vec<String> = vec_u8
            .iter()
            .map(|arr| String::from_utf8_lossy(arr).to_string())
            .collect();

        (vec_string, 2 + total_size)
    }
}
