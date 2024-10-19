use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

use alloc::string::String;
use alloc::vec::Vec;

/// How to play sample
#[derive(Default, Serialize, Deserialize, Copy, Clone, IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum LoopType {
    #[default]
    No = 0,
    Forward = 1,
    PingPong = 2,
}

/// is sample recorded with 8 or 16 bits depth
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SampleDataType {
    Mono8(Vec<i8>),
    Mono16(Vec<i16>),
    Stereo8(Vec<i8>),
    Stereo16(Vec<i16>),
}

/// A Real Data sample
#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    /// Name
    pub name: String,
    /// 0 <= loop_start < len()
    pub loop_start: u32,
    /// 1 <= loop_length <= len() - loop_start
    pub loop_length: u32,
    /// [0..1] linear value
    pub volume: f32,
    /// [-1..1]
    pub finetune: f32,
    /// loop type
    pub flags: LoopType,
    /// [0..1] <=> [left..right]
    pub panning: f32,
    /// [-96..95] with 0 <=> C-4
    pub relative_note: i8,
    /// wave data
    pub data: SampleDataType,
}

impl Sample {
    /// return sample length
    pub fn len(&self) -> usize {
        match &self.data {
            SampleDataType::Mono8(v) => v.len(),
            SampleDataType::Mono16(v) => v.len(),
            SampleDataType::Stereo8(v) => v.len() / 2,
            SampleDataType::Stereo16(v) => v.len() / 2,
        }
    }

    /// return sample at seek
    pub fn at(&self, seek: usize) -> (f32, f32) {
        match &self.data {
            SampleDataType::Mono8(v) => (v[seek] as f32 / 128.0, v[seek] as f32 / 128.0),
            SampleDataType::Mono16(v) => (v[seek] as f32 / 32768.0, v[seek] as f32 / 32768.0),
            SampleDataType::Stereo8(v) => {
                (v[seek * 2] as f32 / 128.0, v[seek * 2 + 1] as f32 / 128.0)
            }
            SampleDataType::Stereo16(v) => (
                v[seek * 2] as f32 / 32768.0,
                v[seek * 2 + 1] as f32 / 32768.0,
            ),
        }
    }

    pub fn clamp(&mut self) {
        self.volume = self.volume.clamp(0.0, 1.0);
        self.panning = self.panning.clamp(0.0, 1.0);
        self.finetune = self.finetune.clamp(-1.0, 1.0);
        self.relative_note = self.relative_note.clamp(-95, 96);
        if self.loop_start as usize > self.len() {
            self.loop_start = 0;
        }
        if self.loop_start as usize + self.loop_length as usize > self.len() {
            self.loop_length = self.len() as u32 - self.loop_start;
        }
    }

    // return corrected position and sample.
    pub fn meta_at(&self, pos: usize) -> Option<(usize, (f32, f32))> {
        let len = self.len();
        let loop_start = self.loop_start as usize;
        let loop_length = self.loop_length as usize;
        let loop_end = loop_start + loop_length;

        match self.flags {
            LoopType::No => {
                if pos < len {
                    return Some((pos, self.at(pos)));
                } else {
                    return None;
                }
            }
            LoopType::Forward => {
                let pos = if pos < loop_end {
                    pos
                } else {
                    loop_start + (pos - loop_start) % loop_length
                };
                return Some((pos, self.at(pos)));
            }
            LoopType::PingPong => {
                let pos = if pos < loop_end {
                    pos
                } else {
                    let total_length = 2 * loop_length;
                    let mod_pos = (pos - loop_start) % total_length;
                    if mod_pos < loop_length {
                        loop_start + mod_pos
                    } else {
                        loop_end - (mod_pos - loop_length) - 1
                    }
                };
                return Some((pos, self.at(pos)));
            }
        }
    }

    /// return sample size (8 or 16 bits)
    pub fn bits(&self) -> u8 {
        match &self.data {
            SampleDataType::Mono8(_) => 8,
            SampleDataType::Mono16(_) => 16,
            SampleDataType::Stereo8(_) => 8,
            SampleDataType::Stereo16(_) => 16,
        }
    }
}
