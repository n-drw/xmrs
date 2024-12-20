use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
#[repr(C)]
pub struct ItMidiMacros {
    global: [[u8; 32]; 9],
    parametric: [[u8; 32]; 16],
    #[serde(with = "BigArray")]
    fixed: [[u8; 32]; 128],
}
