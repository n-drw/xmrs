use crate::prelude::*;

use bincode::error::DecodeError;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use super::it_edit_history::{ItEditHistory, ItEditHistoryEntry};
use super::it_header::ItHeader;
use super::it_instrument::ItInstrument;
use super::it_midi_macros::ItMidiMacros;
use super::it_pattern::ItPattern;
use super::it_plugins::Plugins;
use super::it_sample_header::ItSampleHeader;
use super::it_x_names::ItXNames;

#[derive(Debug)]
#[repr(C)]
pub struct ItModule {
    header: ItHeader,
    orders: Vec<u8>,
    edit_history: Option<Vec<ItEditHistoryEntry>>,
    midi_macros: Option<ItMidiMacros>,
    pattern_names: Vec<String>,
    channel_names: Vec<String>,
    plugins: Plugins,
    message: String,
    instruments: Vec<ItInstrument>,
    samples_header: Vec<ItSampleHeader>,
    patterns: Vec<Vec<Vec<PatternSlot>>>,
    samples: Vec<Option<SampleDataType>>,
}

impl ItModule {
    pub fn load(ser_it_module: &[u8]) -> Result<Self, DecodeError> {
        let data = ser_it_module;

        // === ItHeader =====================================================

        let header = if let Ok(ith) =
            bincode::serde::decode_from_slice::<ItHeader, _>(data, bincode::config::legacy())
        {
            ith
        } else {
            return Err(DecodeError::OtherString(
                "ITHeader Deserialize error.".to_string(),
            ));
        };
        let data = &data[header.1..];

        // === Orders =======================================================

        let orders = &data[..header.0.order_number as usize];
        let data = &data[header.0.order_number as usize..];

        // === Instruments Offsets ==========================================

        let instrument_offsets_u8 = &data[..4 * header.0.instrument_number as usize];
        let instrument_offsets: Vec<u32> = instrument_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let data = &data[4 * header.0.instrument_number as usize..];

        // === Samples Header Offsets =======================================

        let sample_header_offsets_u8 = &data[..4 * header.0.sample_number as usize];
        let sample_header_offsets: Vec<u32> = sample_header_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let data = &data[4 * header.0.sample_number as usize..];

        // === Patterns Offsets ==========================================

        let pattern_offsets_u8 = &data[..4 * header.0.pattern_number as usize];
        let pattern_offsets: Vec<u32> = pattern_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let mut data = &data[4 * header.0.pattern_number as usize..];

        // === Edit History =================================================

        let edit_history = if header.0.is_edit_history_embedded() {
            let (edit_history, l) = ItEditHistory::load(data);
            data = &data[l..];
            edit_history
        } else {
            None
        };

        // === Midi Macros ==================================================

        let midi_macros = if header.0.is_embedded_midi_macro() || header.0.is_embedded_midi_macros()
        {
            let r = bincode::serde::decode_from_slice::<ItMidiMacros, _>(
                data,
                bincode::config::legacy(),
            )
            .unwrap();
            data = &data[r.1..];
            Some(r.0)
        } else {
            None
        };

        // === Pattern names ================================================

        let pattern_names = if ItXNames::is_pnam(data) {
            data = &data[4..];
            let (pattern_names, l) = ItXNames::load(data, 32);
            data = &data[l..];
            pattern_names
        } else {
            vec![]
        };

        // === Channel names ================================================

        let channel_names = if ItXNames::is_cnam(data) {
            data = &data[4..];
            let (channel_names, l) = ItXNames::load(data, 20);
            data = &data[l..];
            channel_names
        } else {
            vec![]
        };

        // === Mix Plugins ==================================================

        let (plugins, l) = Plugins::load(data);
        data = &data[l..];

        // === Message ======================================================

        let message = if header.0.is_song_message_attached() && header.0.message_length != 0 {
            let start = header.0.message_offset as usize;
            let end = start + header.0.message_length as usize;
            let src = &ser_it_module[start..end];
            String::from_utf8_lossy(src).trim().to_string()
        } else {
            String::new()
        };

        // === Instruments ==================================================

        let mut instruments: Vec<ItInstrument> = vec![];
        for i in 0..header.0.instrument_number {
            let i_seek = instrument_offsets[i as usize] as usize;
            let data = &ser_it_module[i_seek..];
            if !header.0.is_post20() {
                instruments.push(ItInstrument::load_post2(data));
            } else {
                instruments.push(ItInstrument::load_pre2(data));
            }
        }

        // === Samples Header ===============================================

        let mut samples_header: Vec<ItSampleHeader> = vec![];
        for i in 0..header.0.sample_number {
            let i_seek = sample_header_offsets[i as usize] as usize;
            let data = &ser_it_module[i_seek..];
            let sample_h = bincode::serde::decode_from_slice::<ItSampleHeader, _>(
                data,
                bincode::config::legacy(),
            )
            .unwrap();
            samples_header.push(sample_h.0);
        }

        // === Patterns =====================================================

        let mut patterns: Vec<Vec<Vec<PatternSlot>>> = vec![];
        for pattern_seek in &pattern_offsets {
            let data = &ser_it_module[*pattern_seek as usize..];
            let itpattern = ItPattern::load(data)?;
            let pattern = itpattern.unpack()?;
            patterns.push(pattern);
        }

        // === Samples ======================================================

        let mut samples = vec![];
        for sh in &samples_header {
            if sh.is_associated_sample() {
                let start = sh.sample_pointer as usize;
                let sample = sh.get_sample_data(&ser_it_module[start..]).unwrap();
                if sample.len() != 0 {
                    samples.push(Some(sample));
                } else {
                    samples.push(None);
                }
            } else {
                samples.push(None);
            }
        }

        // === All in ItModule ==============================================

        let it = Self {
            header: header.0,
            orders: orders.to_vec(),
            edit_history,
            midi_macros,
            pattern_names,
            channel_names,
            plugins,
            message,
            instruments,
            samples_header,
            patterns,
            samples,
        };

        return Ok(it);
    }
}
