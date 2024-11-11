use crate::waveform::Waveform;
use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
#[allow(unused_imports)]
use micromath::F32Ext;
#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::float::Float;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TrackEffect {
    /// `(1st halftone, 2nd halftone)`
    /// F / XM0=0(0), XM=0(0)
    Arpeggio((f32, f32)),
    /// `bool`, round to the nearest halftone when using effects
    /// F / XM0=0xE3(E3)
    Glissando(bool),
    /// `tune`, this effet should be used together with a note.
    /// It will cause another fine-tune value to be used
    /// F / XM0=0xE5(E5)
    InstrumentFineTune(f32),
    /// `position`, change the panning envelope position
    /// P / XM0=0x15(L) (if instr.sustained)
    InstrumentPanningEnvelopePosition(usize),
    /// `offset`, this effect should be used together with a note
    /// The sample will be played from `offset` instead of zero
    /// XM0=9(9)
    InstrumentSampleOffset(usize),
    /// `position`, change the volume envelope position
    /// V / XM0=0x15(L)
    InstrumentVolumeEnvelopePosition(usize),
    /// `(speed, vol)` or (Interval, Volume Change)
    /// Extended version of the `TrackEffect::NoteRetrig` effect
    /// V / XM0=0x1B(R), XM=0x1B(R)
    MultiRetrigNote((usize, f32)),
    /// `tick`, cut the note at the specified tick.
    /// Note that it will only set the volume to zero, and the sample will still be played.
    /// V / XM=0xEC(EC)
    NoteCut(usize),
    /// `ticks`, this effect will delay the note the selected number of ticks
    /// XM0=0xED(ED), XM=0xED(ED)
    NoteDelay(usize),
    /// `tick`, this effect will trigger a "Note Off" at the specified tick
    /// XM0=0x14(K), XM=0x14(K)
    NoteOff(usize),
    /// `interval`, this effect will retrigs the note with the specified interval
    /// V / XM0=0xE9(E9), XM=0xE9(E9)
    NoteRetrig(usize),
    /// `position` [0.0..1.0], sets the panning position for the channel
    /// 0.0 is the leftmost position and 1.0 the rightmost
    /// P / XM0=8(8), XM0=v0xC(vP)
    Panning(f32),
    /// `speed`, this effect slide the panning position
    /// P / XM0=0x19(P), XM=0x19(P), XM0=v0xD(L), XM0=v0xE(R)
    PanningSlide(f32),
    /// `speed`
    /// F / XM0=1(1), XM=1(1), XM0=2(2), XM=2(2), XM0=0xE1(E1), XM0=0xE2(E2), XM0=0x21(X1), XM=0x21(X2)
    Portamento(f32),
    /// `speed`, see `ControlChangeEffect::Glissando` to round to the nearest halftone
    /// F / XM0=3(3), XM=3(3), XM0=5x?(5), XM=5x?(5), XM0=v0xF(vM), XM=v0xF(vM)
    TonePortamento(f32),
    /// `(speed, depth)`, see `ControlChangeEffect::Waveform` to change waveform
    /// V / XM0=7(7), XM=7(7)
    Tremolo((f32, f32)),
    /// `(waveform, retrig)`, change Tremolo waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// V / XM0=0xE7(E7)
    TremoloWaveform((Waveform, bool)),
    /// `(On time, Off time)`
    /// This weird effect will set the volume to zero during `Off time` number of ticks
    /// V / XM0=0x1D(T), XM=0x1D(T)
    Tremor((usize, usize)),
    /// `(speed, depth)`, set Vibrato
    /// F / XM0=4(4), XM=4(4), XM0=6x?(6), XM=6x?(6), XM=v0xB(vV)
    Vibrato((f32, f32)),
    /// `speed`, set Vibrato speed
    /// F / XM0=v0xA(S)
    VibratoSpeed(f32),
    /// `depth`, set Vibrato depth
    /// F / XM0=v0xB(V) FIXME?
    VibratoDepth(f32),
    /// `(waveform, retrig)`, change Vibrato waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// F / XM0=0xE4(E4)
    VibratoWaveform((Waveform, bool)),
    /// `(value, tick)`, sets the current volume at the current tick
    /// V / XM0=C(C), XM0=vV1..5(V)
    Volume((f32, usize)),
    /// `(speed, tick0)`, slides the current volume up or down
    /// if tick0, slide only once at tick0, else at every tick
    /// V / XM0=5?y(5), XM=5?y(5), XM0=6?y(6), XM=6?y(6), XM=0xA(A), XM0=0xEA(EA), XM0=0xEB(EB), XM=v6(v6), XM=v7(v7), XM0=v8(v8), XM0=v9(v9)
    VolumeSlide((f32, bool)),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ControlChangeEffect {
    /// `bpm` set the BPM of the song
    /// XM=0x0F(F)
    Bpm(usize),
    /// `pattern-position`, jump to the next pattern and play from the specified position.
    /// XM=0xD(D)
    PatternBreak(usize),
    /// `notes`, this effect will delay the pattern the selected nubre of notes.
    /// XM=0xEE(EE)
    PatternDelay(usize),
    /// `count`, if count is zero, the beginning of the loop will be specified
    /// When a non-zero value is used, the pattern will be looped from the loop start.
    /// XM=0xE6(E6)
    PatternLoop(usize),
    /// `position`, jump to the selected song position and play the pattern from the beginning
    /// 0x0B(B)
    PositionJump(usize),
    /// `speed` set the speed of the song
    /// XM=0x0F(F)
    Speed(usize),
    /// `value`, set the global Volume
    /// V / XM=0x10(G)
    Volume(f32),
    /// `value`, slide the global Volume
    /// V / XM=0x11(H)
    VolumeSlide(f32),
}
