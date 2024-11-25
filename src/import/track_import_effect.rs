/// Generic Effect enum Helper to parse and record memory data definitively to TrackEffect.
use crate::{effect::TrackEffect, waveform::Waveform};
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
#[allow(unused_imports)]
use micromath::F32Ext;
#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::float::Float;

// This step is mandatory to manage the effects memory before TrackEffect...
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TrackImportEffect {
    /// `(1st halftone, 2nd halftone)`
    /// F / XM0=0(0), XM=0(0)
    Arpeggio(f32, f32),

    /// `position` [0.0..1.0], sets the panning position for the channel
    /// 0.0 is the leftmost position and 1.0 the rightmost
    /// P / XM0=8(8), XM0=v0xC(vP)
    ChannelPanning(f32),

    /// `speed`, this effect slide the panning position
    /// P / XM0=0x19(P), XM=0x19(P), XM0=v0xD(L), XM0=v0xE(R)
    ChannelPanningSlide(f32),

    /// `value`, set the Channel Volume
    /// Channel Volume
    ChannelVolume(f32),

    /// `value`, slide the Channel Volume
    /// Channel Volume
    ChannelVolumeSlide(f32),

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

    /// `(interval, volume change)`
    /// Extended version of the `TrackImportEffectffect::NoteRetrig` effect
    /// V / XM0=0x1B(R), XM=0x1B(R)
    NoteRetrigExtended(usize, f32),

    /// `(speed, depth)`, set Panbrello
    Panbrello(f32, f32),

    /// `(waveform, retrig)`, change Panbrello waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    PanbrelloWaveform(Waveform, bool),

    /// `speed`
    /// F / XM0=1(1), XM=1(1), XM0=2(2), XM=2(2), XM0=0xE1(E1), XM0=0xE2(E2), XM0=0x21(X1), XM=0x21(X2)
    PortamentoUp(f32),
    PortamentoDown(f32),

    /// `speed`
    /// F / XM0=0xE1(E1), XM0=0xE2(E2)
    PortamentoFineUp(f32),
    PortamentoFineDown(f32),

    /// `speed`
    /// F / XM0=0x21(X1), XM=0x21(X2)
    PortamentoExtraFineUp(f32),
    PortamentoExtraFineDown(f32),

    /// `speed`, see `ControlChangeEffect::Glissando` to round to the nearest halftone
    /// F / XM0=3(3), XM=3(3), XM0=5x?(5), XM=5x?(5), XM0=v0xF(vM), XM=v0xF(vM)
    TonePortamento(f32),
    TonePortamentoFxVol(f32),

    /// `(speed, depth)`, see `ControlChangeEffect::Waveform` to change waveform
    /// V / XM0=7(7), XM=7(7)
    Tremolo(f32, f32),

    /// `(waveform, retrig)`, change Tremolo waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// V / XM0=0xE7(E7)
    TremoloWaveform(Waveform, bool),

    /// `(On time, Off time)`
    /// This weird effect will set the volume to zero during `Off time` number of ticks
    /// V / XM0=0x1D(T), XM=0x1D(T)
    Tremor(usize, usize),

    /// `(speed, depth)`, set Vibrato
    /// F / XM0=4(4), XM=4(4), XM0=6x?(6), XM=6x?(6), XM=v0xB(vV)
    Vibrato(f32, f32),

    /// `(speed, depth)`, set Vibrato
    /// IT with memory
    VibratoFine(f32, f32),

    /// `speed`, set Vibrato speed
    /// F / XM0=v0xA(S)
    VibratoSpeed(f32),

    /// `depth`, set Vibrato depth
    /// F / XM0=v0xB(V)
    VibratoDepth(f32),
    VibratoDepthFxVol(f32),

    /// `(waveform, retrig)`, change Vibrato waveform.
    /// `retrig` to true to retrig when a new instrument is played.
    /// F / XM0=0xE4(E4)
    VibratoWaveform(Waveform, bool),

    /// `(value, tick)`, sets the current volume at the current tick
    /// V / XM0=C(C), XM0=vV1..5(V)
    Volume(f32, usize),

    /// `(speed, tick)`, slides the current volume up or down
    /// V / XM0=5?y(5), XM=5?y(5), XM0=6?y(6), XM=6?y(6), XM=0xA(A), XM0=0xEA(EA), XM0=0xEB(EB), XM=v6(v6), XM=v7(v7), XM0=v8(v8), XM0=v9(v9)
    VolumeSlide0(f32),
    VolumeSlideN(f32),
}

impl TrackImportEffect {
    fn sum_vibrato_depth(effects: &[TrackImportEffect]) -> Option<TrackEffect> {
        let s = effects
            .iter()
            .map(|effect| match effect {
                TrackImportEffect::VibratoDepth(depth)
                | TrackImportEffect::VibratoDepthFxVol(depth) => *depth,
                _ => 0.0, // Ignore other variants
            })
            .sum();

        if s == 0.0 {
            return None;
        } else {
            return Some(TrackEffect::VibratoDepth(s));
        }
    }

    fn sum_vibratos(effects: &[TrackImportEffect]) -> Option<TrackEffect> {
        let s = effects
            .iter()
            .map(|effect| match effect {
                TrackImportEffect::Vibrato(speed, depth)
                | TrackImportEffect::VibratoFine(speed, depth) => (*speed, *depth),
                _ => (0.0, 0.0), // Ignore other variants
            })
            .fold((0.0, 0.0), |acc, (a, b)| (acc.0 + a, acc.1 + b));

        if s.0 == 0.0 || s.1 == 0.0 {
            return None;
        } else {
            return Some(TrackEffect::Vibrato(s.0, s.1));
        }
    }

    fn sum_tone_portamento(effects: &[TrackImportEffect]) -> Option<TrackEffect> {
        let s = effects
            .iter()
            .map(|effect| match effect {
                TrackImportEffect::TonePortamento(speed)
                | TrackImportEffect::TonePortamentoFxVol(speed) => *speed,
                _ => 0.0, // Ignore other variants
            })
            .sum();

        if s == 0.0 {
            return None;
        } else {
            return Some(TrackEffect::TonePortamento(s));
        }
    }

    fn sum_portamento_speeds(effects: &[TrackImportEffect]) -> Option<TrackEffect> {
        let s = effects
            .iter()
            .map(|effect| match effect {
                TrackImportEffect::PortamentoUp(speed)
                | TrackImportEffect::PortamentoDown(speed)
                | TrackImportEffect::PortamentoFineUp(speed)
                | TrackImportEffect::PortamentoFineDown(speed)
                | TrackImportEffect::PortamentoExtraFineUp(speed)
                | TrackImportEffect::PortamentoExtraFineDown(speed) => *speed,
                _ => 0.0, // Ignore other variants
            })
            .sum();

        if s == 0.0 {
            return None;
        } else {
            return Some(TrackEffect::Portamento(s));
        }
    }

    // all others fx
    fn to_track_effect(&self) -> Option<TrackEffect> {
        match self {
            TrackImportEffect::Arpeggio(h1, h2) => Some(TrackEffect::Arpeggio(*h1, *h2)),
            TrackImportEffect::ChannelPanning(pos) => Some(TrackEffect::ChannelPanning(*pos)),
            TrackImportEffect::ChannelPanningSlide(speed) => Some(TrackEffect::ChannelPanningSlide(*speed)),
            TrackImportEffect::ChannelVolume(pos) => Some(TrackEffect::ChannelVolume(*pos)),
            TrackImportEffect::ChannelVolumeSlide(pos) => Some(TrackEffect::ChannelVolumeSlide(*pos)),
            TrackImportEffect::Glissando(value) => Some(TrackEffect::Glissando(*value)),
            TrackImportEffect::InstrumentFineTune(tune) => {
                Some(TrackEffect::InstrumentFineTune(*tune))
            }
            TrackImportEffect::InstrumentPanningEnvelopePosition(pos) => {
                Some(TrackEffect::InstrumentPanningEnvelopePosition(*pos))
            }
            TrackImportEffect::InstrumentSampleOffset(offset) => {
                Some(TrackEffect::InstrumentSampleOffset(*offset))
            }
            TrackImportEffect::InstrumentVolumeEnvelopePosition(pos) => {
                Some(TrackEffect::InstrumentVolumeEnvelopePosition(*pos))
            }
            TrackImportEffect::NoteCut(tick) => Some(TrackEffect::NoteCut(*tick)),
            TrackImportEffect::NoteDelay(ticks) => Some(TrackEffect::NoteDelay(*ticks)),
            TrackImportEffect::NoteOff(tick) => Some(TrackEffect::NoteOff(*tick)),
            TrackImportEffect::NoteRetrig(interval) => Some(TrackEffect::NoteRetrig(*interval)),
            TrackImportEffect::NoteRetrigExtended(interval, vol) => {
                Some(TrackEffect::NoteRetrigExtended(*interval, *vol))
            }
            TrackImportEffect::Panbrello(speed, depth) => {
                Some(TrackEffect::Panbrello(*speed, *depth))
            }
            TrackImportEffect::PanbrelloWaveform(waveform, retrig) => {
                Some(TrackEffect::PanbrelloWaveform(*waveform, *retrig))
            }
            TrackImportEffect::Tremolo(speed, depth) => Some(TrackEffect::Tremolo(*speed, *depth)),
            TrackImportEffect::TremoloWaveform(waveform, retrig) => {
                Some(TrackEffect::TremoloWaveform(*waveform, *retrig))
            }
            TrackImportEffect::Tremor(on, off) => Some(TrackEffect::Tremor(*on, *off)),
            TrackImportEffect::VibratoSpeed(speed) => Some(TrackEffect::VibratoSpeed(*speed)),
            TrackImportEffect::VibratoWaveform(waveform, retrig) => {
                Some(TrackEffect::VibratoWaveform(*waveform, *retrig))
            }
            TrackImportEffect::Volume(value, tick) => Some(TrackEffect::Volume(*value, *tick)),
            TrackImportEffect::VolumeSlide0(value) => Some(TrackEffect::VolumeSlide(*value, true)),
            TrackImportEffect::VolumeSlideN(value) => Some(TrackEffect::VolumeSlide(*value, false)),
            _ => None,
        }
    }

    fn remove_duplicates_and_merge(mut effects: Vec<TrackEffect>) -> Vec<TrackEffect> {
        let mut unique_effects = Vec::new();

        for effect in effects.drain(..) {
            if let Some(existing_index) = unique_effects
                .iter()
                .position(|e| core::mem::discriminant(e) == core::mem::discriminant(&effect))
            {
                if let Some(merged_effect) = unique_effects[existing_index].merge(&effect) {
                    unique_effects[existing_index] = merged_effect;
                }
            } else {
                unique_effects.push(effect);
            }
        }

        unique_effects
    }

    pub fn to_track_effects(effects: &Vec<TrackImportEffect>) -> Vec<TrackEffect> {
        let mut vte: Vec<TrackEffect> = vec![];

        // This part is used to aggregate sub-effects which are subject
        // to memory management in the different trackers

        // Portamento
        if let Some(te) = Self::sum_portamento_speeds(effects) {
            vte.push(te);
        }

        // TonePortamento
        if let Some(te) = Self::sum_tone_portamento(effects) {
            vte.push(te);
        }

        // VibratoDepth
        if let Some(te) = Self::sum_vibrato_depth(effects) {
            vte.push(te);
        }

        // Vibrato
        if let Some(te) = Self::sum_vibratos(effects) {
            vte.push(te);
        }

        // All others
        for fx in effects {
            if let Some(te) = Self::to_track_effect(fx) {
                vte.push(te);
            }
        }

        // Remove duplicates
        let vte = Self::remove_duplicates_and_merge(vte);

        return vte;
    }
}
