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
/// A typical Note
pub mod note;
/// A typical pattern slot
pub mod patternslot;
/// Period Helper
pub mod period_helper;
pub(crate) mod period_helper_cache;
/// Sample with Steroid
pub mod sample;

/// Load and Save Historical XM files
#[cfg(feature = "import_xm")]
pub mod xm;

/// Load only Historical MOD files
#[cfg(feature = "import_amiga")]
pub mod amiga;

/// Load only Historical S3M files
#[cfg(feature = "import_s3m")]
pub mod s3m;

/// Load only Historical SID files
#[cfg(feature = "import_sid")]
pub mod sid;

/// The Xmrs Prelude
pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
