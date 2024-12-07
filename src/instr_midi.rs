use serde::{Deserialize, Serialize};

/// Midi Instrument
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct InstrMidi {
    pub on: bool,
    /// MIDI channel (0-16)
    pub channel: u8,
    /// MIDI program (1-128)
    pub program: u16,
    /// MIDI bank (0-16384)
    pub bank: u16,
    pub bend: u16,
}
