use std::io::{BufRead, BufReader, Seek};
use std::fs::File;
use flexstr::{local_str, LocalStr, ToLocalStr};
use smallvec::{smallvec, SmallVec};
use trim_in_place::TrimInPlace;
use crate::candidate_generator::SV_SIZE;
use super::incrementer_trait::RecipeIncrementer;

#[derive(Debug)]
pub struct WordlistIncrementer {
    reader: BufReader<File>,
    current_value: LocalStr,
    current_val_string: String
}

impl WordlistIncrementer {
    pub fn new(filename: String) -> WordlistIncrementer {
        let mut item = WordlistIncrementer {
            reader: BufReader::new(File::open(&filename).unwrap_or_else(|_| panic!("Error reading file {}", &filename))),
            current_value: local_str!(""),
            current_val_string: String::new()
        };
        item.increment();
        item
    }
}

impl RecipeIncrementer for WordlistIncrementer {
    fn increment(&mut self) -> bool {
        self.current_val_string.clear();
        let result = self.reader.read_line(&mut self.current_val_string);
        self.current_val_string.trim_in_place();
        self.current_value = self.current_val_string.to_local_str();
        result.is_ok_and(|n| n > 0)
    }

    fn reset(&mut self) {
        self.reader.rewind().unwrap();
        self.increment();
    }
    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![self.current_value.clone()]
    }
}