use crate::{
    effect::{ControlChangeEffect, TrackEffect},
    pitch::Pitch,
    prelude::Instrument,
};
use alloc::format;
use alloc::string::ToString;
use core::fmt::*;
use serde::{Deserialize, Serialize};

use alloc::vec;
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackUnit {
    pub note: Pitch,
    pub velocity: f32,
    pub instrument: Option<usize>,
    pub effects: Vec<TrackEffect>,
    pub cc_effects: Vec<ControlChangeEffect>,
}

impl Default for TrackUnit {
    fn default() -> Self {
        Self {
            note: Pitch::default(),
            velocity: 1.0,
            instrument: None,
            effects: vec![],
            cc_effects: vec![],
        }
    }
}
