use crate::prelude::PatternSlot;
use alloc::{vec, vec::Vec};

pub struct XtTrack;

impl XtTrack {
    /// Here we use `Vec<PatternSlot>` like a track _not_ like a Pattern row!
    pub fn save(track: &Vec<PatternSlot>) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let version: u16 = 1;
        let nrow: u16 = track.len() as u16;
        data.append(&mut bincode::serde::encode_to_vec(&version, bincode::config::legacy()).unwrap());
        data.append(&mut bincode::serde::encode_to_vec(&nrow, bincode::config::legacy()).unwrap());
        for xmps in track {
            let mut d = xmps.save_unpack();
            data.append(&mut d);
        }
        data
    }
}
