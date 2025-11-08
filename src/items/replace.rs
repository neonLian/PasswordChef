use flexstr::{LocalStr, ToLocalStr};
use smallvec::{smallvec, SmallVec};
use crate::candidate_generator::{SegIndex, SV_SIZE};
use crate::items::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct ReplaceIncrementer {
    source_seg_idx: SegIndex,
    replacements: Vec<(char, char)>,
    // replacement_locs: Vec<Vec<usize>>,  // replacement_locs[repl_type_idx][repl_loc_idx] = char index to replace
    repl_type_idx: usize,
    repl_loc_idx: Option<usize>,
    repl_loc: Option<usize>
}

impl ReplaceIncrementer {
    pub fn new(source_seg_idx: SegIndex, replacements: Vec<(char, char)>) -> Self {
        let num_repl = replacements.len();
        Self {
            source_seg_idx,
            replacements,
            // replacement_locs: vec![Vec::new(); num_repl],
            repl_type_idx: 0,
            repl_loc_idx: None,
            repl_loc: None
        }
    }
}

impl RecipeIncrementer for ReplaceIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        let source_txt = &text_segments[self.source_seg_idx];
        
        let mut loc_idx = 0;

        // Currently on a valid replacement; go to the next one
        if let Some(rli) = self.repl_loc_idx {
            loc_idx = rli + 1;
        }
        
        loop {
            let repl_locs: Vec<(usize, char)> = source_txt.chars()
                .enumerate()
                .filter(|(i, c)| *c == self.replacements[self.repl_type_idx].0)
                .collect();
            // If no more valid replacements of this type
            if loc_idx >= repl_locs.len() {
                loc_idx = 0;
                self.repl_type_idx += 1;
                
                if self.repl_type_idx >= self.replacements.len() {
                    // No replacements left for this word
                    self.repl_loc_idx = None;
                    self.repl_loc = None;
                    return false;
                }
                // Loop to next replacement
                continue;
            } 
            // Found valid next replacement
            self.repl_loc_idx = Some(loc_idx);
            self.repl_loc = Some(repl_locs[loc_idx].0);
            return true;
        }
        return false;
    }

    fn reset(&mut self, text_segments: &Vec<LocalStr>) {
        self.repl_type_idx = 0;
        self.repl_loc_idx = None;
        self.repl_loc = None;
        // let source_txt = &text_segments[self.source_seg_idx];
        // self.repl_type_idx = 0;
        // loop {
        //     let repl_locs: Vec<(usize, char)> = source_txt.chars()
        //         .enumerate()
        //         .filter(|(i, c)| *c == self.replacements[self.repl_type_idx].0)
        //         .collect();
        //     if repl_locs.is_empty() {
        //         self.repl_type_idx += 1;
        //         if self.repl_type_idx >= self.replacements.len() {
        //             // No replacements for this word
        //             self.repl_loc_idx = None;
        //             self.repl_loc = None;
        //             break;
        //         }
        //         // Loop to next replacement
        //         continue;
        //     }
        //     // Found valid starting replacement
        //     self.repl_loc_idx = Some(0);
        //     self.repl_loc = Some(repl_locs[0].0);
        // }
    }


    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        match self.repl_loc {
            Some(loc) => {
                let mut source_txt = text_segments[self.source_seg_idx].clone().to_string();
                source_txt.replace_range(loc..loc+1, self.replacements[self.repl_type_idx].1.to_string().as_str());
                smallvec![source_txt.to_local_str()]
            },
            None => smallvec![text_segments[self.source_seg_idx].clone()]
        }
    }
}
