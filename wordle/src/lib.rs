use hashbrown::HashMap;
use strategies::Strategy;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use lazy_static::lazy_static;
mod strategies;

use crate::strategies::choose_strategy;

static NUM_THREADS: usize = 4;
lazy_static! {
    static ref GUESS_WORDS: Vec<&'static str> = {
        include_str!("../words.txt")
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>()
    };
}
// static GUESS_WORDS: &str = include_str!("../words.txt");
static SOLN_WORDS: &str = include_str!("../wordlist_solutions.txt");

pub struct Wordle {}

impl Wordle {
    pub fn run(game: &mut WordleGame, tx_logger: Sender<String>) -> usize {
        let mut num_guesses = 0;

        loop {
            let (tx_guess, rx_guess): (Sender<(String, u32)>, Receiver<(String, u32)>) =
                mpsc::channel();
            let scores= choose_strategy(&game).build_scores(&game);
            // let scores = &strategy.build_scores(&game);
            print!(".");
            (&GUESS_WORDS).chunks(GUESS_WORDS.len() / NUM_THREADS)
                .map(|dict| {
                    // Wordle::run_worker(dict, strategy, &game, &scores, &tx_guess, &tx_logger))
                    let worker_guesser = tx_guess.clone();
                    let thread_logger = tx_logger.clone();
                    let game_clone = game.clone();
                    let scores_clone = scores.clone();
                    // let strategy_clone = strategy.clone();
                    // print!("::thread");
                    thread::spawn(move || {
                        let mut best_score = 0;
                        let mut guess: String = String::new();
                        let strategy = choose_strategy(&game_clone);
                        // print!("::strategy found");
                        for word in dict.iter() {
                            let new_score = (&strategy).score(word, &game_clone, &scores_clone);
                            if new_score > best_score {
                                // println!("::new_score {} > {}, {}", new_score, best_score, word);
                                guess = word.to_string();
                                best_score = new_score;
                            }
                        }
                        // print!("::guess [{}]", guess);
                        worker_guesser.send((guess, best_score)).unwrap();
                    })
                })
                .for_each(|handle| handle.join().unwrap());
            // print!("::threads joined");
            drop(tx_guess);
            let guess = Wordle::get_best_guess(&rx_guess);
            // print!("\n::Best Guess: --{}--", guess);
            if game.guess(guess) { break; }
            // println!("::Nope...");
            // num_guesses += 1;
        }

        // println!("{} guesses", game.guesses.len() );
        game.guesses.len()
    }

    fn get_best_guess(receiver: &Receiver<(String, u32)>) -> String {
        let (guess, _) = receiver
            .iter()
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .unwrap();
        guess
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
        let mut game = HashMap::new();
        for char in "abcdefghijklmnopqrstuvwxyz".chars() {
            game.insert(char, LetterInfo::Unknown);
        }

        WordleGame {
            target, game,
            guesses: Vec::new(),
        }
    }

    fn remove_from_vec(vec: &mut Vec<usize>, val: usize) {
        let mut i = 0;
        while i < vec.len() {
            if vec[i] == val {
                vec.remove(i);
            } else {
                i += 1;
            }
        }
    }
    pub fn guess(&mut self, guess: String) -> bool {
        self.guesses.push(guess.clone());
        let target = self.target.chars().collect::<Vec<char>>();
        use LetterInfo::*;
        for (ind, char) in guess.chars().enumerate() {
            if target[ind] == char {
                // this one wins, so I'll def want this group
                // but I need to look at whether we knew anything about it before eventually.
                match &self.game.get(&char).unwrap() {
                    Unknown => {
                        let mut possible_indices = vec![0, 1, 2, 3, 4];
                        WordleGame::remove_from_vec(&mut possible_indices, ind);
                        self.game.insert(char, ExistsAt(vec![ind], possible_indices));
                    },
                    Missing => { panic!("Should not find a char that we thought was missing!")},
                    ExistsSomewhere(indices) => {
                        let mut indices = indices.clone();
                        WordleGame::remove_from_vec(&mut indices, ind);
                        self.game.insert(char, ExistsAt(vec![ind], indices));
                    },
                    ExistsAt(found_at, maybe_at) => {
                        let mut found_at = found_at.clone();
                        let mut maybe_at = maybe_at.clone();
                        WordleGame::remove_from_vec(&mut maybe_at, ind);
                        if !found_at.contains(&ind) { found_at.push(ind) }
                        self.game.insert(char, ExistsAt(found_at, maybe_at));
                    }
                }
            } else if target.contains(&char) {
                match self.game.get(&char).unwrap() {
                    ExistsSomewhere(indices) => {
                        let mut possible_indices = indices.clone();
                        WordleGame::remove_from_vec(&mut possible_indices, ind);
                        self.game.insert(char, LetterInfo::ExistsSomewhere(possible_indices ));
                    },
                    ExistsAt(found_at, maybe_at) => {
                        let mut found_at = found_at.clone();
                        let mut maybe_at = maybe_at.clone();
                        WordleGame::remove_from_vec(&mut maybe_at, ind);
                        self.game.insert(char, ExistsAt(found_at, maybe_at));
                    },
                    _ => {
                        let mut possible_indices = vec![0, 1, 2, 3, 4];
                        WordleGame::remove_from_vec(&mut possible_indices, ind);
                        self.game.insert(char, LetterInfo::ExistsSomewhere(possible_indices ));
                    }
                };

            } else {
                self.game.insert(char, LetterInfo::Missing);
            }
        }

        loop  {
            // Find more info based on what letters we know.
            let stuff = self.game
                .iter()
                .filter_map(|(char, letter_info)| match letter_info {
                    LetterInfo::ExistsAt(exists, may_exist) => {
                        Some((*char, exists.clone(), may_exist.clone()))
                    }
                    _ => None,
                })
                .collect::<Vec<(char, Vec<usize>, Vec<usize>)>>();

            stuff.iter().for_each(|(char, to_clear, _)| {
                // ensure we don't have any of these indices in the maybe_at lists
                let game = self.game.clone();
                let to_fix: Vec<(&char, &LetterInfo)> = game
                    .iter()
                    .filter(|(_char, letter_info)| matches!(letter_info, ExistsAt(_, _) | ExistsSomewhere(_)))
                    .collect::<Vec<(&char, &LetterInfo)>>();

                for (target_char, letter_info) in to_fix {
                    if char == target_char { continue }
                    match letter_info {
                        ExistsAt(found_at, maybe_at) => {
                            let mut maybe_at = maybe_at.clone();
                            for ind in to_clear {
                                WordleGame::remove_from_vec(&mut maybe_at, *ind);
                            }
                            self.game.insert(*target_char, ExistsAt(found_at.to_vec(), maybe_at));
                        },
                        ExistsSomewhere(maybe_at) => {
                            let mut maybe_at = maybe_at.clone();
                            for ind in to_clear {
                                WordleGame::remove_from_vec(&mut maybe_at, *ind);
                            }
                            self.game.insert(*target_char, ExistsSomewhere(maybe_at));
                        },
                        _ => {}
                    }
                }
            });


            let list = self.exists_somewheres().filter(|(_c, opts)| opts.len() == 1).collect::<Vec<(char, Vec<usize>)>>();
            if list.len() == 0 { break }
            for (char, location) in list {
                self.game.insert(char, LetterInfo::ExistsAt(location, vec![]));
            }
        }

        return guess == self.target;
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
pub struct WordleScores {
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
        assert_eq!(game.guesses.len(), 0);
        matches!(game.game[&'k'], LetterInfo::Unknown);
    }

    // #[test]
    // fn test_wordle_game_choose_strategy() {
    //     let game = WordleGame::new("guess".to_string());
    //     assert_eq!(game.choose_strategy(), "random");
    // }

    #[test]
    fn test_wordle_game_update_game() {
        let mut game = WordleGame::new("guess".to_string());
        let result = game.guess("guess".to_string());
        assert!(result);
        assert_eq!(game.game[&'g'], LetterInfo::ExistsAt(vec![0], vec![]));
        assert_eq!(game.game[&'u'], LetterInfo::ExistsAt(vec![1], vec![]));

        // now guess something wrong - S exists but in the wrong spot
        let mut game = WordleGame::new("guess".to_string());
        let result = game.guess("start".to_string());
        assert!(!result);
        assert_eq!(game.game[&'s'], LetterInfo::ExistsSomewhere(vec![1, 2, 3, 4]));
        assert_eq!(game.game[&'t'], LetterInfo::Missing);
        assert_eq!(game.game[&'a'], LetterInfo::Missing);
        assert_eq!(game.game[&'r'], LetterInfo::Missing);

        // now make sure that the 'S' is slated for fewer locations
        let result = game.guess("misty".to_string());
        assert!(!result);
        assert_eq!(game.game[&'m'], LetterInfo::Missing);
        assert_eq!(game.game[&'i'], LetterInfo::Missing);
        assert_eq!(game.game[&'s'], LetterInfo::ExistsSomewhere(vec![1, 3, 4]));
        assert_eq!(game.game[&'t'], LetterInfo::Missing);
        assert_eq!(game.game[&'y'], LetterInfo::Missing);

        let result = game.guess("clash".to_string());
        assert!(!result);
        assert_eq!(game.game[&'c'], LetterInfo::Missing);
        assert_eq!(game.game[&'l'], LetterInfo::Missing);
        assert_eq!(game.game[&'a'], LetterInfo::Missing);
        assert_eq!(game.game[&'s'], LetterInfo::ExistsAt(vec![3], vec![1, 4]));
        assert_eq!(game.game[&'h'], LetterInfo::Missing);

        // Now get both of the S's in the right spot
        let result = game.guess("class".to_string());
        assert!(!result);
        assert_eq!(game.game[&'c'], LetterInfo::Missing);
        assert_eq!(game.game[&'l'], LetterInfo::Missing);
        assert_eq!(game.game[&'a'], LetterInfo::Missing);
        assert_eq!(game.game[&'s'], LetterInfo::ExistsAt(vec![3, 4], vec![1]));

        // make sure that now guessing an S in the wrong position doesn't wreck things
        let result = game.guess("start".to_string());
        assert!(!result);
        assert_eq!(game.game[&'s'], LetterInfo::ExistsAt(vec![3, 4], vec![1]));

        // now, work the ExistsSomewhere list down to just 1 possibility, and make sure it flips
        // to ExistsAt
        game.guess("agaaa".to_string());
        game.guess("aagaa".to_string());
        game.guess("aaaga".to_string());
        game.guess("aaaag".to_string());
        // g exists somewhere, but not in the right spot, but I know what that spot MUST be
        assert_eq!(game.game[&'g'], LetterInfo::ExistsAt(vec![0], vec![]));
    }
}
