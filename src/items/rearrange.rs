use std::ops::Range;
use flexstr::LocalStr;
use itertools::{Itertools, Permutations};
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::{SegIndex, SV_SIZE};
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct RearrangeIncrementer {
    source_id_indices: Vec<SegIndex>,
    permute_iter: Permutations<Range<usize>>,
    current_permutation: Vec<usize>
}

impl RearrangeIncrementer {
    pub fn new(source_id_indices: Vec<SegIndex>) -> Self {
        let n: usize = source_id_indices.len();
        let mut permute_iter = (0..n).permutations(n);
        let current_permutation = permute_iter.next().unwrap();
        Self { source_id_indices, permute_iter, current_permutation }
    }
}

impl RecipeIncrementer for RearrangeIncrementer {
    fn increment(&mut self) -> bool {
        if let Some(p) = self.permute_iter.next() {
            self.current_permutation = p;
            true
        }
        else { false }
    }

    fn reset(&mut self) {
        let n = self.source_id_indices.len();
        self.permute_iter = (0..n).permutations(n);
        self.current_permutation = self.permute_iter.next().unwrap();
    }

    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        self.current_permutation.iter()
            .map(|pi| text_segments[self.source_id_indices[*pi]].clone())
            .collect::<SmallVec<[LocalStr; SV_SIZE]>>()
    }
}