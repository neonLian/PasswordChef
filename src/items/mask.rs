use smallvec::{smallvec, SmallVec};
use constcat::concat;
use flexstr::{LocalStr, ToLocalStr};
use crate::candidate_generator::SV_SIZE;
use crate::items::incrementer_trait::RecipeIncrementer;
use MaskCharType::*;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIAL_ALL: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ";
const SPECIAL_SUBSET: &str = "!@#$%^&*+-=_";
const HEXLETTERS_LOWER: &str = "abcdef";
const HEXLETTERS_UPPER: &str = "ABCDEF";

#[derive(Debug)]
pub enum MaskCharType {
    Lowercase,
    Uppercase,
    Letters,
    Digits,
    Alphanumeric,
    Special,
    SpecialSubset,
    Everything,
    EverythingAlmost,
    HexLower,
    HexUpper,
    Custom1
}

impl MaskCharType {
    fn mask_type_from_letter(char: &u8) -> Option<MaskCharType> {
        match char {
            b'l' => Some(Lowercase),
            b'u' => Some(Uppercase),
            b'L' => Some(Letters),
            b'd' => Some(Digits),
            b'A' => Some(Alphanumeric),
            b's' => Some(Special),
            b'S' => Some(SpecialSubset),
            b'e' => Some(Everything),
            b'E' => Some(EverythingAlmost),
            b'h' => Some(HexLower),
            b'H' => Some(HexUpper),
            b'1' => Some(Custom1),
            _ => None
        }
    }
    fn charset(mask_char_type: &MaskCharType) -> &'static str {
        match mask_char_type {
            Lowercase => LOWERCASE,
            Uppercase => UPPERCASE,
            Letters => concat!(LOWERCASE, UPPERCASE),
            Digits => DIGITS,
            Alphanumeric => concat!(LOWERCASE, UPPERCASE, DIGITS),
            Special => SPECIAL_ALL,
            SpecialSubset => SPECIAL_SUBSET,
            Everything => concat!(LOWERCASE, UPPERCASE, DIGITS, SPECIAL_ALL),
            EverythingAlmost => concat!(LOWERCASE, UPPERCASE, DIGITS, SPECIAL_SUBSET),
            HexLower => concat!(DIGITS, HEXLETTERS_LOWER),
            HexUpper => concat!(DIGITS, HEXLETTERS_UPPER),
            Custom1 => ""
        }
    }
}

#[derive(Debug)]
pub struct MaskIncrementer {
    char_type: Vec<MaskCharType>,
    char_idx: Vec<usize>,
    char_max: Vec<usize>  // Maximum index for each character in the mask (exclusive)
}

impl MaskIncrementer {
    pub fn new(mask: String) -> Self {
        let char_type: Vec<MaskCharType> = mask.as_bytes().iter()
            .map(|c| MaskCharType::mask_type_from_letter(c)
                                   .unwrap_or_else(|| panic!("Invalid mask character: {}", *c as char)))
            .collect();
        let char_max: Vec<usize> = char_type.iter().map(|t| MaskCharType::charset(t).len()).collect();
        Self { char_type, char_idx: vec![0; char_max.len()], char_max }
    }
}

impl RecipeIncrementer for MaskIncrementer {
    fn increment(&mut self, text_segments: &Vec<LocalStr>) -> bool {
        for i in (0..self.char_idx.len()).rev() {
            self.char_idx[i] += 1;

            if self.char_idx[i] < self.char_max[i] { return true; }
            else { self.char_idx[i] = 0; }
        }
        false
    }

    fn output(&self, text_segments: &Vec<LocalStr>) -> SmallVec<[LocalStr; SV_SIZE]> {
        smallvec![String::from_utf8(
            self.char_idx
                .iter()
                .enumerate()
                .map(|(i, ci)| MaskCharType::charset(&self.char_type[i]).as_bytes()[*ci])
                .collect()
        ).unwrap().to_local_str()]
    }
}
