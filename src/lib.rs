#![forbid(unsafe_code)]

//!
//! XMrs is a Safe SoundTracker Library
//!
//! ```
//! module+--->instrument+--->instr_defaut+--->sample
//!       |              |                +--->envelope
//!       |              |                +--->vibrato
//!       |              +--->instr_ekn
//!       |              +--->instr_midi
//!       |              +--->instr_opl
//!       |              +--->instr_sid
//!       |              +-+->instr_robrs
//!       |                +--->instr_sid
//!       +--->Pattern--->Row--->patternslot
//! ```
//!
//! You can load (and save) historical XM files using `xm` (see `README.md`)
//!
//! You can load (and save) your work using `load()` and `save()` serde fn
//!

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

/// All effects
pub mod effect;
/// Envelope with Steroid
pub mod envelope;
/// Historical XM Instrument
pub mod instr_default;
/// Euclidian Rythm Instrument
pub mod instr_ekn;
/// Midi Instrument
pub mod instr_midi;
/// Yamaha OPL Instrument
pub mod instr_opl;
/// Rob Hubbard Instrument
pub mod instr_robsid;
/// MOS6581 SID Instrument
pub mod instr_sid;
/// Vibrato with Steroid
pub mod instr_vibrato;
/// Instrument with Steroid
pub mod instrument;
/// SoundTracker Module with Steroid
pub mod module;
/// Period Helper
pub mod period_helper;
pub(crate) mod period_helper_cache;
/// A typical Note
pub mod pitch;
/// Sample with Steroid
pub mod sample;
/// A slot
pub mod track_unit;
/// All Waveform type
pub mod waveform;

/// The Xmrs Prelude
pub mod prelude;

// #[cfg(any(
//     feature = "import",
//     feature = "import_amiga",
//     feature = "import_it",
//     feature = "import_s3m",
//     feature = "import_sid",
//     feature = "import_xm",
// ))]
pub mod import;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
