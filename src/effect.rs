use crate::{prelude::NewNoteAction, waveform::Waveform};
use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
#[allow(unused_imports)]
use micromath::F32Ext;
#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::float::Float;

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrackEffect {
    /// Pitch effect
    Arpeggio {
        /// 1st halftone
        half1: usize,
        /// 2nd halftone
        half2: usize,
    },

    /// `position` [0.0..1.0], sets the panning position for the channel
    /// 0.0 is the leftmost position and 1.0 the rightmost
    /// Panning effect
    ChannelPanning(f32),

    /// `(speed, tick)`, this effect slide the panning position
    /// Panning effect
    ChannelPanningSlide {
        speed: f32,
        /// if true, only at tick0, otherwise from tick1
        fine: bool,
    },

    /// `value`, set the Channel Volume
    /// Channel Volume effect
    ChannelVolume(f32),

    // `(speed, tick)`, slides the current channel volume up or down
    /// Channel Volume effect
    ChannelVolumeSlide {
        speed: f32,
        /// if true, only at tick0, otherwise from tick1
        fine: bool,
    },

    /// `bool`, round to the nearest halftone when using effects
    /// Pitch effect
    Glissando(bool),

    /// `tune`, this effet should be used together with a note.
    /// It will cause another fine-tune value to be used
    /// Pitch effect
    InstrumentFineTune(f32),

    /// `nna` Change Instrument New Note Action
    /// Misc effect
    InstrumentNewNoteAction(NewNoteAction),

    /// `position`, change the panning envelope position
    /// Panning effect
    InstrumentPanningEnvelopePosition(usize),

    /// `activate`, set the panning envelope on
    /// Panning effect
    InstrumentPanningEnvelope(bool),

    /// `activate`, set the pitch envelope on
    /// Pitch effect
    InstrumentPitchEnvelope(bool),

    /// `offset`, this effect should be used together with a note
    /// The sample will be played from `offset` instead of zero
    /// Misc effect
    InstrumentSampleOffset(usize),

    /// `surround`
    /// Misc effect
    InstrumentSurround(bool),

    /// `position`, change the volume envelope position
    /// Volume effect
    InstrumentVolumeEnvelopePosition(usize),

    /// `activate`, set the volume envelope on
    /// Volume effect
    InstrumentVolumeEnvelope(bool),

    /// `(tick, past)`, cut the note at the specified tick.
    /// if past is true, do it for past note too
    /// Note that it will only set the volume to zero, and the sample will still be played.
    /// Volume effect
    NoteCut {
        /// cut at tick
        tick: usize,
        /// do it for past note too
        past: bool,
    },

    /// `ticks`, this effect will delay the note the selected number of ticks
    /// Misc effect
    NoteDelay(usize),

    /// `(tick, past)`, fadeout the note at the specified tick
    /// if past is true, do it for past note too
    /// Volume effect
    NoteFadeOut {
        /// fadeout the note at the specified tick
        tick: usize,
        /// do it for past note too
        past: bool,
    },

    /// `tick`, this effect will trigger a "Note Off" at the specified tick
    /// if past is true, do it for past note too
    /// Misc effect
    NoteOff {
        /// off the note at the specified tick
        tick: usize,
        /// do it for past note too
        past: bool,
    },

    /// `interval`, this effect will retrigs the note with the specified interval
    /// Misc effect
    NoteRetrig(usize),

    /// `(interval, volume change)`
    /// Extended version of the `TrackEffect::NoteRetrig` effect
    /// Misc effect
    NoteRetrigExtended { interval: usize, volume_change: f32 },

    /// `(speed, depth)`, set Panbrello
    /// Panning effect
    Panbrello { speed: f32, depth: f32 },

    /// `(waveform, retrig)`, change Panbrello waveform.
    /// Panning effect
    PanbrelloWaveform {
        waveform: Waveform,
        /// retrig when a new instrument is played.
        retrig: bool,
    },

    /// `speed`
    /// Pitch effect
    Portamento(f32),

    /// `speed`, portamento to note at speed.
    /// see `::Glissando` to round to the nearest halftone
    /// Pitch efect
    TonePortamento(f32),

    /// `(speed, depth)`
    /// Volume effect
    Tremolo { speed: f32, depth: f32 },

    /// `(waveform, retrig)`, change Tremolo waveform.
    /// Volume effect
    TremoloWaveform {
        waveform: Waveform,
        /// retrig when a new instrument is played.
        retrig: bool,
    },

    /// `(On time, Off time)`
    /// This weird effect will set the volume to zero during `Off time` number of ticks
    /// Volume effect
    Tremor {
        on_time: usize,
        /// set the volume to zero during off time numbre of ticks
        off_time: usize,
    },

    /// `(speed, depth)`, set Vibrato
    /// Pitch effect
    Vibrato { speed: f32, depth: f32 },

    /// `speed`, set Vibrato speed
    /// Pitch effect
    VibratoSpeed(f32),

    /// `depth`, set Vibrato depth
    /// Pitch effect
    VibratoDepth(f32),

    /// `(waveform, retrig)`, change Vibrato waveform.
    /// Pitch effect
    VibratoWaveform {
        waveform: Waveform,
        /// retrig when a new instrument is played.
        retrig: bool,
    },

    /// `(value, tick)`, set the current volume at the current tick
    /// Volume effect
    Volume {
        value: f32,
        /// set at the current tick
        tick: usize,
    },

    /// `(speed, tick)`, slides the current volume up or down
    /// Volume effect
    VolumeSlide {
        speed: f32,
        /// if true, only at tick0, otherwise from tick1
        fine: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MidiMacroType {
    // None: use the last index
    Parametric(Option<usize>),
    Fixed(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GlobalEffect {
    /// `bpm` set the BPM of the song
    Bpm(usize),

    /// `value` slide the BPM on every tick of the row
    BpmSlide(isize),

    /// Midi macro
    MidiMacro(MidiMacroType),

    /// `pattern-position`, jump to the next pattern and play from the specified position.
    PatternBreak(usize),

    /// `(quantity, unit)`, this effect will delay the row the selected number of units.
    /// if `unit` is true, `unit` is on tempo, `else` unit is on tick
    /// If multiple commands are on the same row, the sum of their parameters is used.
    ///
    /// All effects in the row will be repeated for each row delayed.
    ///
    /// if `unit` is true (tempo unit)
    ///     does not delay for the sum of all commands in in the row
    /// else
    ///     delays by the sum of that row
    PatternDelay {
        quantity: usize,
        /// if true, `unit` is on tempo, `else` unit is on tick
        tempo: bool,
    },

    /// `value`, if value is zero, set the loopbackpoint
    /// is value is a non-zero, loop from previous loopback point value times
    PatternLoop(usize),

    /// `position`, jump to the selected song position and play the pattern from the beginning
    PositionJump(usize),

    /// `speed` set the speed of the song
    Speed(usize),

    /// `value`, set the global Volume
    Volume(f32),

    /// `(speed, tick)`, slides the global volume up or down
    /// if tick is true, only at tick0, otherwise from tick1
    /// Volume
    VolumeSlide { speed: f32, fine: bool },
}

impl TrackEffect {
    pub fn merge(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (
                TrackEffect::Arpeggio {
                    half1: h1,
                    half2: h2,
                },
                TrackEffect::Arpeggio {
                    half1: o1,
                    half2: o2,
                },
            ) => Some(TrackEffect::Arpeggio {
                half1: h1 + o1,
                half2: h2 + o2,
            }),
            (TrackEffect::ChannelPanning(h1), TrackEffect::ChannelPanning(o1)) => {
                Some(TrackEffect::ChannelPanning(h1 + o1))
            }
            (
                TrackEffect::ChannelPanningSlide {
                    speed: h1,
                    fine: _tick1,
                },
                TrackEffect::ChannelPanningSlide {
                    speed: o1,
                    fine: tick2,
                },
            ) => Some(TrackEffect::ChannelPanningSlide {
                speed: h1 + o1,
                fine: *tick2,
            }),
            (TrackEffect::ChannelVolume(value1), TrackEffect::ChannelVolume(value2)) => Some(
                TrackEffect::ChannelVolume((value1 + value2).clamp(0.0, 1.0)),
            ),
            (
                TrackEffect::ChannelVolumeSlide {
                    speed: h1,
                    fine: _tick1,
                },
                TrackEffect::ChannelVolumeSlide {
                    speed: o1,
                    fine: tick2,
                },
            ) => Some(TrackEffect::ChannelVolumeSlide {
                speed: h1 + o1,
                fine: *tick2,
            }),
            (TrackEffect::Glissando(_h1), TrackEffect::Glissando(o1)) => {
                Some(TrackEffect::Glissando(*o1)) // overwrite glissando value.
            }
            (TrackEffect::InstrumentFineTune(h1), TrackEffect::InstrumentFineTune(o1)) => {
                Some(TrackEffect::InstrumentFineTune(h1 + o1))
            }
            (
                TrackEffect::InstrumentPanningEnvelopePosition(h1),
                TrackEffect::InstrumentPanningEnvelopePosition(o1),
            ) => Some(TrackEffect::InstrumentPanningEnvelopePosition(h1 + o1)),
            (TrackEffect::InstrumentSampleOffset(h1), TrackEffect::InstrumentSampleOffset(o1)) => {
                Some(TrackEffect::InstrumentSampleOffset(h1 + o1))
            }
            (
                TrackEffect::InstrumentVolumeEnvelopePosition(h1),
                TrackEffect::InstrumentVolumeEnvelopePosition(o1),
            ) => Some(TrackEffect::InstrumentVolumeEnvelopePosition(h1 + o1)),
            (
                TrackEffect::NoteCut {
                    tick: h1,
                    past: _h2,
                },
                TrackEffect::NoteCut { tick: o1, past: o2 },
            ) => Some(TrackEffect::NoteCut {
                tick: h1 + o1,
                past: *o2,
            }),
            (TrackEffect::NoteDelay(h1), TrackEffect::NoteDelay(o1)) => {
                Some(TrackEffect::NoteDelay(h1 + o1))
            }
            (
                TrackEffect::NoteFadeOut {
                    tick: h1,
                    past: _h2,
                },
                TrackEffect::NoteFadeOut { tick: o1, past: o2 },
            ) => Some(TrackEffect::NoteFadeOut {
                tick: h1 + o1,
                past: *o2,
            }),
            (
                TrackEffect::NoteOff {
                    tick: h1,
                    past: _h2,
                },
                TrackEffect::NoteOff { tick: o1, past: o2 },
            ) => Some(TrackEffect::NoteOff {
                tick: h1 + o1,
                past: *o2,
            }),
            (TrackEffect::NoteRetrig(h1), TrackEffect::NoteRetrig(o1)) => {
                Some(TrackEffect::NoteRetrig(h1 + o1))
            }
            (
                TrackEffect::NoteRetrigExtended {
                    interval: h1,
                    volume_change: h2,
                },
                TrackEffect::NoteRetrigExtended {
                    interval: o1,
                    volume_change: o2,
                },
            ) => Some(TrackEffect::NoteRetrigExtended {
                interval: h1 + o1,
                volume_change: h2 + o2,
            }),
            (
                TrackEffect::Panbrello {
                    speed: h1,
                    depth: h2,
                },
                TrackEffect::Panbrello {
                    speed: o1,
                    depth: o2,
                },
            ) => Some(TrackEffect::Panbrello {
                speed: h1 + o1,
                depth: h2 + o2,
            }),
            (
                TrackEffect::PanbrelloWaveform {
                    waveform: _h1,
                    retrig: _h2,
                },
                TrackEffect::PanbrelloWaveform {
                    waveform: o1,
                    retrig: o2,
                },
            ) => {
                Some(TrackEffect::PanbrelloWaveform {
                    waveform: o1.clone(),
                    retrig: *o2,
                }) // overwrite values
            }
            (TrackEffect::Portamento(h1), TrackEffect::Portamento(o1)) => {
                Some(TrackEffect::Portamento(h1 + o1))
            }
            (TrackEffect::TonePortamento(h1), TrackEffect::TonePortamento(o1)) => {
                Some(TrackEffect::TonePortamento(h1 + o1))
            }
            (
                TrackEffect::Tremolo {
                    speed: h1,
                    depth: h2,
                },
                TrackEffect::Tremolo {
                    speed: o1,
                    depth: o2,
                },
            ) => Some(TrackEffect::Tremolo {
                speed: h1 + o1,
                depth: h2 + o2,
            }),
            (
                TrackEffect::TremoloWaveform {
                    waveform: _h1,
                    retrig: _h2,
                },
                TrackEffect::TremoloWaveform {
                    waveform: o1,
                    retrig: o2,
                },
            ) => {
                Some(TrackEffect::TremoloWaveform {
                    waveform: o1.clone(),
                    retrig: *o2,
                }) // overwrite values
            }
            (
                TrackEffect::Tremor {
                    on_time: h1,
                    off_time: h2,
                },
                TrackEffect::Tremor {
                    on_time: o1,
                    off_time: o2,
                },
            ) => Some(TrackEffect::Tremor {
                on_time: h1 + o1,
                off_time: h2 + o2,
            }),
            (
                TrackEffect::Vibrato {
                    speed: h1,
                    depth: h2,
                },
                TrackEffect::Vibrato {
                    speed: o1,
                    depth: o2,
                },
            ) => Some(TrackEffect::Vibrato {
                speed: h1 + o1,
                depth: h2 + o2,
            }),
            (TrackEffect::VibratoSpeed(h1), TrackEffect::VibratoSpeed(o1)) => {
                Some(TrackEffect::VibratoSpeed(h1 + o1))
            }
            (TrackEffect::VibratoDepth(h1), TrackEffect::VibratoDepth(o1)) => {
                Some(TrackEffect::VibratoDepth(h1 + o1))
            }
            (
                TrackEffect::VibratoWaveform {
                    waveform: _h1,
                    retrig: _h2,
                },
                TrackEffect::VibratoWaveform {
                    waveform: o1,
                    retrig: o2,
                },
            ) => {
                Some(TrackEffect::VibratoWaveform {
                    waveform: o1.clone(),
                    retrig: *o2,
                }) // overwrite values
            }
            (
                TrackEffect::Volume {
                    value: value1,
                    tick: tick1,
                },
                TrackEffect::Volume {
                    value: value2,
                    tick: tick2,
                },
            ) => Some(TrackEffect::Volume {
                value: (value1 + value2).clamp(0.0, 1.0),
                tick: tick1 + tick2,
            }),
            (
                TrackEffect::VolumeSlide {
                    speed: speed1,
                    fine: _tick_based1,
                },
                TrackEffect::VolumeSlide {
                    speed: speed2,
                    fine: tick_based2,
                },
            ) => {
                Some(TrackEffect::VolumeSlide {
                    speed: speed1 + speed2,
                    fine: *tick_based2,
                }) // overwrite tick_based
            }
            _ => None,
        }
    }
}
