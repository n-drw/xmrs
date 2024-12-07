use crate::import::patternslot::PatternSlot;
use crate::import::track_import_effect::TrackImportEffect;
use crate::import::track_import_unit::TrackImportUnit;
use crate::prelude::*;
use alloc::vec;
use alloc::vec::Vec;

pub struct ModXmEffect;

impl ModXmEffect {
    fn mod_xm_volume_slide(param: u8) -> Option<f32> {
        let upper_nibble = param & 0xF0;
        let lower_nibble = param & 0x0F;

        return match (upper_nibble, lower_nibble) {
            (f, 0) => Some((f >> 4) as f32 / 64.0), // Slide up
            (0, f) => Some(-(f as f32) / 64.0),     // Slide down
            _ => None,
        };
    }

    fn mod_xm_walk_effect(
        freq_type: FrequencyType,
        current: &PatternSlot,
    ) -> Option<Vec<TrackImportEffect>> {
        match current.effect_type {
            0x00 => {
                let param = current.effect_parameter;
                if param > 0 {
                    let v1 = (param >> 4) as f32;
                    let v2 = (param & 0x0F) as f32;
                    return Some(vec![TrackImportEffect::Arpeggio(v1, v2)]);
                } else {
                    return None;
                }
            }
            0x01 => {
                return Some(vec![TrackImportEffect::PortamentoUp(
                    -4.0 * current.effect_parameter as f32,
                )])
            }
            0x02 => {
                return Some(vec![TrackImportEffect::PortamentoDown(
                    4.0 * current.effect_parameter as f32,
                )])
            }
            0x03 => {
                let param = current.effect_parameter as f32;
                let speed = match freq_type {
                    FrequencyType::LinearFrequencies => 4.0 * param,
                    FrequencyType::AmigaFrequencies => param,
                };
                return Some(vec![TrackImportEffect::TonePortamento(speed)]);
            }
            0x04 => {
                let param = current.effect_parameter;
                let speed = ((param & 0xF0) >> 4) as f32 / 64.0;
                let depth = (param & 0x0F) as f32 / 16.0;
                return Some(vec![TrackImportEffect::Vibrato(speed, depth)]);
            }
            0x05 => {
                let vs = Self::mod_xm_volume_slide(current.effect_parameter);
                if let Some(vste) = vs {
                    return Some(vec![
                        TrackImportEffect::TonePortamento(0.0),
                        TrackImportEffect::VolumeSlideN(vste),
                    ]);
                } else {
                    return Some(vec![
                        TrackImportEffect::TonePortamento(0.0),
                        TrackImportEffect::VolumeSlideN(0.0),
                    ]);
                }
            }
            0x06 => {
                if let Some(vste) = Self::mod_xm_volume_slide(current.effect_parameter) {
                    return Some(vec![
                        TrackImportEffect::Vibrato(0.0, 0.0),
                        TrackImportEffect::VolumeSlideN(vste),
                    ]);
                } else {
                    return Some(vec![
                        TrackImportEffect::Vibrato(0.0, 0.0),
                        TrackImportEffect::VolumeSlideN(0.0),
                    ]);
                }
            }
            0x07 => {
                let param = current.effect_parameter;
                let speed = ((param & 0xF0) >> 4) as f32 / 64.0;
                let depth = (param & 0x0F) as f32 / 16.0;
                return Some(vec![TrackImportEffect::Tremolo(speed, depth)]);
            }
            0x08 => {
                return Some(vec![TrackImportEffect::ChannelPanning(
                    (current.effect_parameter as f32) / 255.0,
                )])
            }
            0x09 => {
                return Some(vec![TrackImportEffect::InstrumentSampleOffset(
                    current.effect_parameter as usize * 256,
                )])
            }
            0x0A => {
                if let Some(vste) = Self::mod_xm_volume_slide(current.effect_parameter) {
                    return Some(vec![TrackImportEffect::VolumeSlideN(vste)]);
                } else {
                    return None;
                }
            }
            0x0C => {
                return Some(vec![TrackImportEffect::Volume(
                    (current.effect_parameter.max(64) as f32) / 64.0,
                    0,
                )])
            }
            0x0E => {
                let param = current.effect_parameter & 0x0F;
                match current.effect_parameter >> 4 {
                    0x1 => {
                        return (param != 0).then(|| {
                            vec![TrackImportEffect::PortamentoFineUp(-4.0 * param as f32)]
                        })
                    }
                    0x2 => {
                        return (param != 0).then(|| {
                            vec![TrackImportEffect::PortamentoFineDown(4.0 * param as f32)]
                        })
                    }
                    0x3 => {
                        return Some(vec![TrackImportEffect::Glissando(
                            current.effect_parameter != 0,
                        )])
                    }
                    0x4 => {
                        let waveform = match param & 0b0000_0011 {
                            1 => Waveform::RampDown,
                            2 => Waveform::Square,
                            _ => Waveform::Sine,
                        };
                        let retrig = (param & 0b0000_0100) == 0;
                        return Some(vec![TrackImportEffect::VibratoWaveform(waveform, retrig)]);
                    }
                    0x5 => {
                        return Some(vec![TrackImportEffect::InstrumentFineTune(
                            param as f32 / 8.0 - 1.0,
                        )])
                    }
                    0x7 => {
                        let waveform = match param & 0b0000_0011 {
                            1 => Waveform::RampDown,
                            2 => Waveform::Square,
                            _ => Waveform::Sine,
                        };
                        let retrig = (param & 0b0000_0100) == 0;
                        return Some(vec![TrackImportEffect::TremoloWaveform(waveform, retrig)]);
                    }
                    0x9 => return Some(vec![TrackImportEffect::NoteRetrig(param as usize)]),
                    0xA => {
                        return Some(vec![TrackImportEffect::VolumeSlide0((param as f32) / 64.0)])
                    }
                    0xB => {
                        return Some(vec![TrackImportEffect::VolumeSlide0(
                            -(param as f32) / 64.0,
                        )])
                    }
                    0xC => return Some(vec![TrackImportEffect::NoteCut(param as usize, false)]),
                    0xD => return Some(vec![TrackImportEffect::NoteDelay(param as usize)]),
                    _ => return None,
                }
            }
            // Kxx: Key off
            0x14 => {
                return Some(vec![TrackImportEffect::NoteOff(
                    current.effect_parameter as usize,
                    false,
                )])
            }
            // Lxx: Set envelope position
            0x15 => {
                return Some(vec![TrackImportEffect::InstrumentVolumeEnvelopePosition(
                    current.effect_parameter as usize,
                )])
            }
            // Pxy: Panning slide
            0x19 => {
                let param = current.effect_parameter;
                let upper_nibble = param & 0xF0;
                let lower_nibble = param & 0x0F;

                return match (upper_nibble, lower_nibble) {
                    (f, 0) => Some(vec![TrackImportEffect::ChannelPanningSlideN(
                        (f >> 4) as f32 / 16.0,
                    )]), // Slide up
                    (0, f) => Some(vec![TrackImportEffect::ChannelPanningSlideN(
                        -(f as f32) / 16.0,
                    )]), // Slide down
                    _ => None,
                };
            }
            // Rxy: Multi retrig note
            0x1B => {
                let param = current.effect_parameter;
                let vol = (param >> 4) as f32 / 16.0;
                let speed = param & 0x0F;
                return Some(vec![TrackImportEffect::NoteRetrigExtended(
                    speed as usize,
                    vol,
                )]);
            }
            // Txy: Tremor
            // Rapidly switches the sample volume on and off on every tick of the row except the first.
            // Volume is on for x + 1 ticks and off for y + 1 ticks.
            // tremor_on: bool = [(T-1) % (X+1+Y+1) ] > X
            0x1D => {
                let param = current.effect_parameter;
                let on_time = param >> 4;
                let off_time = param & 0x0F;
                return Some(vec![TrackImportEffect::Tremor(
                    on_time as usize,
                    off_time as usize,
                )]);
            }
            // X1 & X2
            0x21 => {
                let effect_case = current.effect_parameter >> 4;
                let param = current.effect_parameter & 0x0F;

                return match effect_case {
                    1 => {
                        // X1y: Extra fine portamento up
                        Some(vec![TrackImportEffect::PortamentoExtraFineUp(
                            -(param as f32),
                        )])
                    }
                    2 => {
                        // X2y: Extra fine portamento down
                        Some(vec![TrackImportEffect::PortamentoExtraFineDown(
                            param as f32,
                        )])
                    }
                    _ => None,
                };
            }
            _ => return None,
        }
    }

    fn mod_xm_walk_volume(
        freq_type: FrequencyType,
        current: &PatternSlot,
    ) -> Option<TrackImportEffect> {
        match current.volume {
            0x0 => return None,
            // V - Set volume (0..63)
            0x1..=0x4 => {
                return Some(TrackImportEffect::Volume(
                    (current.volume - 0x10) as f32 / 64.0,
                    0,
                ))
            }
            // V - 0x51..0x5F undefined...
            0x5 => {
                return Some(TrackImportEffect::Volume(
                    (current.volume - 0x20) as f32 / 64.0,
                    0,
                ))
            }
            // - - Volume slide down (0..15)
            0x6 => {
                return Some(TrackImportEffect::VolumeSlide0(
                    -((current.volume & 0x0F) as f32) / 64.0,
                ))
            }
            // + - Volume slide up (0..15)
            0x7 => {
                return Some(TrackImportEffect::VolumeSlide0(
                    ((current.volume & 0x0F) as f32) / 64.0,
                ))
            }
            // D - Fine volume slide down (0..15)
            0x8 => {
                return Some(TrackImportEffect::VolumeSlideN(
                    -((current.volume & 0x0F) as f32) / 64.0,
                ))
            }
            // U - Fine volume slide up (0..15)
            0x9 => {
                return Some(TrackImportEffect::VolumeSlideN(
                    ((current.volume & 0x0F) as f32) / 64.0,
                ))
            }
            // S - Vibrato speed (0..15)
            0xA => {
                let s = current.volume & 0x0F;
                if s != 0 {
                    return Some(TrackImportEffect::VibratoSpeed((s as f32) / 64.0));
                } else {
                    return None;
                }
            }
            // V - Vibrato depth (0..15)
            0xB => {
                let d = current.volume & 0x0F;
                if d != 0 {
                    return Some(TrackImportEffect::VibratoDepthFxVol((d as f32) / 16.0));
                } else {
                    return None;
                }
            }
            // P - Set panning
            0xC => {
                return Some(TrackImportEffect::ChannelPanning(
                    (current.volume as f32) / 16.0,
                ))
            }
            // L - Panning slide left
            0xD => {
                return Some(TrackImportEffect::ChannelPanningSlideN(
                    -(current.volume as f32) / 16.0,
                ))
            }
            // R - Panning slide right
            0xE => {
                return Some(TrackImportEffect::ChannelPanningSlideN(
                    (current.volume as f32) / 16.0,
                ))
            }
            // M - Tone portamento (0..15)
            0xF => {
                let speed = (current.volume & 0x0F) as f32 * 16.0;
                let speed = match freq_type {
                    FrequencyType::LinearFrequencies => 4.0 * speed,
                    FrequencyType::AmigaFrequencies => speed,
                };
                return Some(TrackImportEffect::TonePortamentoFxVol(speed));
            }
            _ => {
                return None;
            }
        }
    }

    fn mod_xm_walk_global_effect(current: &PatternSlot) -> Option<GlobalEffect> {
        match current.effect_type {
            // Bxx: Position jump
            0x0B => {
                // TODO: counts from 1, so 0 and 1 jump to zero...
                return Some(GlobalEffect::PositionJump(
                    current.effect_parameter as usize,
                ));
            }
            // Dxx: Pattern break
            0x0D => {
                let param = current.effect_parameter;
                let ten = (param >> 4) as usize * 10;
                let unit = (param & 0x0F) as usize;
                let total = ten + unit;
                return Some(GlobalEffect::PatternBreak(total));
            }
            // EXy: Extended command
            0x0E => {
                let param = current.effect_parameter & 0x0F;
                match current.effect_parameter >> 4 {
                    // E6y: Pattern loop
                    0x6 => return Some(GlobalEffect::PatternLoop(param as usize)),
                    // EEy: Pattern delay
                    0xE => Some(GlobalEffect::PatternDelay(param as usize, true)),
                    _ => None,
                }
            }
            // Fxx: Set speed/BPM
            0x0F => {
                let param = current.effect_parameter;
                if param < 32 {
                    return Some(GlobalEffect::Speed(param as usize));
                } else {
                    return Some(GlobalEffect::Bpm(param as usize));
                }
            }
            // Gxx: Set global volume
            0x10 => {
                return Some(GlobalEffect::Volume(
                    (current.effect_parameter.max(64) as f32) / 64.0,
                ))
            }
            // Hxy: Global volume slide
            0x11 => {
                let param = current.effect_parameter;
                let upper_nibble = param & 0xF0;
                let lower_nibble = param & 0x0F;

                return match (upper_nibble, lower_nibble) {
                    (0, f) => Some(GlobalEffect::VolumeSlide(-(f as f32) / 64.0, false)), // Slide down
                    (f, 0) => Some(GlobalEffect::VolumeSlide((f >> 4) as f32 / 64.0, false)), // Slide up
                    _ => None,
                };
            }
            _ => return None,
        }
    }

    pub fn mod_xm_unpack(freq_type: FrequencyType, current: &PatternSlot) -> TrackImportUnit {
        let te = Self::mod_xm_walk_effect(freq_type, current);
        let ve = Self::mod_xm_walk_volume(freq_type, current);
        let cc = Self::mod_xm_walk_global_effect(current);

        let mut tiu = TrackImportUnit::default();
        tiu.note = current.note;
        tiu.instrument = current.instrument;
        if let Some(e) = ve {
            tiu.effects.push(e);
        }
        if let Some(e) = te {
            tiu.effects.extend(e)
        }

        if let Some(c) = cc {
            tiu.global_effects.push(c);
        }
        tiu
    }

    pub fn mod_xm_unpack_row(
        freq_type: FrequencyType,
        row: &Vec<PatternSlot>,
    ) -> Vec<TrackImportUnit> {
        row.iter()
            .map(|pattern_slot| ModXmEffect::mod_xm_unpack(freq_type, &pattern_slot))
            .collect()
    }

    pub fn mod_xm_unpack_pattern(
        freq_type: FrequencyType,
        pattern: &Vec<Vec<PatternSlot>>,
    ) -> Vec<Vec<TrackImportUnit>> {
        pattern
            .iter()
            .map(|row| Self::mod_xm_unpack_row(freq_type, &row))
            .collect()
    }
}
