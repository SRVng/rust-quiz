use std::io::{self, Write};

pub trait Quiz {
    fn ask(self);
}

pub struct Quizzes<'a> {
    pub question: &'a str,
    pub choices: Vec<&'a str>,
    pub answer: &'a str,
}

pub fn intro() {
    println!();
    println!("=== Instructions ===");
    println!("Type A, B, C, or D (Non-case sensitive) to answer");
    println!("Type exit to leave the game");
    println!("Type help to show this again");
    println!();
}

fn prompt(symbol: &str) -> String {
    let mut line: String = String::new();

    print!("{}", symbol);

    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut line)
        .expect("Invalid line argument");

    line.trim().to_string()
}

fn print_question(quiz: &Quizzes, index: usize) {
    println!("\u{001b}[31;1mQ{}: {}", index + 1, quiz.question);

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

impl<'a> Quiz for Vec<Quizzes<'a>> {
    fn ask(self) {
        'outer: for (index, quiz) in self.into_iter().enumerate() {
            let mut wrong_counter: u8 = 0;

            print_question(&quiz, index);

            'inner: loop {
                match prompt("> ") {
                    answer if answer.to_lowercase().eq(&quiz.answer.to_lowercase()) => {
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
