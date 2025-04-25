#![allow(unused)]

mod recipe_parser;
mod recipe_step;
mod candidate_generator;

mod items;

use std::io::BufWriter;
use clap::Parser;
use crate::candidate_generator::CandidateGenerator;
use crate::recipe_parser::RecipeParser;

#[derive(Parser, Debug)]
#[command(version, about = "Password candidate generator using step-by-step recipes")]
struct PasswordChefArgs
{
    #[arg(short, long, value_name="FILE")]
    recipe: String,

    #[arg(short='w', long, help="Directory where wordlists will be checked", value_name="DIR")]
    wordlist_dir: Option<String>
}

fn main() -> std::io::Result<()> {
    let args = PasswordChefArgs::parse();

    // println!("Starting PasswordChef");

    let recipe_text = std::fs::read_to_string(&args.recipe)?;

    let recipe = RecipeParser::parse(recipe_text);

    let mut candidate_gen: CandidateGenerator = CandidateGenerator::from_recipe(recipe, Box::new(BufWriter::new(std::io::stdout())));

    while candidate_gen.print_next() {}

    Ok(())
}
