use flexstr::LocalStr;
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::{SegIndex, SV_SIZE};
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct DuplicateIncrementer {
    source_seg_idx: SegIndex
}

impl DuplicateIncrementer {
    pub fn new(source_seg_idx: SegIndex) -> Self {
        Self { source_seg_idx }
    }
}

impl RecipeIncrementer for DuplicateIncrementer {
    fn increment(&mut self) -> bool {
        false
    }

    fn reset(&mut self) {}


    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![text_segments[self.source_seg_idx].clone()]
    }
}
