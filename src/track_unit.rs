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
    // TODO?: Option<Pitch>
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

impl TrackUnit {
    pub fn has_arpeggio(&self) -> bool {
        self.effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::Arpeggio { half1: _, half2: _ }))
    }

    pub fn has_delay(&self) -> bool {
        self.effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::NoteDelay(_)))
    }

    pub fn get_delay(&self) -> usize {
        self.effects
            .iter()
            .find_map(|effect| {
                if let TrackEffect::NoteDelay(delay) = effect {
                    Some(*delay)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    pub fn has_note_off(&self) -> bool {
        let fx = self
            .effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::NoteOff { tick: _, past: _ }));
        fx || self.note.is_keyoff()
    }

    pub fn has_tone_portamento(&self) -> bool {
        self.effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::TonePortamento(_)))
    }

    pub fn has_vibrato(&self) -> bool {
        self.effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::Vibrato { speed: _, depth: _ }))
    }

    pub fn has_volume_slide(&self) -> bool {
        self.effects
            .iter()
            .any(|effect| matches!(effect, TrackEffect::VolumeSlide { speed: _, fine: _ }))
    }
}
