use std::cell::RefCell;
use std::rc::Rc;
use flexstr::{LocalStr, ToLocalStr};
use smallvec::{smallvec, SmallVec};
use strum::IntoEnumIterator;
use unicode_titlecase::StrTitleCase;
use crate::candidate_generator::SV_SIZE;
use crate::items::incrementer_trait::RecipeIncrementer;
use crate::recipe_step::CaseModifierType::{Lowercase, OriginalCase};
use crate::recipe_step::{CaseModifierType, CaseModifiers, GeneratorModifiers};

use crate::recipe_step::CaseModifierType::*;

#[derive(Debug)]
pub struct CaseModifierIncrementer {
    source_seg_idx: usize,
    modifiers: CaseModifiers,
    start_case: usize,
    cur_case: usize
}

impl CaseModifierIncrementer {
    pub fn new(source_seg_idx: usize, modifiers: CaseModifiers) -> CaseModifierIncrementer {
        let starting_case =
            if modifiers.originalcase { OriginalCase }
            else if modifiers.lowercase { Lowercase }
            else if modifiers.uppercase { Uppercase }
            else if modifiers.titlecase { TitleCase }
            else { OriginalCase };

        let start_case_num = CaseModifierType::iter().position(|c| c == starting_case).unwrap();

        CaseModifierIncrementer {
            source_seg_idx, modifiers, start_case: start_case_num, cur_case: start_case_num
        }
    }
}

impl RecipeIncrementer for CaseModifierIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        self.cur_case += 1;
        let max_case = CaseModifierType::iter().len()-1;
        while (self.cur_case <= max_case && !self.modifiers.includes_case(CaseModifierType::iter().nth(self.cur_case).unwrap())) {
            self.cur_case += 1;
        }
        self.cur_case <= max_case
    }

    fn reset(&mut self, text_segments: &Vec<LocalStr>) {
        self.cur_case = self.start_case;
    }

    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        let case = CaseModifierType::iter().nth(self.cur_case).unwrap();
        let text = text_segments[self.source_seg_idx].clone();
        smallvec![
            match case {
                OriginalCase => text,
                Lowercase => text.to_ascii_lowercase().to_local_str(),
                Uppercase => text.to_ascii_uppercase().to_local_str(),
                TitleCase => text.to_titlecase_lower_rest().to_local_str()
            }
        ]
    }
}