mod helper;
mod parser;
mod questions;

use helper::intro;
use parser::parse_question_to_string;
use questions::{Ask, Question};

fn main() {
    intro();
    serde_json::from_str::<Vec<Question<String>>>(&parse_question_to_string())
        .unwrap()
        .ask();
}
