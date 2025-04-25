use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{BufWriter, Write};
use std::mem;
use std::rc::Rc;
use flexstr::{local_str, LocalStr};
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use RecipeStep::*;
use crate::items::case_modifier::CaseModifierIncrementer;
use crate::recipe_step::*;
use crate::items::incrementer_trait::RecipeIncrementer;
use crate::items::wordlist::WordlistIncrementer;
use crate::items::constant::ConstantIncrementer;
use crate::items::duplicate::DuplicateIncrementer;
use crate::items::mask::MaskIncrementer;
use crate::items::optional_modifier::OptionalModifierIncrementer;
use crate::items::rearrange::RearrangeIncrementer;

pub const SV_SIZE: usize = 4;

pub type IncIndex = usize;
pub type SegIndex = usize;

pub struct CandidateGenerator {
    incrementers: Vec<Box<dyn RecipeIncrementer>>,
    write_indices: Vec<SmallVec<[SegIndex; SV_SIZE]>>,     // Which text segments an incrementer will write to
    text_segments: Vec<LocalStr>,
    output_indices: Vec<SegIndex>,                         // Which text segments will be outputted as the password and in what order
    buffer: String,
    writer: Box<BufWriter<dyn Write>>,
}

struct CandidateGeneratorFields {
    incrementers: Vec<Box<dyn RecipeIncrementer>>,
    write_indices: Vec<SmallVec<[SegIndex; SV_SIZE]>>,
    output_indices: Vec<SegIndex>,
    // #ID or .class -> text segment index
    id_map: HashMap<String, SegIndex>,
    class_map: HashMap<String, Vec<SegIndex>>,
    cur_seg_idx: SegIndex
}

impl CandidateGenerator {
    pub fn from_recipe(recipe: Recipe, writer: Box<BufWriter<dyn Write>>) -> CandidateGenerator {

        let mut fields = CandidateGeneratorFields {
            incrementers: Vec::new(),
            write_indices: Vec::new(),
            output_indices: Vec::new(),
            cur_seg_idx: 0,
            id_map: HashMap::new(),
            class_map: HashMap::new()
        };

        for (i, step) in recipe.into_iter().enumerate() {
            let step_id_idx = i + 1;
            let id_to_seg_idx = |id| *fields.id_map.get(id).expect("ERROR: ID doesn't exist");
            match step {
                Wordlist { filename, attr, modifiers } => {
                    Self::add_inc(WordlistIncrementer::new(filename), attr, modifiers, step_id_idx, &mut fields);
                }
                Constant { value, attr, modifiers } => {
                    Self::add_inc(ConstantIncrementer::new(value), attr, modifiers, step_id_idx, &mut fields);
                }
                Duplicate { target_id, attr, modifiers } => {
                    Self::add_inc(DuplicateIncrementer::new(id_to_seg_idx(&target_id)),
                                  attr, modifiers, step_id_idx, &mut fields);
                }
                Mask { mask, attr, modifiers } => {
                    Self::add_inc(MaskIncrementer::new(mask), attr, modifiers, step_id_idx, &mut fields);
                }
                Rearrange { target_list } => {
                    let source_seg_indices: Vec<SegIndex> = target_list.iter()
                        .flat_map(|tag| Self::tag_to_seg_indices(tag, &fields))
                        .collect();
                    Self::add_multimod_incrementer(
                        RearrangeIncrementer::new(source_seg_indices.clone()), 
                        &source_seg_indices, 
                        &mut fields
                    )
                }
                _ => {}
            }
        }

        CandidateGenerator {
            incrementers: fields.incrementers,
            write_indices: fields.write_indices,
            text_segments: vec![local_str!(""); fields.cur_seg_idx],
            output_indices: fields.output_indices,
            buffer: String::new(),
            writer,
        }
    }

    // Shorthand for adding an incrementer, adding modifiers, and updating IDs and classes maps
    fn add_inc<T: RecipeIncrementer + 'static>(
        inc: T, attr: CommonAttributes, modifiers: GeneratorModifiers,
        step_id_idx: usize, fields: &mut CandidateGeneratorFields
    ) {
        Self::add_basic_incrementer(inc, fields);
        Self::add_modifiers(fields.cur_seg_idx - 1, modifiers, fields);
        Self::add_attr(fields.cur_seg_idx - 1, step_id_idx, attr, &mut fields.id_map, &mut fields.class_map);
    }

    fn add_attr(seg_idx: SegIndex, step_id_idx: usize, attr: CommonAttributes,
                id_map: &mut HashMap<String, SegIndex>, class_map: &mut HashMap<String, Vec<SegIndex>>) {
        id_map.insert(format!("#{}", step_id_idx), seg_idx);
        // Assumes that the recipe parser kept # and . in the ID/class names
        if let Some(id) = attr.id {
            let prev_seg = id_map.insert(id, seg_idx);
            if prev_seg.is_some() { panic!("ERROR: duplicate ID in recipe"); }
        }
        for class in attr.classes {
            class_map.entry(class)
                .and_modify(|v| v.push(seg_idx))
                .or_insert(vec![seg_idx]);
        }
    }

    fn add_basic_incrementer<T: RecipeIncrementer + 'static>(
        inc: T,
        mut fields: &mut CandidateGeneratorFields
    ) {
        fields.incrementers.push(Box::new(inc));
        fields.write_indices.push(smallvec![fields.cur_seg_idx]);
        fields.output_indices.push(fields.cur_seg_idx);
        fields.cur_seg_idx += 1;
    }

    fn add_multi_incrementer<T: RecipeIncrementer + 'static>(
        inc: T,
        num_outputs: usize,
        mut fields: &mut CandidateGeneratorFields
    ) {
        fields.incrementers.push(Box::new(inc));
        let range = (fields.cur_seg_idx .. (fields.cur_seg_idx + num_outputs)).collect_vec();
        fields.write_indices.push(range.iter().copied().collect::<SmallVec<[SegIndex; SV_SIZE]>>());
        for seg_idx in range {
            fields.output_indices.push(fields.cur_seg_idx);
            fields.cur_seg_idx += 1;
        }
    }

    fn add_multimod_incrementer<T: RecipeIncrementer + 'static>(
        inc: T,
        source_seg_indices: &Vec<SegIndex>,
        mut fields: &mut CandidateGeneratorFields
    ) {
        fields.incrementers.push(Box::new(inc));
        let n = source_seg_indices.len();
        let out_seg_range: Vec<SegIndex> = (fields.cur_seg_idx .. (fields.cur_seg_idx + n)).collect_vec();
        fields.write_indices.push(out_seg_range.iter().copied().collect::<SmallVec<[SegIndex; SV_SIZE]>>());
        // println!("SOURCE SEGS: {:?}", source_seg_indices);
        // println!("OUT SEG RANGE: {:?}", out_seg_range);
        // println!("OUT INDICES: {:?}", fields.output_indices);
        // Update output indices
        for i in 0..n {
            // Replace source index with our new index
            if let Some(oi) = fields.output_indices.iter().position(|oi| *oi == source_seg_indices[i]) {
                fields.output_indices[oi] = out_seg_range[i];
            }
            fields.cur_seg_idx += 1;
        }
        // Update IDs and classes
        Self::replace_tags_for_segs(source_seg_indices, &out_seg_range, fields);
    }

    fn add_modifiers(
        mut source_seg_idx: SegIndex,
        modifiers: GeneratorModifiers,
        mut fields: &mut CandidateGeneratorFields,
    ) {
        // Case modifiers
        if modifiers.case != CaseModifiers::default() {
            Self::add_basic_incrementer(CaseModifierIncrementer::new(source_seg_idx, modifiers.case), fields);
            let new_seg_idx = fields.cur_seg_idx - 1;
            // Make sure source text is no longer included in final output
            Self::remove_seg_from_output(source_seg_idx, fields);
            // Update IDs and classes
            Self::replace_tags_for_segs(&vec![source_seg_idx], &vec![new_seg_idx], fields);

            source_seg_idx = new_seg_idx;
        }

        // Optional modifier
        if modifiers.optional {
            Self::add_basic_incrementer(OptionalModifierIncrementer::new(source_seg_idx), fields);
            let new_seg_idx = fields.cur_seg_idx - 1;
            // Make sure source text is no longer included in final output
            Self::remove_seg_from_output(source_seg_idx, fields);
            // Update IDs and classes
            Self::replace_tags_for_segs(&vec![source_seg_idx], &vec![new_seg_idx], fields);
        }
    }

    fn remove_seg_from_output(mut source_seg_idx: SegIndex, fields: &mut CandidateGeneratorFields) {
        if let Some(oi) = fields.output_indices.iter().position(|i| *i == source_seg_idx) {
            fields.output_indices.remove(oi);
        }
    }
    
    fn tag_to_seg_indices(tag: &String, fields: &CandidateGeneratorFields) -> Vec<SegIndex> {
        if tag.as_bytes()[0] == b'#' {
            vec![*fields.id_map.get(tag).expect("ERROR: ID doesn't exist")]
        } else if tag.as_bytes()[0] == b'.' {
            fields.class_map.get(tag).expect("ERROR: Class doesn't exist").clone()
        } else {
            Vec::<SegIndex>::new()
        }
    }

    fn replace_tags_for_segs(source_seg_indices: &Vec<SegIndex>, new_seg_indices: &Vec<SegIndex>, mut fields: &mut CandidateGeneratorFields) {
        assert_eq!(source_seg_indices.len(), new_seg_indices.len());
        for i in 0..source_seg_indices.len() {
            // Find ID that points to source seg and then replace with new seg
            if let Some((id, _)) = fields.id_map.iter().find(|(k, v)| **v == source_seg_indices[i]) {
                fields.id_map.entry(id.clone()).insert_entry(new_seg_indices[i]);
            }
            for (_, class_segs) in fields.class_map.iter_mut() {
                if let Some(pos) = class_segs.iter().position(|s| *s == source_seg_indices[i]) {
                    class_segs.push(new_seg_indices[i]);
                    class_segs.swap_remove(pos);
                }
            }
        }
    }

    pub fn print_next(&mut self) -> bool {
        self.update_buffer();
        writeln!(self.writer, "{}", self.buffer);
        self.increment()
    }

    fn increment(&mut self) -> bool {
        for inc in self.incrementers.iter_mut().rev() {
            let inc_success = inc.increment();

            if inc_success { return true; }
            else { inc.reset(); }
        }
        false
    }

    fn update_buffer(&mut self) {
        self.buffer.clear();
        // Update text_segments
        for inc_idx in 0..self.incrementers.len() {
            let inc = &self.incrementers[inc_idx];
            let outputs = inc.output(&self.text_segments);
            for i in 0usize..self.write_indices[inc_idx].len() {
                self.text_segments[self.write_indices[inc_idx][i]] = outputs[i].clone();
            }
        }
        // Update output buffer
        for outseg_idx in self.output_indices.iter() {
            self.buffer.push_str(self.text_segments[*outseg_idx].as_str());
        }
    }
}
