use super::xorshift::XorShift32;
use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
#[allow(unused_imports)]
use micromath::F32Ext;
#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::float::Float;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum Waveform {
    #[default]
    TranslatedSine,
    TranslatedSquare,
    TranslatedRampUp,
    TranslatedRampDown,
    Sine,
    RampDown,
    Square,
    Random {
        rng: XorShift32,
    },
}

impl Waveform {
    pub fn new_random(seed: Option<u32>) -> Self {
        let seed = if let Some(v) = seed { v } else { 4294967291 };
        Waveform::Random {
            rng: XorShift32::new(seed),
        }
    }

    pub fn value(&mut self, step: f32) -> f32 {
        let step = step % 1.0;
        return match self {
            Waveform::TranslatedSine => 0.5 + 0.5 * (core::f32::consts::TAU * (step + 0.25)).sin(),
            Waveform::TranslatedSquare => {
                if step < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            Waveform::TranslatedRampUp => {
                if step < 0.5 {
                    0.5 * step
                } else {
                    0.5 * step + 0.5
                }
            }
            Waveform::TranslatedRampDown => {
                if step < 0.5 {
                    1.0 - 0.5 * step
                } else {
                    -0.5 * step + 0.5
                }
            }
            Waveform::Sine => -(core::f32::consts::TAU * step).sin(),
            Waveform::RampDown => -2.0 * step + if step < 0.5 { 0.0 } else { 2.0 },
            Waveform::Square => {
                if step < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Random { rng } => rng.next_f32(),
        };
    }
}
