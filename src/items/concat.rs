use std::ops::Range;
use flexstr::{LocalStr, ToLocalStr};
use itertools::{Group, Itertools, Permutations};
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::{SegIndex, SV_SIZE};
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct ConcatIncrementer {
    source_id_indices: Vec<SegIndex>
}

impl ConcatIncrementer {
    pub fn new(source_id_indices: Vec<SegIndex>) -> Self {
        let n: usize = source_id_indices.len();
        Self { source_id_indices }
    }
}

impl RecipeIncrementer for ConcatIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        false
    }
    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![
            self.source_id_indices.iter()
                .map(|src_id| text_segments[*src_id].clone())
                .join("")
                .to_local_str()
        ]
    }
}