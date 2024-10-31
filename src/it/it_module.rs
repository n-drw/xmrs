use crate::prelude::*;

use bincode::error::DecodeError;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::Deserialize;

use super::it_header::ItHeader;
use super::it_edit_history::{ItEditHistory, ItEditHistoryEntry};
use super::it_midi_macros::ItMidiMacros;
use super::it_instrument::ItInstrument;
use super::it_sample_header::ItSampleHeader;
use super::it_pattern::ItPattern;

#[derive(Deserialize, Debug)]
#[repr(C)]
pub struct ItModule {
    header: ItHeader,
    orders: Vec<u8>,
    edit_history: Option<Vec<ItEditHistoryEntry>>,
    midi_macros: Option<ItMidiMacros>,
    // pattern_names: Vec<String>,
    // channel_names: Vec<String>,
    message: String,
    instruments: Vec<ItInstrument>,
    samples_header: Vec<ItSampleHeader>,
    patterns: Vec<ItPattern>,
    samples: Vec<Option<SampleDataType>>,
}

impl ItModule {
    pub fn load(ser_it_module: &[u8]) -> Result<Self, DecodeError> {
        let data = ser_it_module;

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

        let orders = &data[..header.0.order_number as usize];
        let data = &data[header.0.order_number as usize..];

        let instrument_offsets_u8 = &data[..4 * header.0.instrument_number as usize];
        let instrument_offsets: Vec<u32> = instrument_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let data = &data[4 * header.0.instrument_number as usize..];

        let sample_header_offsets_u8 = &data[..4 * header.0.sample_number as usize];
        let sample_header_offsets: Vec<u32> = sample_header_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let data = &data[4 * header.0.sample_number as usize..];

        let pattern_offsets_u8 = &data[..4 * header.0.pattern_number as usize];
        let pattern_offsets: Vec<u32> = pattern_offsets_u8
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .ok()
            .unwrap();
        let data = &data[4 * header.0.pattern_number as usize..];

        let (edit_history, l) = ItEditHistory::load(data);
        let mut data = &data[l..];

        let midi_macros = if header.0.flags & 0b0000_0000_1000_0000 != 0
            || header.0.special_flags & 0b0000_0000_0000_1000 != 0
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

        // FIXME: Wtf?

        // OMPT only
        // let pattern_names = if header.0.is_ompt() {
        //     let (pattern_names, l) = ItXNames::load(data);
        //     data = &data[l..];
        //     pattern_names
        // } else {
        //     vec![]
        // };
        // OMPT only
        // let channel_names = if header.0.is_ompt() {
        //     let (channel_names, l) = ItXNames::load(data);
        //     data = &data[l..];
        //     channel_names
        // } else {
        //     vec![]
        // };

        // FIXME: Plugins here???

        let message = if header.0.message_length != 0 {
            let start = header.0.message_offset as usize;
            let end = start + header.0.message_length as usize;
            let src = &ser_it_module[start..end];
            String::from_utf8_lossy(src).to_string()
        } else {
            String::new()
        };

        let mut instruments: Vec<ItInstrument> = vec![];
        for i in 0..header.0.instrument_number {
            let i_seek = instrument_offsets[i as usize] as usize;
            let mut data = &ser_it_module[i_seek..];
            if !header.0.is_post20() {
                instruments.push(ItInstrument::load_post2(data));
            } else {
                instruments.push(ItInstrument::load_pre2(data));
            }
        }

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

        let mut patterns: Vec<ItPattern> = vec![];
        for pattern_seek in &pattern_offsets {
            let data = &ser_it_module[*pattern_seek as usize..];
            let pattern = ItPattern::load(data);
            patterns.push(pattern);
        }

        let mut samples = vec![];
        for sh in &samples_header {
            if sh.is_associated_sample() {
                let start = sh.sample_pointer as usize;
                let sample = sh.get_sample_data(&ser_it_module[start..]).unwrap();
                samples.push(Some(sample));
            } else {
                samples.push(None);
            }
        }

        let it = Self {
            header: header.0,
            orders: orders.to_vec(),
            edit_history,
            midi_macros,
            // pattern_names,
            // channel_names,
            message,
            instruments,
            samples_header,
            patterns,
            samples,
        };

        return Ok(it);
    }
}
