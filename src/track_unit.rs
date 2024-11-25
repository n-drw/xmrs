use crate::{
    effect::{GlobalEffect, TrackEffect},
    pitch::Pitch,
};
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
    pub global_effects: Vec<GlobalEffect>,
}

impl Default for TrackUnit {
    fn default() -> Self {
        Self {
            note: Pitch::default(),
            velocity: 1.0,
            instrument: None,
            effects: vec![],
            global_effects: vec![],
        }
    }
}
