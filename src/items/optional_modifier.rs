use std::cell::RefCell;
use std::rc::Rc;
use flexstr::{local_str, LocalStr};
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::SV_SIZE;
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
// Iteration order: include=true, include=false
pub struct OptionalModifierIncrementer {
    source_seg_idx: usize,
    current_include: bool
}

impl OptionalModifierIncrementer {
    pub fn new(source_seg_idx: usize) -> OptionalModifierIncrementer {
        OptionalModifierIncrementer { source_seg_idx, current_include: true }
    }
}

impl RecipeIncrementer for OptionalModifierIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        if (self.current_include) {
            self.current_include = false;
            true
        } else {
            false
        }
    }

    fn reset(&mut self, text_segments: &Vec<LocalStr>) {
        self.current_include = true;
    }

    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![
            if (self.current_include) {
                text_segments[self.source_seg_idx].clone()
            } else {
                local_str!("")
            }
        ]
    }
}
