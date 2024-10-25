use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::envelope::Envelope;
use crate::instr_midi::InstrMidi;
use crate::instr_vibrato::InstrVibrato;
use crate::sample::Sample;

use alloc::{vec, vec::Vec};

/// Historical XM Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrDefault {
    #[serde(with = "BigArray")]
    pub sample_for_pitch: [u8; 120],
    pub volume_envelope: Envelope,  // Envelope.points[].value: 0.0..1.0
    pub panning_envelope: Envelope, // Envelope.points[].value: 0.0..1.0
    pub vibrato: InstrVibrato,
    pub volume_fadeout: f32, // 0.0..1.0
    pub sample: Vec<Sample>,
    pub midi: InstrMidi,
    pub midi_mute_computer: bool,
}

impl Default for InstrDefault {
    fn default() -> Self {
        Self {
            sample_for_pitch: [0; 120],
            volume_envelope: Envelope::default(),
            panning_envelope: Envelope::default(),
            vibrato: InstrVibrato::default(),
            volume_fadeout: 0.0,
            sample: vec![],
            midi: InstrMidi::default(),
            midi_mute_computer: false,
        }
    }
}
