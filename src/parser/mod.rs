use std::fs::read_to_string;

pub fn parse_question_to_string<'a>() -> String {
    read_to_string("question.json").expect("Failed to read question.json")
}
