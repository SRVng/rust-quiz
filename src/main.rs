mod questions;

use questions::{intro, Quiz, Quizzes};

fn main() {
    intro();

    let test: Vec<Quizzes> = vec![
        Quizzes {
            question: "which programming language is this CLI written in",
            choices: vec!["Typescript", "Python", "Rust", "English"],
            answer: "C",
        },
        Quizzes {
            question: "What does the abbreviation \"LRU Cache\" stand for",
            choices: vec![
                "Laugh Really Ugly",
                "Lone Recovery Union",
                "Large Resemble Union",
                "Least Recently Used",
            ],
            answer: "D",
        },
    ];

    test.ask();
}
