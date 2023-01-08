use serde::Deserialize;
use std::fmt::Display;

use crate::helper::{intro, prompt};

pub trait Builder<'a, T> {
    type Currying;
    fn builder() -> Self::Currying;
}

#[derive(Deserialize, Clone, Debug)]
pub struct Question<'a, T>
where
    T: Clone + Default,
    Vec<T>: Clone,
{
    pub question: &'a str,
    pub quote: Option<&'a str>,
    pub choices: Vec<T>,
    pub answer: &'a str,
}

impl Builder<'static, String> for Question<'static, String> {
    type Currying = Box<
        dyn FnOnce(
            &'static str,
        ) -> Box<
            dyn FnOnce(
                Option<&'static str>,
            ) -> Box<
                dyn FnOnce(
                    Vec<String>,
                ) -> Box<
                    dyn FnOnce(
                            &'static str,
                        )
                            -> Box<dyn FnOnce() -> Question<'static, String> + 'static>
                        + 'static,
                >,
            >,
        >,
    >;

    fn builder() -> Self::Currying {
        let mut builder = QuestionBuilder::default();

        Box::new(move |question: &'static str| {
            Box::new(move |quote: Option<&'static str>| {
                Box::new(move |choices: Vec<String>| {
                    Box::new(move |answer: &'static str| {
                        Box::new(move || {
                            builder.question(question);
                            builder.quote(quote);
                            builder.choices(choices);
                            builder.answer(answer);
                            builder.build()
                        })
                    })
                })
            })
        })
    }
}

#[derive(Default)]
pub struct QuestionBuilder<'a, T> {
    question: Option<&'a str>,
    quote: Option<&'a str>,
    choices: Option<Vec<T>>,
    answer: Option<&'a str>,
}

impl<'a, T> QuestionBuilder<'a, T>
where
    T: Clone + Default,
    Vec<T>: Clone,
{
    fn build(&self) -> Question<'a, T> {
        Question {
            question: self.question.expect("No question provided").into(),
            quote: self.quote.into(),
            choices: self.choices.clone().expect("No choices provided"),
            answer: self.answer.expect("No answer provided").into(),
        }
    }
    fn answer(&mut self, answer: &'a str) {
        self.answer = Some(answer);
    }

    fn choices(&mut self, choices: Vec<T>) {
        // TODO: Logic here
        self.choices = Some(choices);
    }

    fn quote(&mut self, quote: Option<&'a str>) {
        // TODO: Logic here

        self.quote = quote;
    }

    fn question(&mut self, question: &'a str) {
        // TODO: Logic here

        self.question = Some(question);
    }
}

pub trait Ask {
    fn ask(&self);
}

impl<'a, T> Ask for Vec<Question<'a, T>>
where
    T: Clone + Default + Display,
    Vec<T>: Clone,
{
    fn ask(&self) {
        'outer: for (index, quiz) in self.into_iter().enumerate() {
            let mut wrong_counter: u8 = 0;

            print_question(quiz, index);

            'inner: loop {
                match prompt("> ") {
                    answer if quiz.answer.to_lowercase().eq(&answer.to_lowercase()) => {
                        println!("Correct ✅");
                        break 'inner;
                    }
                    exit if exit.eq("exit") => {
                        println!("Goodbye! 👋");
                        break 'outer;
                    }
                    help if help.eq("help") => intro(),
                    _ => {
                        wrong_counter += 1;
                        println!("Try again!");
                        if wrong_counter == 8 {
                            wrong_counter = 0;
                            print_question(&quiz, index);
                        }
                    }
                }
            }
        }
    }
}

fn print_question<T>(quiz: &Question<T>, index: usize)
where
    T: Clone + Default + Display,
    Vec<T>: Clone,
{
    println!("\u{001b}[31;1mQ{}: {}", index + 1, &quiz.question);

    if let Some(quote) = &quiz.quote {
        println!(
            "\u{001b}[36;1m
        {}
        \u{001b}[0m",
            quote
        );
    }

    let choices = |choice_index: usize| -> char {
        if let Some((_, choice)) = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .char_indices()
            .find(|(index, _)| index.eq(&choice_index))
        {
            choice
        } else {
            panic!("Too much choice")
        }
    };

    quiz.choices.iter().enumerate().for_each(|(index, choice)| {
        println!("\u{001b}[37;1m{}) {}\u{001b}[0m", choices(index), choice)
    });
}
