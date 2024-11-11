use super::patternslot::PatternSlot;
use crate::effect::TrackEffect;
use crate::prelude::*;
use crate::track_unit::TrackUnit;
use alloc::vec::Vec;

pub struct XmEffect;

impl XmEffect {
    fn walk_effect(freq_type: FrequencyType, current: &PatternSlot) -> Option<TrackEffect> {
        match current.effect_type {
            0x00 => {
                let param = current.effect_parameter;
                if param > 0 {
                    let v1 = (param >> 4) as f32;
                    let v2 = (param & 0x0F) as f32;
                    return Some(TrackEffect::Arpeggio((v1, v2)));
                } else {
                    return None;
                }
            }
            0x01 => {
                return Some(TrackEffect::Portamento(
                    -4.0 * current.effect_parameter as f32,
                ))
            }
            0x02 => {
                return Some(TrackEffect::Portamento(
                    4.0 * current.effect_parameter as f32,
                ))
            }
            0x03 => {
                let param = current.effect_parameter as f32;
                let speed = match freq_type {
                    FrequencyType::LinearFrequencies => 4.0 * param,
                    FrequencyType::AmigaFrequencies => param,
                };
                return Some(TrackEffect::TonePortamento(speed));
            }
            0x04 => {
                let param = current.effect_parameter;
                let speed = ((param & 0xF0) >> 4) as f32 / 64.0;
                let depth = (param & 0x0F) as f32 / 16.0;
                return Some(TrackEffect::Vibrato((speed, depth)));
            }
            0x05 | 0x06 | 0x0A => {
                let param = current.effect_parameter;
                let upper_nibble = param & 0xF0;
                let lower_nibble = param & 0x0F;

                return match (upper_nibble, lower_nibble) {
                    (f, 0) => Some(TrackEffect::VolumeSlide(((f >> 4) as f32 / 64.0, false))), // Slide up
                    (0, f) => Some(TrackEffect::VolumeSlide((-(f as f32) / 64.0, false))), // Slide down
                    _ => None,
                };
            }
            0x07 => {
                let param = current.effect_parameter;
                let speed = ((param & 0xF0) >> 4) as f32 / 64.0;
                let depth = (param & 0x0F) as f32 / 16.0;
                return Some(TrackEffect::Tremolo((speed, depth)));
            }
            0x08 => {
                return Some(TrackEffect::Panning(
                    (current.effect_parameter as f32) / 255.0,
                ))
            }
            0x09 => {
                return Some(TrackEffect::InstrumentSampleOffset(
                    current.effect_parameter as usize * 256,
                ))
            }
            0x0C => {
                return Some(TrackEffect::Volume((
                    (current.effect_parameter.max(64) as f32) / 64.0,
                    0,
                )))
            }
            0x0E => {
                let param = current.effect_parameter & 0x0F;
                match current.effect_parameter >> 4 {
                    0x1 => {
                        return (param != 0).then(|| TrackEffect::Portamento(-4.0 * param as f32))
                    }
                    0x2 => {
                        return (param != 0).then(|| TrackEffect::Portamento(4.0 * param as f32))
                    }
                    0x3 => return Some(TrackEffect::Glissando(current.effect_parameter != 0)),
                    0x4 => {
                        let waveform = match param & 0b0000_0011 {
                            1 => Waveform::RampDown,
                            2 => Waveform::Square,
                            _ => Waveform::Sine,
                        };
                        let retrig = (param & 0b0000_0100) == 0;
                        return Some(TrackEffect::VibratoWaveform((waveform, retrig)));
                    }
                    0x5 => return Some(TrackEffect::InstrumentFineTune(param as f32 / 8.0 - 1.0)),
                    0x7 => {
                        let waveform = match param & 0b0000_0011 {
                            1 => Waveform::RampDown,
                            2 => Waveform::Square,
                            _ => Waveform::Sine,
                        };
                        let retrig = (param & 0b0000_0100) == 0;
                        return Some(TrackEffect::TremoloWaveform((waveform, retrig)));
                    }
                    0x9 => return Some(TrackEffect::NoteRetrig(param as usize)),
                    0xA => return Some(TrackEffect::VolumeSlide(((param as f32) / 64.0, true))),
                    0xB => return Some(TrackEffect::VolumeSlide((-(param as f32) / 64.0, true))),
                    0xC => return Some(TrackEffect::NoteCut(param as usize)),
                    0xD => return Some(TrackEffect::NoteDelay(param as usize)),
                    _ => return None,
                }
            }
            // Kxx: Key off
            0x14 => return Some(TrackEffect::NoteOff(current.effect_parameter as usize)),
            // Lxx: Set envelope position
            0x15 => {
                return Some(TrackEffect::InstrumentVolumeEnvelopePosition(
                    current.effect_parameter as usize,
                ))
            }
            // Pxy: Panning slide
            0x19 => {
                let param = current.effect_parameter;
                let upper_nibble = param & 0xF0;
                let lower_nibble = param & 0x0F;

                return match (upper_nibble, lower_nibble) {
                    (f, 0) => Some(TrackEffect::PanningSlide((f >> 4) as f32 / 16.0)), // Slide up
                    (0, f) => Some(TrackEffect::PanningSlide(-(f as f32) / 16.0)),     // Slide down
                    _ => None,
                };
            }
            // Rxy: Multi retrig note
            0x1B => {
                let param = current.effect_parameter;
                let vol = (param >> 4) as f32 / 16.0;
                let speed = param & 0x0F;
                return Some(TrackEffect::MultiRetrigNote((speed as usize, vol)));
            }
            // Txy: Tremor
            // Rapidly switches the sample volume on and off on every tick of the row except the first.
            // Volume is on for x + 1 ticks and off for y + 1 ticks.
            // tremor_on: bool = [(T-1) % (X+1+Y+1) ] > X
            0x1D => {
                let param = current.effect_parameter;
                let on_time = param >> 4;
                let off_time = param & 0x0F;
                return Some(TrackEffect::Tremor((on_time as usize, off_time as usize)));
            }
            // X1 & X2
            0x21 => {
                let effect_case = current.effect_parameter >> 4;
                let param = current.effect_parameter & 0x0F;

                return match effect_case {
                    1 => {
                        // X1y: Extra fine portamento up
                        Some(TrackEffect::Portamento(-(param as f32)))
                    }
                    2 => {
                        // X2y: Extra fine portamento down
                        Some(TrackEffect::Portamento(param as f32))
                    }
                    _ => None,
                };
            }
            _ => return None,
        }
    }

    fn walk_volume(freq_type: FrequencyType, current: &PatternSlot) -> Option<TrackEffect> {
        match current.volume {
            0x0 => return None,
            // V - Set volume (0..63)
            0x1..=0x4 => {
                return Some(TrackEffect::Volume((
                    (current.volume - 0x10) as f32 / 64.0,
                    0,
                )))
            }
            // V - 0x51..0x5F undefined...
            0x5 => {
                return Some(TrackEffect::Volume((
                    (current.volume - 0x20) as f32 / 64.0,
                    0,
                )))
            }
            // - - Volume slide down (0..15)
            0x6 => {
                return Some(TrackEffect::VolumeSlide((
                    -((current.volume & 0x0F) as f32) / 64.0,
                    false,
                )))
            }
            // + - Volume slide up (0..15)
            0x7 => {
                return Some(TrackEffect::VolumeSlide((
                    ((current.volume & 0x0F) as f32) / 64.0,
                    false,
                )))
            }
            // D - Fine volume slide down (0..15)
            0x8 => {
                return Some(TrackEffect::VolumeSlide((
                    -((current.volume & 0x0F) as f32) / 64.0,
                    true,
                )))
            }
            // U - Fine volume slide up (0..15)
            0x9 => {
                return Some(TrackEffect::VolumeSlide((
                    ((current.volume & 0x0F) as f32) / 64.0,
                    true,
                )))
            }
            // S - Vibrato speed (0..15)
            0xA => {
                let s = current.volume & 0x0F;
                if s != 0 {
                    return Some(TrackEffect::VibratoSpeed((s as f32) / 64.0));
                } else {
                    return None;
                }
            }
            // V - Vibrato depth (0..15)
            0xB => {
                let d = current.volume & 0x0F;
                if d != 0 {
                    return Some(TrackEffect::VibratoDepth((d as f32) / 16.0));
                } else {
                    return None;
                }
            }
            // P - Set panning
            0xC => return Some(TrackEffect::Panning((current.volume as f32) / 16.0)),
            // L - Panning slide left
            0xD => return Some(TrackEffect::PanningSlide(-(current.volume as f32) / 16.0)),
            // R - Panning slide right
            0xE => return Some(TrackEffect::PanningSlide((current.volume as f32) / 16.0)),
            // M - Tone portamento (0..15)
            0xF => {
                let speed = (current.volume & 0x0F) as f32 * 16.0;
                let speed = match freq_type {
                    FrequencyType::LinearFrequencies => 4.0 * speed,
                    FrequencyType::AmigaFrequencies => speed,
                };
                return Some(TrackEffect::TonePortamento(speed));
            }
            _ => {
                return None;
            }
        }
    }

    fn walk_control_change_effect(current: &PatternSlot) -> Option<ControlChangeEffect> {
        match current.effect_type {
            // Bxx: Position jump
            0x0B => {
                // TODO: counts from 1, so 0 and 1 jump to zero...
                return Some(ControlChangeEffect::PositionJump(
                    current.effect_parameter as usize,
                ));
            }
            // Dxx: Pattern break
            0x0D => {
                let param = current.effect_parameter;
                let ten = (param >> 4) as usize * 10;
                let unit = (param & 0x0F) as usize;
                let total = ten + unit;
                return Some(ControlChangeEffect::PatternBreak(total));
            }
            // EXy: Extended command
            0x0E => {
                let param = current.effect_parameter & 0x0F;
                match current.effect_parameter >> 4 {
                    // E6y: Pattern loop
                    0x6 => return Some(ControlChangeEffect::PatternLoop(param as usize)),
                    // EEy: Pattern delay
                    0xE => Some(ControlChangeEffect::PatternDelay(param as usize)),
                    _ => None,
                }
            }
            // Fxx: Set tempo/BPM
            0x0F => {
                let param = current.effect_parameter;
                if param < 32 {
                    return Some(ControlChangeEffect::Speed(param as usize));
                } else {
                    return Some(ControlChangeEffect::Bpm(param as usize));
                }
            }
            // Gxx: Set global volume
            0x10 => {
                return Some(ControlChangeEffect::Volume(
                    (current.effect_parameter.max(64) as f32) / 64.0,
                ))
            }
            // Hxy: Global volume slide
            0x11 => {
                let param = current.effect_parameter;
                let upper_nibble = param & 0xF0;
                let lower_nibble = param & 0x0F;

                return match (upper_nibble, lower_nibble) {
                    (f, 0) => Some(ControlChangeEffect::VolumeSlide((f >> 4) as f32 / 64.0)), // Slide up
                    (0, f) => Some(ControlChangeEffect::VolumeSlide(-(f as f32) / 64.0)), // Slide down
                    _ => None,
                };
            }
            _ => return None,
        }
    }

    pub fn unpack(freq_type: FrequencyType, current: &PatternSlot) -> TrackUnit {
        let te = Self::walk_effect(freq_type, current);
        let ve = Self::walk_volume(freq_type, current);
        let cc = Self::walk_control_change_effect(current);

        let mut tu = TrackUnit::default();
        tu.note = current.note;
        tu.instrument = current.instrument;
        if let Some(e) = te {
            tu.effects.push(e)
        }
        if let Some(e) = ve {
            tu.effects.push(e);
        }

        // TODO: one day clean up the effects to remove redundancies by grouping them together (eg. Volume...)

        if let Some(c) = cc {
            tu.cc_effects.push(c);
        }
        tu
    }

    pub fn unpack_row(freq_type: FrequencyType, row: &Vec<PatternSlot>) -> Vec<TrackUnit> {
        row.iter().map(|pattern_slot| {
            XmEffect::unpack(freq_type, &pattern_slot)
        }).collect()
    }

    pub fn unpack_pattern(freq_type: FrequencyType, pattern: &Vec<Vec<PatternSlot>>) -> Vec<Vec<TrackUnit>> {
        pattern.iter().map(|row| {
            XmEffect::unpack_row(freq_type, &row)
        }).collect()
    }

    pub fn unpack_patterns(freq_type: FrequencyType, patterns: &Vec<Vec<Vec<PatternSlot>>>) -> Vec<Vec<Vec<TrackUnit>>> {
        patterns.iter().map(|pattern| {
            XmEffect::unpack_pattern(freq_type, &pattern)
        }).collect()
    }

}
