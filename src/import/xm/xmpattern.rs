/// Original XM Pattern
use bincode::error::{DecodeError, EncodeError};
use serde::{Deserialize, Serialize};

use alloc::{vec, vec::Vec};

use crate::module::Module;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmPatternHeader {
    pattern_header_len: u32,
    packing_type: u8,
    num_rows: u16,
    pattern_data_size: u16,
}

impl Default for XmPatternHeader {
    fn default() -> Self {
        XmPatternHeader {
            pattern_header_len: 9,
            packing_type: 0,
            num_rows: 0,
            pattern_data_size: 0,
        }
    }
}

impl XmPatternHeader {
    pub fn new(size: usize) -> Self {
        let mut ph = XmPatternHeader::default();
        ph.num_rows = size as u16;
        ph
    }

    pub fn load(data: &[u8]) -> Result<(&[u8], XmPatternHeader), DecodeError> {
        match bincode::serde::decode_from_slice::<XmPatternHeader, _>(
            data,
            bincode::config::legacy(),
        ) {
            Ok((xmph, _)) => {
                let hl = xmph.pattern_header_len as usize;
                Ok((&data[hl..], xmph))
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct XmPattern {
    pub header: XmPatternHeader,
    pub pattern: Pattern,
}

impl XmPattern {
    pub fn new(rows: usize, noc: usize) -> Self {
        Self {
            header: XmPatternHeader::new(rows),
            pattern: vec![vec![PatternSlot::default(); noc]; rows],
        }
    }

    pub fn load(data: &[u8], number_of_channels: u16) -> Result<(&[u8], XmPattern), DecodeError> {
        let (data, xmph) = XmPatternHeader::load(data)?;
        let (_data_out, xmps) = Self::get_slots(
            &data[0..xmph.pattern_data_size as usize],
            number_of_channels as usize,
            xmph.num_rows as usize,
        )
        .unwrap();
        let seek = xmph.pattern_data_size as usize;

        let xmp = Self {
            header: xmph,
            pattern: xmps,
        };

        Ok((&data[seek..], xmp))
    }

    fn get_empty_line(number_of_channels: usize) -> Vec<PatternSlot> {
        let mut row: Vec<PatternSlot> = vec![];
        let xmps = PatternSlot::default();
        for _ in 0..number_of_channels {
            row.push(xmps.clone());
        }
        row
    }

    fn get_slots(
        data: &[u8],
        number_of_channels: usize,
        number_of_rows: usize,
    ) -> Result<(&[u8], Vec<Vec<PatternSlot>>), DecodeError> {
        let mut lines: Vec<Vec<PatternSlot>> = vec![];
        let mut row: Vec<PatternSlot> = vec![];

        let mut d2 = data;
        loop {
            if d2.is_empty() {
                break;
            }
            let (d3, xps) = PatternSlot::load_xm(d2)?;
            d2 = d3;
            row.push(xps);
            if row.len() == number_of_channels {
                lines.push(row);
                row = vec![];
            }
        }

        while lines.len() < number_of_rows {
            lines.push(Self::get_empty_line(number_of_channels));
        }

        Ok((d2, lines))
    }

    /// All patterns
    pub fn from_module(module: &Module) -> Vec<Self> {
        let mut all: Vec<Self> = vec![];
        for p in &module.pattern {
            let mut xmp = XmPattern {
                pattern: p.clone(),
                ..Default::default()
            };
            xmp.header.num_rows = p.len() as u16;
            // uncompressed patternslot
            xmp.header.pattern_data_size = (p.len() * p[0].len() * 5) as u16;
            all.push(xmp);
        }
        all
    }

    pub fn save(&mut self) -> Result<Vec<u8>, EncodeError> {
        let mut p_output: Vec<u8> = vec![];

        for p in &self.pattern {
            for ps in p {
                let mut b = ps.save_xm();
                p_output.append(&mut b);
            }
        }
        self.header.pattern_data_size = p_output.len() as u16;

        let mut output = bincode::serde::encode_to_vec(&self.header, bincode::config::legacy())?;
        output.append(&mut p_output);
        Ok(output)
    }
}
