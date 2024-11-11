use serde::Deserialize;
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
#[repr(C)]
pub struct ItMidiMacros {
    global_macros: [[u8; 32]; 9],
    parametric_macros: [[u8; 32]; 16],
    #[serde(with = "BigArray")]
    fixed_macros: [[u8; 32]; 128],
}
