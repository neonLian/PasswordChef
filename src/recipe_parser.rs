use crate::recipe_step::{CommonAttributes, GeneratorModifiers, Recipe};
use crate::recipe_step::RecipeStep;

use std::default::Default;
use logos::Logos;

pub struct RecipeParser {}

impl RecipeParser {
    pub fn parse(recipe_text: String) -> Recipe {
        // Split by newlines and commas
        let recipe_steps_text = recipe_text
            .lines()
            .flat_map(|line| line.split(',').map(str::trim))
            .filter(|s| !s.is_empty());

        // println!("{recipe_steps_text:?}");

        let recipe_steps = recipe_steps_text
            .map(Self::parse_step)
            .map(Result::unwrap)
            .collect();

        // println!("\n=== RECIPE ===");
        // for step in &recipe_steps {
        //     println!("{:?}", step)
        // }

        recipe_steps
    }

    fn parse_step(step_text: &str) -> Result<RecipeStep, RecipeParseError> {
        let (first_token, remainder): (&str, &str) = step_text
            .split_once([' '])
            .unwrap_or((step_text, ""));
        let remainder = remainder.trim();

        let step_type: &str = first_token
            .split_once(['+', '?', '#', '.', '[', '^'])
            .map(|(a, b)| a)
            .unwrap_or(first_token);

        // println!("Parsed step type for \"{step_text}\" is {step_type}");

        // Parse ID, classes, and optional modifier
        let mut attr: CommonAttributes = Default::default();
        let mut modifiers: GeneratorModifiers = Default::default();
        let mut modifiers_lex = AttributeToken::lexer(first_token);
        let mut first_modifier = true;
        while let Some(result) = modifiers_lex.next() {
            if let Ok(token) = result { match token {
                AttributeToken::ID => { attr.id = Some(modifiers_lex.slice().trim().to_owned()) }
                AttributeToken::Class => { attr.classes.push(modifiers_lex.slice().trim().to_owned()) }
                AttributeToken::Optional => { modifiers.optional = true }
                AttributeToken::Modifiers => {
                    let modifier_chars = modifiers_lex.slice().trim().to_ascii_lowercase();
                    if (first_modifier) {
                        modifiers.case.originalcase = false;
                        first_modifier = false;
                    }
                    if modifier_chars.contains('u') { modifiers.case.uppercase = true }
                    if modifier_chars.contains('l') { modifiers.case.lowercase = true }
                    if modifier_chars.contains('o') { modifiers.case.originalcase = true }
                    if modifier_chars.contains('t') { modifiers.case.titlecase = true }
                }
                AttributeToken::Hidden => { modifiers.hidden = true }
            } }
        }

        match step_type {
            "w" | "word" | "wl" | "wordlist" => Ok(RecipeStep::Wordlist { filename: remainder.to_owned(), attr, modifiers }),
            "m" | "mask" => Ok(RecipeStep::Mask { mask: remainder.to_owned(), attr, modifiers }),
            "mi" | "maskinc" | "maskincremental" => Ok(RecipeStep::MaskIncremental { mask: remainder.to_owned(), attr, modifiers }),
            "c" | "const" | "constant" => Ok(RecipeStep::Constant { value: remainder.to_owned(), attr, modifiers }),
            "d" | "dup" | "duplicate" => Ok(RecipeStep::Duplicate { target_id: remainder.to_owned(), attr, modifiers }),
            "l" | "loc" | "location" => Ok(RecipeStep::Location { attr }),
            "r" | "rearr" | "rearrange" => Ok(RecipeStep::Rearrange { target_list: remainder.split_whitespace().map(|s| s.trim().to_owned()).collect() }),
            "cc" | "concat" => Ok(RecipeStep::Concat { target_list: remainder.split_whitespace().map(|s| s.trim().to_owned()).collect(), attr, modifiers }),
            "rep" | "replace" => {
                let remaining_tokens: Vec<String> = remainder.split_whitespace().map(|s| s.trim().to_owned()).collect();
                let target_id = remaining_tokens[0].clone();
                let mut repl_chars: Vec<(char, char)> = Vec::new();
                for repl_str in &remaining_tokens[1..] {
                    repl_chars.push((repl_str.chars().nth(0).unwrap(), repl_str.chars().nth(1).unwrap()));
                }
                Ok(RecipeStep::ReplaceChar { target_id, replacements: repl_chars, attr, modifiers })
            }
            "sp" | "space" => Ok(RecipeStep::Constant { value: " ".to_owned(), attr, modifiers }),
            _ => Err(RecipeParseError)
        }

    }
}

#[derive(Logos, Debug, PartialEq)]
enum AttributeToken {
    #[regex(r"\#\w+")]
    ID,
    #[regex(r"\.\w+")]
    Class,
    #[regex(r"\+[ulot]+")]
    Modifiers,
    #[token("?")]
    Optional,
    #[token("^")]
    Hidden
}

#[derive(Debug, Clone)]
struct RecipeParseError;