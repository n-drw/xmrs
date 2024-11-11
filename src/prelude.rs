/// The Xmrs Prelude.
///
/// The purpose of this module is to alleviate imports of Xmrs module parts
///
/// ```
/// #![allow(unused_imports)]
/// use xmrs::prelude::*;
/// ```
///
pub use crate::{
    effect::{ControlChangeEffect, TrackEffect},
    envelope::{Envelope, EnvelopePoint},
    instr_default::InstrDefault,
    instr_ekn::InstrEkn,
    instr_midi::InstrMidi,
    instr_opl::{InstrOpl, MdiInstr, MdiOpl},
    instr_robsid::InstrRobSid,
    instr_sid::InstrSid,
    instr_vibrato::InstrVibrato,
    instrument::{Instrument, InstrumentType},
    module::{Module, Pattern, Row, MAX_NUM_ROWS},
    patternslot::PatternSlot,
    period_helper::{FrequencyType, PeriodHelper},
    pitch::Pitch,
    sample::{LoopType, Sample, SampleDataType},
    track_unit::TrackUnit,
    waveform::Waveform,
};
