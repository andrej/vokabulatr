use std::{env, thread::sleep, time::Duration};

mod quiz;

enum ProgramMode {
    Quiz,
    Menu,
    Preferences,
    Quit
}

fn menu_loop() -> ProgramMode {
    let mut input = String::new();
    println!(
        r#"Menu
q: Quits the program.
p: Change matching preferences.
f: Flip the cards.
s: Shuffle the cards.
r: Return to quiz.
        "#
    );
    loop {
        println!("(Enter a command, or type h for help) ");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Could not read from standard input.");
        match input.as_str() {
            "q\n" => {
                break ProgramMode::Quit
            },
            "r\n" => {
                break ProgramMode::Quiz
            }
            "p\n" => {
                break ProgramMode::Preferences
            }
            "h\n" => {
            }
            _ => { println!("Unknown command {}.", &input[0..input.len()-1]); }
        }
    }
}

fn preferences_loop() -> ProgramMode {
    panic!("Not implemented");
    ProgramMode::Menu
}

fn quiz_loop(quiz: &mut quiz::Quiz, matcher: &mut quiz::Matcher) -> ProgramMode {
    let flashcard = quiz.current();
    let question = &flashcard.front;
    let answer_key = flashcard.back.clone();
    let percent_recently_correct = if flashcard.statistics.recent_attempts() == 0 {
        100.0
    } else {
        flashcard.statistics.recently_correct() as f64 / flashcard.statistics.recent_attempts() as f64 * 100.0
    };
    let history_string = flashcard.statistics.history_string("✔", "✗");
    let global_history_string = quiz.aggregate_statistics.history_string("✔", "✗");
    println!("");
    println!("-{:->3}/{:-<3}--{:-<64}-{:->3}%-", quiz.current, quiz.items.len(), history_string, percent_recently_correct);
    println!("|  {:74}  |", question);
    println!("--{:->76}--", global_history_string);
    println!("");

    let mut correct = false;
    while !correct {
        println!("(Type answer, press enter for menu, or type q to quit) ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).expect("could not read from stdin");

        if answer == "\n" {
            return ProgramMode::Menu
        } else if answer == "q\n" {
            return ProgramMode::Quit
        }
        correct = matcher.matches(answer_key.as_str(), answer.as_str());
        quiz.record_attempt(correct);
        if correct {
            println!("✔ Correct!");
            println!("");
        } else {
            println!("✗ Correct answer: {}", answer_key);
            println!("");
        }
    }

    quiz.advance();
    ProgramMode::Quiz
}

fn quit_screen(quiz: &quiz::Quiz) {
    let percent_correct = quiz.aggregate_statistics.correct as f64 / quiz.aggregate_statistics.attempts as f64;
    println!("Attempts: {:5}", quiz.aggregate_statistics.attempts);
    println!("Correct:  {:5}, {:3}%", quiz.aggregate_statistics.correct, percent_correct);
    println!("Goodbye!");
}

fn main() {
    let mut program_mode = ProgramMode::Quiz;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the path to a CSV file as an input argument.");
    }
    let file = std::fs::File::open(&args[1]).expect("Could not open file");
    let mut quiz = quiz::Quiz::from_csv_input(file);
    let mut matcher = quiz::Matcher::new();
    loop {
        program_mode = match program_mode {
            ProgramMode::Quiz => quiz_loop(&mut quiz, &mut matcher),
            ProgramMode::Menu => menu_loop(),
            ProgramMode::Preferences => preferences_loop(),
            ProgramMode::Quit => {
                quit_screen(&quiz);
                break
            }
        }
    }
}
