use strum_macros::EnumIter;
use crate::recipe_step::RecipeStep::*;

pub type Recipe = Vec<RecipeStep>;

type StepID = String;

#[derive(Debug)]
pub enum RecipeStep {
    // Generators
    Wordlist { filename: String, attr: CommonAttributes, modifiers: GeneratorModifiers },
    Mask { mask: String, attr: CommonAttributes, modifiers: GeneratorModifiers  },
    MaskIncremental { mask: String, attr: CommonAttributes, modifiers: GeneratorModifiers  },
    Constant { value: String, attr: CommonAttributes, modifiers: GeneratorModifiers  },
    Duplicate { target_id: StepID, attr: CommonAttributes, modifiers: GeneratorModifiers  },

    // Index
    Location { attr: CommonAttributes },

    // Operation
    Rearrange { target_list: Vec<StepID> }
}

#[derive(Default, Debug)]
pub struct CommonAttributes {
    pub id: Option<String>,
    pub classes: Vec<String>
}

#[derive(Debug, PartialEq)]
pub struct CaseModifiers {
    pub titlecase: bool,
    pub uppercase: bool,
    pub lowercase: bool,
    pub originalcase: bool
}

impl Default for CaseModifiers {
    fn default() -> Self {
        CaseModifiers {
            titlecase: false,
            uppercase: false,
            lowercase: false,
            originalcase: true,
        }
    }
}

impl CaseModifiers {
    pub fn includes_case(&self, case: CaseModifierType) -> bool {
        match case {
            CaseModifierType::OriginalCase => self.originalcase,
            CaseModifierType::Lowercase => self.lowercase,
            CaseModifierType::Uppercase => self.uppercase,
            CaseModifierType::TitleCase => self.titlecase
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GeneratorModifiers {
    pub case: CaseModifiers,
    pub optional: bool
}

impl Default for GeneratorModifiers {
    fn default() -> Self {
        GeneratorModifiers {
            case: CaseModifiers::default(),
            optional: false,
        }
    }
}

#[derive(EnumIter, PartialEq)]
pub enum CaseModifierType {
    OriginalCase,
    Lowercase,
    Uppercase,
    TitleCase
}