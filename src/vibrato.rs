use serde::{Deserialize, Serialize};

use crate::waveform::Waveform;

#[cfg(feature = "micromath")]
#[allow(unused_imports)]
use micromath::F32Ext;
#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::float::Float;

/// Instrument Vibrato
#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Vibrato {
    pub waveform: Waveform,
    pub speed: f32,
    pub depth: f32,
    pub sweep: f32,
}
