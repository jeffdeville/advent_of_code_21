use hashbrown::HashMap;
use strategies::Strategy;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

mod strategies;

use crate::strategies::choose_strategy;

pub struct Wordle {}

impl Wordle {
    #[inline]
    pub fn GUESS_WORDS() -> Vec<&'static str> {
        GUESS_WORDS
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
    }

    fn split_dict() -> Vec<Vec<&'static str>> {
        let guess_words = GUESS_WORDS
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let chunk_size = guess_words.len() / NUM_THREADS;
        guess_words
            .chunks(chunk_size)
            .map(|dict| dict.to_vec())
            .collect::<Vec<Vec<&'static str>>>()
    }

    pub fn run(game: WordleGame, tx_logger: Sender<String>) -> u32 {
        let mut guesses: Vec<String> = Vec::new();
        let mut num_guesses = 0;

        loop {
            let (tx_guess, rx_guess): (Sender<(String, u32)>, Receiver<(String, u32)>) =
                mpsc::channel();
            let strategy = choose_strategy(&game);
            let scores = strategy.build_scores(&game);

            Wordle::split_dict()
                .iter()
                .map(|dict| Wordle::run_worker(*dict, strategy, &game, &scores, &tx_guess, &tx_logger))
                .for_each(|handle| handle.join().unwrap());
            drop(tx_guess);
            let guess = Wordle::get_best_guess(&rx_guess);
            if game.guess(guess) {
                break;
            }
        }

        println!("{} guesses", num_guesses );
        num_guesses
    }

    fn run_worker(
        dict: Vec<&'static str>,
        strategy: impl Strategy + Clone + Send,
        game: &WordleGame,
        scores: &WordleScores,
        tx_guess: &Sender<(String, u32)>,
        tx_logger: &Sender<String>,
    ) -> JoinHandle<()> {
        let worker_guesser = tx_guess.clone();
        let thread_logger = tx_logger.clone();
        let game_clone = game.clone();
        let scores_clone = scores.clone();
        let strategy_clone = strategy.clone();

        thread::spawn(move || {
            let mut best_score = 0;
            let mut guess: String = String::new();
            for word in dict.iter() {
                let new_score = strategy_clone.score(word, &game_clone, &scores_clone);
                if new_score > best_score {
                    guess = word.to_string();
                    best_score = new_score;
                }
            }
            worker_guesser.send((guess, best_score)).unwrap();
        })
    }

    fn get_best_guess(receiver: &Receiver<(String, u32)>) -> String {
        let (guess, _) = receiver
            .iter()
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .unwrap();
        guess
    }
}

static NUM_THREADS: usize = 4;
static GUESS_WORDS: &str = include_str!("../words.txt");
static SOLN_WORDS: &str = include_str!("../wordlist_solutions.txt");

#[derive(Debug, Clone)]
pub enum LetterInfo {
    Unknown,
    Missing,
    ExistsSomewhere(Vec<usize>),
    ExistsAt(Vec<usize>, Vec<usize>),
}

#[derive(Debug, Clone)]
pub struct WordleGame {
    pub game: HashMap<char, LetterInfo>,
    pub guesses: Vec<String>,
    pub target: String,
}

impl WordleGame {
    pub fn new(target: String) -> Self {
        let mut default_game = HashMap::new();
        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
            default_game.insert(char, LetterInfo::Unknown);
        }

        WordleGame {
            target,
            game: default_game,
            guesses: Vec::new(),
        }
    }

    pub fn guess(&mut self, guess: String) -> bool {
        false
    }

    pub fn unknowns(&self) -> impl Iterator<Item = char> + '_ {
        self.game.iter().filter_map(|(k, v)| {
            if let LetterInfo::Unknown = v {
                Some(*k)
            } else {
                None
            }
        })
    }

    pub fn exists_ats(&self) -> impl Iterator<Item = (char, Vec<usize>, Vec<usize>)> + '_ {
        self.game
            .iter()
            .filter_map(|(char, letter_info)| match letter_info {
                LetterInfo::ExistsAt(exists, may_exist) => {
                    Some((*char, exists.clone(), may_exist.clone()))
                }
                _ => None,
            })
    }

    pub fn exists_somewheres(&self) -> impl Iterator<Item = (char, Vec<usize>)> + '_ {
        self.game
            .iter()
            .filter_map(|(char, letter_info)| match letter_info {
                LetterInfo::ExistsSomewhere(indices) => Some((*char, indices.clone())),
                _ => None,
            })
    }
}

#[derive(Debug, Clone)]
struct WordleScores {
    letter_scores: HashMap<char, Vec<u32>>,
}

impl WordleScores {
    pub fn new() -> Self {
        let mut letter_scores: HashMap<char, Vec<u32>> = HashMap::new();
        letter_scores.insert('b', vec![3; 5]);
        letter_scores.insert('a', vec![10; 5]);
        letter_scores.insert('d', vec![5; 5]);
        letter_scores.insert('c', vec![3; 5]);
        letter_scores.insert('f', vec![3; 5]);
        letter_scores.insert('e', vec![10; 5]);
        letter_scores.insert('h', vec![3; 5]);
        letter_scores.insert('g', vec![5; 5]);
        letter_scores.insert('j', vec![1; 5]);
        letter_scores.insert('i', vec![10; 5]);
        letter_scores.insert('l', vec![10; 5]);
        letter_scores.insert('k', vec![2; 5]);
        letter_scores.insert('n', vec![10; 5]);
        letter_scores.insert('m', vec![3; 5]);
        letter_scores.insert('p', vec![3; 5]);
        letter_scores.insert('o', vec![10; 5]);
        letter_scores.insert('r', vec![10; 5]);
        letter_scores.insert('q', vec![1; 5]);
        letter_scores.insert('t', vec![10; 5]);
        letter_scores.insert('s', vec![10; 5]);
        letter_scores.insert('v', vec![3; 5]);
        letter_scores.insert('u', vec![10; 5]);
        letter_scores.insert('x', vec![1; 5]);
        letter_scores.insert('w', vec![3; 5]);
        letter_scores.insert('z', vec![1; 5]);
        letter_scores.insert('y', vec![3; 5]);

        WordleScores { letter_scores }
    }

    pub fn zeros() -> Self {
        let mut letter_scores: HashMap<char, Vec<u32>> = HashMap::new();
        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
            letter_scores.insert(char, vec![0; 5]);
        }
        WordleScores { letter_scores }
    }

    pub fn set_char_column_score(&mut self, char: char, col_index: usize, score: u32) -> () {
        self.letter_scores.get_mut(&char).unwrap()[col_index] = score;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_wordle_game_new() {
        let game = WordleGame::new("guess".to_string());
        assert_eq!(game.target, "guess");
    }

    // #[test]
    // fn test_wordle_game_choose_strategy() {
    //     let game = WordleGame::new("guess".to_string());
    //     assert_eq!(game.choose_strategy(), "random");
    // }

    #[test]
    fn test_wordle_game_update_game() {}
}
