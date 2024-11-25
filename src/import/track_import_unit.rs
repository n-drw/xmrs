use super::track_import_effect::TrackImportEffect;
use crate::prelude::TrackUnit;
use crate::{effect::GlobalEffect, pitch::Pitch};
use core::fmt::*;
use serde::{Deserialize, Serialize};

use alloc::vec;
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackImportUnit {
    pub note: Pitch,
    pub velocity: f32,
    pub instrument: Option<usize>,
    pub effects: Vec<TrackImportEffect>,
    pub global_effects: Vec<GlobalEffect>,
}

impl Default for TrackImportUnit {
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

impl TrackImportUnit {
    pub fn prepare_track_unit(&self) -> TrackUnit {
        TrackUnit {
            note: self.note,
            velocity: self.velocity,
            instrument: self.instrument,
            effects: vec![],
            global_effects: self.global_effects.clone(),
        }
    }
}
