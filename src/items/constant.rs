use flexstr::{LocalStr, ToLocalStr};
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::SV_SIZE;
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct ConstantIncrementer {
    pub value: LocalStr
}

impl ConstantIncrementer {
    pub fn new(value: String) -> Self {
        Self { value: value.to_local_str() }
    }
}

impl RecipeIncrementer for ConstantIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        false
    }

    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![self.value.clone()]
    }
}