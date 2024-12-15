/// Original XM Module
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

use alloc::format;
use alloc::{vec, vec::Vec};

use super::xmheader::{XmFlagType, XmHeader};
use super::xminstrument::XmInstrument;
use super::xmpattern::XmPattern;

use crate::import::import_memory::{ImportMemory, MemoryType};
use crate::import::orders_helper;
use crate::import::patternslot::PatternSlot;
use crate::module::Module;
use crate::period_helper::FrequencyType;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct XmModule {
    pub header: XmHeader,
    pub pattern_order: Vec<u8>,
    pub pattern: Vec<XmPattern>,
    pub instrument: Vec<XmInstrument>,
}

impl XmModule {
    pub fn load(data: &[u8]) -> Result<Self, DecodeError> {
        let (data, header, pattern_order) = XmHeader::load(data)?;
        let mut data = data;

        // Create patterns from xm
        let mut pattern: Vec<XmPattern> = vec![];
        for _i in 0..header.number_of_patterns {
            let (d2, xmp) = XmPattern::load(data, header.number_of_channels)?;
            data = d2;
            pattern.push(xmp);
        }

        // Add empty patterns
        if pattern_order.len() > pattern.len() {
            let empty_ones = pattern_order.len() - pattern.len();
            let empty = XmPattern::new(64, header.number_of_channels.into());
            pattern.extend(core::iter::repeat(empty).take(empty_ones));
        }

        let mut instrument: Vec<XmInstrument> = vec![];
        for _i in 0..header.number_of_instruments {
            // Create instruments form xm
            let (d2, xmi) = XmInstrument::load(data)?;
            data = d2;
            instrument.push(xmi);
        }

        Ok(XmModule {
            header,
            pattern_order,
            pattern,
            instrument,
        })
    }

    pub fn to_module(&self) -> Module {
        // Create module from xm
        let mut module = Module {
            name: self.header.name.clone(),
            comment: format!(
                "{} ({}.{:02})",
                self.header.tracker_name,
                self.header.version_number >> 8,
                self.header.version_number & 0xFF
            ),
            frequency_type: match self.header.flags {
                XmFlagType::XmAmigaFrequencies => FrequencyType::AmigaFrequencies,
                XmFlagType::XmLinearFrequencies => FrequencyType::LinearFrequencies,
            },
            restart_position: self.header.restart_position as usize,
            default_tempo: self.header.default_tempo as usize,
            default_bpm: self.header.default_bpm as usize,
            pattern_order: orders_helper::parse_orders(&self.pattern_order),
            pattern: vec![],
            pattern_names: vec![],
            channel_names: vec![],
            instrument: vec![],
        };

        let patterns: Vec<Vec<Vec<PatternSlot>>> =
            self.pattern.iter().map(|p| p.pattern.clone()).collect();
        let mut im = ImportMemory::default();
        module.pattern = im.unpack_patterns(
            module.frequency_type,
            MemoryType::Xm,
            &module.pattern_order,
            &patterns,
        );

        for i in &self.instrument {
            module.instrument.push(i.to_instrument())
        }

        module
    }
}
