use std::fmt::Debug;
use flexstr::LocalStr;
use smallvec::SmallVec;
use crate::candidate_generator::SV_SIZE;

// A RecipeItem is like an iterator but is able to reset
// RecipeItems are called in order of recipe steps and are responsible for looping through permutations of a step
pub trait RecipeIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool;  // new items should start on first entry, so only increment aftewards
    fn reset(&mut self, text_segments: &Vec<LocalStr>) {}                                             // will reset to first entry
    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]>;
}


