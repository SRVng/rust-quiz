mod helper;
mod questions;
mod parser;

use helper::intro;
use parser::parse_question_to_string;
use questions::{Question, Ask};

fn main() {
    intro();
    serde_json::from_str::<Vec<Question<String>>>(&parse_question_to_string().as_str()).unwrap().ask();
}
