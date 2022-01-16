use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use wordle::{Wordle, WordleGame};

fn setup_logging() -> (Sender<String>, JoinHandle<()>) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let logger_handle = thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            println!("{}", msg);
        }
    });
    (tx, logger_handle)
}

fn main() {
    let solution_lines = include_str!("../wordlist_solutions.txt").to_string();
    let solution_dict = solution_lines.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    // let solution_dict = vec!["evade"];
    let (tx, logger_handle) = setup_logging();

    let mut guesses_required: Vec<usize> = Vec::new();
    for current_round in 0..solution_dict.len() {
        if current_round % 50 == 0 { print_average(&guesses_required); }
        // let mut wordle = Wordle::new();
        let tx_logger = tx.clone();
        let mut game = WordleGame::new(solution_dict[current_round].to_string());

        println!("Target Word: {}", solution_dict[current_round]);
        guesses_required.push(Wordle::run(&mut game, tx_logger));
        println!("Num Guesses: {}", guesses_required.last().unwrap());
    }
    print_average(&guesses_required);

    drop(tx);
    logger_handle.join().expect("The logger panicked");
}

fn print_average(guesses_required: &Vec<usize>) {
    let total_guesses = guesses_required.iter().map(|n| *n as u32).sum::<u32>();
    println!("<<<<< Avg Guesses: {}", total_guesses as f32 / guesses_required.len() as f32);
}

// #[test]
// fn test_score_correct_word() {
//     let mut wordle = WordleMaster::new(vec!["hello"]);
//     wordle.run("hello", None);
//     assert_eq!(wordle.score("hello"), 38);
// }

// #[test]
// fn test_guess_correct_word() {
//     let mut wordle = WordleMaster::new(vec!["hello"]);
//     let handle = wordle.run("hello", None);
//     assert_eq!(wordle.guess("hello"), Some("hello".to_string()));
//     assert_eq!(wordle.num_guesses, 1);
// }

// #[test]
// fn test_guess_incorrect_word() {
//     let mut wordle = WordleMaster::new(vec!["hello", "world"]);
//     let handle = wordle.run("hello", None);
//     assert_eq!(wordle.guess("world"), None);
//     assert_eq!(wordle.num_guesses, 1);
// }

// #[test]
// fn test_score_duplicate_guess() {
//     let mut wordle = WordleMaster::new(vec!["hello", "pizza"]);
//     let handle = wordle.run("hello", None);
//     assert_eq!(wordle.guess("pizza"), None);
//     assert_eq!(wordle.num_guesses, 1);
//     // You should not get points for guessing a letter already guessed
//     // if it is missing
//     assert_eq!(wordle.score("pizza"), 0);
// }

// #[test]
// fn test_score_does_nothing_for_words_not_in_dict() {
//     let mut wordle = WordleMaster::new(vec!["hello"]);
//     let handle = wordle.run("hello", None);
//     assert_eq!(wordle.num_guesses, 0);
//     assert_eq!(wordle.guess("not in dict"), None);
//     assert_eq!(wordle.num_guesses, 0);
// }

// #[test]
// fn test_score_a_round() {
//     let mut wordle = WordleMaster::new(vec!["hello", "pizza", "world", "jello"]);
//     let handle = wordle.run("hello", None);
//     assert_eq!(wordle.score("world"), 38);
//     assert_eq!(wordle.score("jello"), 36);
//     assert_eq!(wordle.guess("world"), None);
//     assert_eq!(wordle.score("jello"), 86);
//     assert_eq!(wordle.guess("jello"), None);
//     assert_eq!(wordle.score("jello"), 180);

// }

