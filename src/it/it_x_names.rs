use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[repr(C)]
pub struct ItXNames;

impl ItXNames {
    pub fn load(source: &[u8], chunk_size: usize) -> (Vec<String>, usize) {
        let data = source;

        let length = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if length == 0 {
            return (vec![], 4);
        }
        let data = &data[4..4 + length as usize];
        // let dest: Vec<String> = data.chunks(chunk_size).map(|chunk| String::from_utf8_lossy(chunk).trim().to_string()).collect();

        let dest: Vec<String> = data
            .chunks(chunk_size)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&byte| if byte == 0x00 { 0x20 } else { byte }) // Remplace 0x00 par 0x20
                    .collect::<Vec<u8>>() // Collecte dans un Vec<u8>
            })
            .map(|chunk| String::from_utf8_lossy(&chunk).trim().to_string()) // Convertit chaque chunk en String
            .collect();

        (dest, 4 + length as usize)
    }

    pub fn is_pnam(data: &[u8]) -> bool {
        data[0] == b'P' && data[1] == b'N' && data[2] == b'A' && data[3] == b'M'
    }

    pub fn is_cnam(data: &[u8]) -> bool {
        data[0] == b'C' && data[1] == b'N' && data[2] == b'A' && data[3] == b'M'
    }
}
