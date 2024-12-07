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
    /// `(1st halftone, 2nd halftone)`
    /// Pitch
    Arpeggio(f32, f32),

    /// `position` [0.0..1.0], sets the panning position for the channel
    /// 0.0 is the leftmost position and 1.0 the rightmost
    /// Panning
    ChannelPanning(f32),

    /// `(speed, tick)`, this effect slide the panning position
    /// if tick is true, only at tick0, otherwise from tick1
    /// Panning
    ChannelPanningSlide(f32, bool),

    /// `value`, set the Channel Volume
    /// Channel Volume
    ChannelVolume(f32),

    // `(speed, tick)`, slides the current channel volume up or down
    /// if tick is true, only at tick0, otherwise from tick1
    /// Channel Volume
    ChannelVolumeSlide(f32, bool),

    /// `bool`, round to the nearest halftone when using effects
    /// Pitch
    Glissando(bool),

    /// `tune`, this effet should be used together with a note.
    /// It will cause another fine-tune value to be used
    /// Pitch
    InstrumentFineTune(f32),

    /// `nna` Change Instrument New Note Action
    InstrumentNewNoteAction(NewNoteAction),

    /// `position`, change the panning envelope position
    /// Panning
    InstrumentPanningEnvelopePosition(usize),

    /// `activate`, set the panning envelope on
    /// Panning
    InstrumentPanningEnvelope(bool),

    /// `activate`, set the pitch envelope on
    /// Pitch
    InstrumentPitchEnvelope(bool),

    /// `offset`, this effect should be used together with a note
    /// The sample will be played from `offset` instead of zero
    /// Misc
    InstrumentSampleOffset(usize),

    /// `surround`
    InstrumentSurround(bool),

    /// `position`, change the volume envelope position
    /// Volume
    InstrumentVolumeEnvelopePosition(usize),

    /// `activate`, set the volume envelope on
    /// Volume
    InstrumentVolumeEnvelope(bool),

    /// `(tick, past)`, cut the note at the specified tick.
    /// if past is true, do it for past note too
    /// Note that it will only set the volume to zero, and the sample will still be played.
    /// Volume
    NoteCut(usize, bool),

    /// `ticks`, this effect will delay the note the selected number of ticks
    /// Misc
    NoteDelay(usize),

    /// `(tick, past)`, fadeout the note at the specified tick
    /// if past is true, do it for past note too
    /// Volume
    NoteFadeOut(usize, bool),

    /// `tick`, this effect will trigger a "Note Off" at the specified tick
    /// if past is true, do it for past note too
    /// Misc
    NoteOff(usize, bool),

    /// `interval`, this effect will retrigs the note with the specified interval
    /// Misc
    NoteRetrig(usize),

    /// `(interval, volume change)`
    /// Extended version of the `TrackEffect::NoteRetrig` effect
    /// Misc
    NoteRetrigExtended(usize, f32),

    /// `(speed, depth)`, set Panbrello
    /// Panning
    Panbrello(f32, f32),

    /// `(waveform, retrig)`, change Panbrello waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// Panning
    PanbrelloWaveform(Waveform, bool),

    /// `speed`
    /// Pitch
    Portamento(f32),

    /// `speed`, portamento to note at speed.
    /// see `ControlChangeEffect::Glissando` to round to the nearest halftone
    /// Pitch
    TonePortamento(f32),

    /// `(speed, depth)`, see `ControlChangeEffect::Waveform` to change waveform
    /// Volume
    Tremolo(f32, f32),

    /// `(waveform, retrig)`, change Tremolo waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// Volume
    TremoloWaveform(Waveform, bool),

    /// `(On time, Off time)`
    /// This weird effect will set the volume to zero during `Off time` number of ticks
    /// Volume
    Tremor(usize, usize),

    /// `(speed, depth)`, set Vibrato
    /// Pitch
    Vibrato(f32, f32),

    /// `speed`, set Vibrato speed
    /// Pitch
    VibratoSpeed(f32),

    /// `depth`, set Vibrato depth
    /// Pitch
    VibratoDepth(f32),

    /// `(waveform, retrig)`, change Vibrato waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// Pitch
    VibratoWaveform(Waveform, bool),

    /// `(value, tick)`, sets the current volume at the current tick
    /// Volume
    Volume(f32, usize),

    /// `(speed, tick)`, slides the current volume up or down
    /// if tick is true, only at tick0, otherwise from tick1
    /// Volume
    VolumeSlide(f32, bool),
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
    PatternDelay(usize, bool),

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
    VolumeSlide(f32, bool),
}

impl TrackEffect {
    pub fn merge(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (TrackEffect::Arpeggio(h1, h2), TrackEffect::Arpeggio(o1, o2)) => {
                Some(TrackEffect::Arpeggio(h1 + o1, h2 + o2))
            }
            (TrackEffect::ChannelPanning(h1), TrackEffect::ChannelPanning(o1)) => {
                Some(TrackEffect::ChannelPanning(h1 + o1))
            }
            (
                TrackEffect::ChannelPanningSlide(h1, _tick1),
                TrackEffect::ChannelPanningSlide(o1, tick2),
            ) => Some(TrackEffect::ChannelPanningSlide(h1 + o1, *tick2)),
            (TrackEffect::ChannelVolume(value1), TrackEffect::ChannelVolume(value2)) => Some(
                TrackEffect::ChannelVolume((value1 + value2).clamp(0.0, 1.0)),
            ),
            (
                TrackEffect::ChannelVolumeSlide(speed1, _tick1),
                TrackEffect::ChannelVolumeSlide(speed2, tick2),
            ) => Some(TrackEffect::ChannelVolumeSlide(speed1 + speed2, *tick2)),
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
            (TrackEffect::NoteCut(h1, _h2), TrackEffect::NoteCut(o1, o2)) => {
                Some(TrackEffect::NoteCut(h1 + o1, *o2))
            }
            (TrackEffect::NoteDelay(h1), TrackEffect::NoteDelay(o1)) => {
                Some(TrackEffect::NoteDelay(h1 + o1))
            }
            (TrackEffect::NoteFadeOut(h1, _h2), TrackEffect::NoteFadeOut(o1, o2)) => {
                Some(TrackEffect::NoteFadeOut(h1 + o1, *o2))
            }
            (TrackEffect::NoteOff(h1, _h2), TrackEffect::NoteOff(o1, o2)) => {
                Some(TrackEffect::NoteOff(h1 + o1, *o2))
            }
            (TrackEffect::NoteRetrig(h1), TrackEffect::NoteRetrig(o1)) => {
                Some(TrackEffect::NoteRetrig(h1 + o1))
            }
            (TrackEffect::NoteRetrigExtended(h1, h2), TrackEffect::NoteRetrigExtended(o1, o2)) => {
                Some(TrackEffect::NoteRetrigExtended(h1 + o1, h2 + o2))
            }
            (TrackEffect::Panbrello(h1, h2), TrackEffect::Panbrello(o1, o2)) => {
                Some(TrackEffect::Panbrello(h1 + o1, h2 + o2))
            }
            (TrackEffect::PanbrelloWaveform(_h1, _h2), TrackEffect::PanbrelloWaveform(o1, o2)) => {
                Some(TrackEffect::PanbrelloWaveform(o1.clone(), *o2)) // overwrite values
            }
            (TrackEffect::Portamento(h1), TrackEffect::Portamento(o1)) => {
                Some(TrackEffect::Portamento(h1 + o1))
            }
            (TrackEffect::TonePortamento(h1), TrackEffect::TonePortamento(o1)) => {
                Some(TrackEffect::TonePortamento(h1 + o1))
            }
            (TrackEffect::Tremolo(h1, h2), TrackEffect::Tremolo(o1, o2)) => {
                Some(TrackEffect::Tremolo(h1 + o1, h2 + o2))
            }
            (TrackEffect::TremoloWaveform(_h1, _h2), TrackEffect::TremoloWaveform(o1, o2)) => {
                Some(TrackEffect::TremoloWaveform(o1.clone(), *o2)) // overwrite values
            }
            (TrackEffect::Tremor(h1, h2), TrackEffect::Tremor(o1, o2)) => {
                Some(TrackEffect::Tremor(h1 + o1, h2 + o2))
            }
            (TrackEffect::Vibrato(h1, h2), TrackEffect::Vibrato(o1, o2)) => {
                Some(TrackEffect::Vibrato(h1 + o1, h2 + o2))
            }
            (TrackEffect::VibratoSpeed(h1), TrackEffect::VibratoSpeed(o1)) => {
                Some(TrackEffect::VibratoSpeed(h1 + o1))
            }
            (TrackEffect::VibratoDepth(h1), TrackEffect::VibratoDepth(o1)) => {
                Some(TrackEffect::VibratoDepth(h1 + o1))
            }
            (TrackEffect::VibratoWaveform(_h1, _h2), TrackEffect::VibratoWaveform(o1, o2)) => {
                Some(TrackEffect::VibratoWaveform(o1.clone(), *o2)) // overwrite values
            }
            (TrackEffect::Volume(value1, tick1), TrackEffect::Volume(value2, tick2)) => Some(
                TrackEffect::Volume((value1 + value2).clamp(0.0, 1.0), tick1 + tick2),
            ),
            (
                TrackEffect::VolumeSlide(speed1, _tick_based1),
                TrackEffect::VolumeSlide(speed2, tick_based2),
            ) => {
                Some(TrackEffect::VolumeSlide(speed1 + speed2, *tick_based2)) // overwrite tick_based
            }
            _ => None,
        }
    }
}
