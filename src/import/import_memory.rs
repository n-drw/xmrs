/*
Managing Dynamic Memory Effects

Here we fill the shortcuts to 0 with the correct values
​​so as not to have to manage it in the player.

It is impossible to do this update perfectly.
Indeed, patterns can be with zero effects and called
with different initialization pre-patterns.
So the last call to the pattern will be the winner.


| **MEMORY 00**                | **MOD** | **XM** | **S3M** | **IT** |
| ---------------------------- | ------- | ------ | ------- | ------ |
| **Arpeggio**                 |         |        | GLOBAL  | TRUE   |
| **Panbrello**                |         | TRUE   | TRUE    | TRUE   |
| **PanningSlide**             |         | TRUE   | TRUE    | TRUE   |
| **PortamentoUp**             |         | TRUE   | GLOBAL  | TRUE   |
| **PortamentoDown**           |         | TRUE   | GLOBAL  | TRUE   |
| **PortamentoFineUp**         |         | TRUE   | GLOBAL  | TRUE   |
| **PortamentoFineDown**       |         | TRUE   | GLOBAL  | TRUE   |
| **PortamentoFineExtraUp**    |         | TRUE   | GLOBAL  | TRUE   |
| **PortamentoFineExtraDown**  |         | TRUE   | GLOBAL  | TRUE   |
| **NoteRetrigExtended**       |         | TRUE   | GLOBAL  | TRUE   |
| **TonePortamento**           | TRUE    | TRUE   | TRUE    | TRUE   |
| **Tremolo**                  | TRUE    | TRUE   | GLOBAL  | TRUE   |
| **Tremor**                   |         | TRUE   | GLOBAL  | TRUE   |
| **SampleOffset**             | TRUE    | TRUE   | TRUE    | TRUE   |
| **Vibrato**                  | TRUE    | TRUE   | TRUE    | TRUE   |
| **VibratoFine**              |         |        |         | TRUE   |
| **VolumeSlide0**             |         | TRUE   | GLOBAL  | TRUE   | 0 EA EB v6 v7
| **VolumeSlideN**             |         | TRUE   | GLOBAL  | TRUE   | N 5 6 A v8 v9
| **FxVolume Tone Portamento** |         | TRUE   |         | FX MEM |
| **FxVolume Vibrato Depth**   |         | TRUE   |         | FX MEM |
|                              |         |        |         |        |
| **Channel Volume Slide**     |         | TRUE   | TRUE    | TRUE   |
| **global Volume Slide**      |         | TRUE   | TRUE    | TRUE   |
| **TempoUp**                  |         |        | TRUE    | TRUE   |
| **TempoDown**                |         |        | TRUE    | TRUE   |

*/

use alloc::vec;
use alloc::vec::Vec;

#[cfg(feature = "import_it")]
use crate::import::it::it_effect::ItEffect;
#[cfg(feature = "import_xm")]
use crate::import::xm::mod_xm_effect::ModXmEffect;

use super::patternslot::PatternSlot;
use super::track_import_effect::TrackImportEffect;
use super::track_import_unit::TrackImportUnit;
use crate::prelude::*;

// Memory type
pub enum MemoryType {
    Mod,
    Xm,
    S3m,
    It,
}

// Memory Array index
const ARPEGGIO: usize = 0;
const PANBRELLO: usize = 1;
const PANNING_SLIDE: usize = 2;
const PORTAMENTO_UP: usize = 3;
const PORTAMENTO_DOWN: usize = 4;
const PORTAMENTO_FINE_UP: usize = 5;
const PORTAMENTO_FINE_DOWN: usize = 6;
const PORTAMENTO_FINE_EXTRA_UP: usize = 7;
const PORTAMENTO_FINE_EXTRA_DOWN: usize = 8;
const NOTE_RETRIG_EXTENDED: usize = 9;
const TONE_PORTAMENTO: usize = 10;
const TREMOLO: usize = 11;
const TREMOR: usize = 12;
const SAMPLE_OFFSET: usize = 13;
const VIBRATO: usize = 14;
const VIBRATO_FINE: usize = 15;
const PITCH_VOLUME_SLIDE_0: usize = 16;
const PITCH_VOLUME_SLIDE_N: usize = 17;
const FX_VOLUME_TONE_PORTAMENTO: usize = 18;
const FX_VOLUME_VIBRATO_DEPTH: usize = 19;
const CHANNEL_VOLUME_SLIDE: usize = 20;
const GLOBAL_VOLUME_SLIDE: usize = 21;
const TEMPO_UP: usize = 22;
const TEMPO_DOWN: usize = 23;
const SAMPLE_OFFSET_ADD_HIGH: usize = 24;
const ARRAY_SIZE: usize = 25;

/// For compatibility between formats and to compensate for special case difficulties with memory management, a best effort analysis is performed during import to associate the exact value with each effect.
// Horrible hack to store values...use from left to right. First usable type wins.
pub struct ImportMemory {
    global: [(f32, f32, usize, usize); 64], // s3m way
    channel: [[(f32, f32, usize, usize); ARRAY_SIZE]; 64], // mod, xm, it way
}

impl Default for ImportMemory {
    fn default() -> Self {
        Self {
            global: [(0.0, 0.0, 0, 0); 64],
            channel: [[(0.0, 0.0, 0, 0); ARRAY_SIZE]; 64],
        }
    }
}

impl ImportMemory {
    fn update_memory(&mut self, mem: &MemoryType, index: usize, tiu: &mut TrackImportUnit) {
        tiu.effects.iter_mut().for_each(|e| {
            match e {
                TrackImportEffect::Arpeggio(a, b) => {
                    match mem {
                        MemoryType::Mod | MemoryType::Xm => {
                            // No memory use
                        }
                        MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][ARPEGGIO].0
                            } else {
                                self.channel[index][ARPEGGIO].0 = *a;
                            }
                            // second arg
                            if *b == 0.0 {
                                *b = self.channel[index][ARPEGGIO].1
                            } else {
                                self.channel[index][ARPEGGIO].1 = *b;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                            // second arg
                            if *b == 0.0 {
                                *b = self.global[index].1
                            } else {
                                self.global[index].1 = *b;
                            }
                        }
                    }
                }

                TrackImportEffect::Panbrello(a, b) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.channel[index][PANBRELLO].0
                            } else {
                                self.channel[index][PANBRELLO].0 = *a;
                            }
                            // second arg
                            if *b == 0.0 {
                                *b = self.channel[index][PANBRELLO].1
                            } else {
                                self.channel[index][PANBRELLO].1 = *b;
                            }
                        }
                    }
                }

                TrackImportEffect::ChannelPanningSlideN(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.channel[index][PANNING_SLIDE].0
                            } else {
                                self.channel[index][PANNING_SLIDE].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoUp(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_UP].0
                            } else {
                                self.channel[index][PORTAMENTO_UP].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoDown(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_DOWN].0
                            } else {
                                self.channel[index][PORTAMENTO_DOWN].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoFineUp(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_FINE_UP].0
                            } else {
                                self.channel[index][PORTAMENTO_FINE_UP].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoFineDown(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_FINE_DOWN].0
                            } else {
                                self.channel[index][PORTAMENTO_FINE_DOWN].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoExtraFineUp(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_FINE_EXTRA_UP].0
                            } else {
                                self.channel[index][PORTAMENTO_FINE_EXTRA_UP].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::PortamentoExtraFineDown(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PORTAMENTO_FINE_EXTRA_DOWN].0
                            } else {
                                self.channel[index][PORTAMENTO_FINE_EXTRA_DOWN].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::NoteRetrigExtended(a, b) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory use
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0 {
                                *a = self.channel[index][NOTE_RETRIG_EXTENDED].2
                            } else {
                                self.channel[index][NOTE_RETRIG_EXTENDED].2 = *a;
                            }
                            if *b == 0.0 {
                                *b = self.channel[index][NOTE_RETRIG_EXTENDED].0
                            } else {
                                self.channel[index][NOTE_RETRIG_EXTENDED].0 = *b;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0 {
                                *a = self.global[index].2
                            } else {
                                self.global[index].2 = *a;
                            }
                            if *b == 0.0 {
                                *b = self.global[index].0
                            } else {
                                self.global[index].0 = *b;
                            }
                        }
                    }
                }

                TrackImportEffect::TonePortamento(a) => match mem {
                    MemoryType::Mod | MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                        if *a == 0.0 {
                            *a = self.channel[index][TONE_PORTAMENTO].0
                        } else {
                            self.channel[index][TONE_PORTAMENTO].0 = *a;
                        }
                    }
                },

                TrackImportEffect::Tremolo(a, b) => match mem {
                    MemoryType::Mod | MemoryType::Xm | MemoryType::It => {
                        if *a == 0.0 {
                            *a = self.channel[index][TREMOLO].0
                        } else {
                            self.channel[index][TREMOLO].0 = *a;
                        }
                        if *b == 0.0 {
                            *b = self.channel[index][TREMOLO].1
                        } else {
                            self.channel[index][TREMOLO].1 = *b;
                        }
                    }
                    MemoryType::S3m => {
                        if *a == 0.0 {
                            *a = self.global[index].0
                        } else {
                            self.global[index].0 = *a;
                        }
                        if *b == 0.0 {
                            *b = self.global[index].1
                        } else {
                            self.global[index].1 = *b;
                        }
                    }
                },

                TrackImportEffect::Tremor(a, b) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0 {
                                *a = self.channel[index][TREMOR].2
                            } else {
                                self.channel[index][TREMOR].2 = *a;
                            }
                            if *b == 0 {
                                *b = self.channel[index][TREMOR].3
                            } else {
                                self.channel[index][TREMOR].3 = *b;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0 {
                                *a = self.global[index].2
                            } else {
                                self.global[index].2 = *a;
                            }
                            if *b == 0 {
                                *b = self.global[index].3
                            } else {
                                self.global[index].3 = *b;
                            }
                        }
                    }
                }

                TrackImportEffect::InstrumentSampleOffset(a) => match mem {
                    MemoryType::Mod | MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                        if *a == 0 {
                            *a = self.channel[index][SAMPLE_OFFSET].2
                                + self.channel[index][SAMPLE_OFFSET_ADD_HIGH].2;
                        } else {
                            self.channel[index][SAMPLE_OFFSET].2 = *a;
                        }
                    }
                },

                TrackImportEffect::InstrumentSampleOffsetAddHigh(a) => match mem {
                    MemoryType::Mod | MemoryType::Xm | MemoryType::S3m => {
                        // No memory
                    }
                    MemoryType::It => {
                        self.channel[index][SAMPLE_OFFSET_ADD_HIGH].2 = *a;
                    }
                },

                TrackImportEffect::Vibrato(a, b) => match mem {
                    MemoryType::Mod | MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                        if *a == 0.0 {
                            *a = self.channel[index][VIBRATO].0
                        } else {
                            self.channel[index][VIBRATO].0 = *a;
                        }
                        if *b == 0.0 {
                            *b = self.channel[index][VIBRATO].1
                        } else {
                            self.channel[index][VIBRATO].1 = *b;
                        }
                    }
                },

                TrackImportEffect::VibratoFine(a, b) => {
                    match mem {
                        MemoryType::Mod | MemoryType::Xm | MemoryType::It => {
                            // No memory
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.channel[index][VIBRATO_FINE].0
                            } else {
                                self.channel[index][VIBRATO_FINE].0 = *a;
                            }
                            if *b == 0.0 {
                                *b = self.channel[index][VIBRATO_FINE].1
                            } else {
                                self.channel[index][VIBRATO_FINE].1 = *b;
                            }
                        }
                    }
                }

                TrackImportEffect::VolumeSlide0(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PITCH_VOLUME_SLIDE_0].0
                            } else {
                                self.channel[index][PITCH_VOLUME_SLIDE_0].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::VolumeSlideN(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It => {
                            if *a == 0.0 {
                                *a = self.channel[index][PITCH_VOLUME_SLIDE_N].0
                            } else {
                                self.channel[index][PITCH_VOLUME_SLIDE_N].0 = *a;
                            }
                        }
                        MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.global[index].0
                            } else {
                                self.global[index].0 = *a;
                            }
                        }
                    }
                }

                TrackImportEffect::TonePortamentoFxVol(a) => {
                    match mem {
                        MemoryType::Mod | MemoryType::S3m => {
                            // No memory
                        }
                        MemoryType::Xm => {
                            if *a == 0.0 {
                                *a = self.channel[index][FX_VOLUME_TONE_PORTAMENTO].0
                            } else {
                                self.channel[index][FX_VOLUME_TONE_PORTAMENTO].0 = *a;
                            }
                        }
                        MemoryType::It => {
                            // never called because no use
                        }
                    }
                }

                TrackImportEffect::VibratoDepthFxVol(a) => {
                    match mem {
                        MemoryType::Mod | MemoryType::S3m => {
                            // No memory
                        }
                        MemoryType::Xm => {
                            if *a == 0.0 {
                                *a = self.channel[index][FX_VOLUME_VIBRATO_DEPTH].0
                            } else {
                                self.channel[index][FX_VOLUME_VIBRATO_DEPTH].0 = *a;
                            }
                        }
                        MemoryType::It => {
                            // never called because no use
                        }
                    }
                }

                TrackImportEffect::ChannelVolumeSlide0(a)
                | TrackImportEffect::ChannelVolumeSlideN(a) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.channel[index][CHANNEL_VOLUME_SLIDE].0
                            } else {
                                self.channel[index][CHANNEL_VOLUME_SLIDE].0 = *a;
                            }
                        }
                    }
                }

                _ => {}
            }
        });

        tiu.global_effects.iter_mut().for_each(|e| {
            match e {
                GlobalEffect::BpmSlide(a) => {
                    let fx_index = if *a < 0 { TEMPO_DOWN } else { TEMPO_UP };
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                            if *a == 0 {
                                *a = self.channel[index][fx_index].2 as isize;
                            } else {
                                self.channel[index][fx_index].2 = *a as usize;
                            }
                        }
                    }
                }
                GlobalEffect::VolumeSlide(a, _b) => {
                    match mem {
                        MemoryType::Mod => {
                            // No memory
                        }
                        MemoryType::Xm | MemoryType::It | MemoryType::S3m => {
                            if *a == 0.0 {
                                *a = self.channel[index][CHANNEL_VOLUME_SLIDE].0
                            } else {
                                self.channel[index][CHANNEL_VOLUME_SLIDE].0 = *a;
                            }
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn apply_memory(
        &mut self,
        freq_type: FrequencyType,
        mem: MemoryType,
        order: &Vec<Vec<usize>>,
        patterns: &Vec<Vec<Vec<PatternSlot>>>,
    ) -> Vec<Vec<Vec<TrackImportUnit>>> {
        let mut source: Vec<Vec<Vec<TrackImportUnit>>> = patterns
            .iter()
            .map(|pattern: &Vec<Vec<PatternSlot>>| match mem {
                #[cfg(any(feature = "import_amiga", feature = "import_xm"))]
                MemoryType::Mod | MemoryType::Xm => {
                    ModXmEffect::mod_xm_unpack_pattern(freq_type, &pattern)
                }
                #[cfg(feature = "import_s3m")]
                MemoryType::S3m => {
                    // TODO: rewrite `s3m_effect.rs` to gain overall consistency in the code
                    ModXmEffect::mod_xm_unpack_pattern(freq_type, &pattern)
                }
                #[cfg(feature = "import_it")]
                MemoryType::It => ItEffect::it_unpack_pattern(freq_type, &pattern),
                // _ => todo!(),
            })
            .collect();

        // It is impossible to guarantee that a pattern will not be designed
        // to have different pre-memory configurations.
        // We "play" the music in order and it will remain a best effort.
        order.iter().flat_map(|inner| inner.iter()).for_each(|o| {
            let pattern = if *o < source.len() {
                &mut source[*o]
            } else {
                &mut vec![]
            };
            pattern.iter_mut().for_each(|row| {
                row.iter_mut()
                    .enumerate()
                    .for_each(|(index, tiu)| self.update_memory(&mem, index, tiu));
            });
        });

        source
    }

    pub fn unpack_patterns(
        &mut self,
        freq_type: FrequencyType,
        mem: MemoryType,
        order: &Vec<Vec<usize>>,
        patterns: &Vec<Vec<Vec<PatternSlot>>>,
    ) -> Vec<Vec<Vec<TrackUnit>>> {
        /* first step: get the same patterns but using TrackImportUnit */
        let source = self.apply_memory(freq_type, mem, order, patterns);

        /* second step: prepare patterns with TrackUnit */
        let mut dest: Vec<Vec<Vec<TrackUnit>>> = source
            .iter()
            .map(|pattern| {
                pattern
                    .iter()
                    .map(|row| row.iter().map(|tiu| tiu.prepare_track_unit()).collect())
                    .collect()
            })
            .collect();

        /* third step: add effects */
        for ol in order.iter().flat_map(|inner| inner.iter()) {
            let pattern_s = if *ol < source.len() {
                &source[*ol]
            } else {
                &mut vec![]
            };

            let pattern_d = if *ol < dest.len() {
                &mut dest[*ol]
            } else {
                &mut vec![]
            };

            for (index_row, row_s) in pattern_s.iter().enumerate() {
                let row_d = &mut pattern_d[index_row];

                for (index_ch, ch_s) in row_s.iter().enumerate() {
                    let ch_d = &mut row_d[index_ch];

                    // no use of GlobalEffects::PositionJump and GlobalEffects::PatternBreak
                    // due to implementation complexity and risk of infinite loop
                    // maybe one day someone will have the motivation if a bug too important appears

                    ch_d.effects = TrackImportEffect::to_track_effects(&ch_s.effects);
                }
            }
        }
        return dest;
    }
}
