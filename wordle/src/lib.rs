use hashbrown::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct WordleMaster {
    target: String,
    // dict: Vec<String>,
    guesses: Vec<String>,
    pub num_guesses: u32,
    workers: Vec<WordleSlave>,
    letter_vals: HashMap<char, Vec<u32>>,
    been_guessed: HashMap<char, bool>,
}

static GUESS_WORDS: &str = include_str!("../words.txt");
static SOLN_WORDS: &str = include_str!("../wordlist_solutions.txt");

impl WordleMaster {
    #[inline]
    pub fn GUESS_WORDS() -> Vec<&'static str> {
        GUESS_WORDS.split("\n").map(|x| x.trim()).collect::<Vec<&str>>()
    }

    #[inline]
    pub fn SOLN_WORDS() -> Vec<&'static str> {
        SOLN_WORDS.split("\n").map(|x| x.trim()).collect::<Vec<&str>>()
    }

    pub fn new(dict: Vec<&str>) -> Self {
        // let mut rng = rand::thread_rng();
        // let target = dict[rng.gen_range(0..dict.len())].to_string();
        let mut letter_vals = HashMap::new();
        letter_vals.insert('a',vec![10; 5]);
        letter_vals.insert('b',vec![3; 5]);
        letter_vals.insert('c',vec![3; 5]);
        letter_vals.insert('d',vec![5; 5]);
        letter_vals.insert('e',vec![10; 5]);
        letter_vals.insert('f',vec![3; 5]);
        letter_vals.insert('g',vec![5; 5]);
        letter_vals.insert('h',vec![3; 5]);
        letter_vals.insert('i',vec![10; 5]);
        letter_vals.insert('j',vec![1; 5]);
        letter_vals.insert('k',vec![2; 5]);
        letter_vals.insert('l',vec![10; 5]);
        letter_vals.insert('m',vec![3; 5]);
        letter_vals.insert('n',vec![10; 5]);
        letter_vals.insert('o',vec![10; 5]);
        letter_vals.insert('p',vec![3; 5]);
        letter_vals.insert('q',vec![1; 5]);
        letter_vals.insert('r',vec![10; 5]);
        letter_vals.insert('s',vec![10; 5]);
        letter_vals.insert('t',vec![10; 5]);
        letter_vals.insert('u',vec![10; 5]);
        letter_vals.insert('v',vec![3; 5]);
        letter_vals.insert('w',vec![3; 5]);
        letter_vals.insert('x',vec![1; 5]);
        letter_vals.insert('y',vec![3; 5]);
        letter_vals.insert('z',vec![1; 5]);

        let mut been_guessed: HashMap<char, bool> = HashMap::new();

        Self {
            target: "".to_string(),
            letter_vals, been_guessed,
            guesses: Vec::new(),
            // dict: dict.iter().map(|x| x.to_string()).collect(),
            num_guesses: 0,
            workers: Vec::new(),
        }
    }

    pub fn run(&mut self, target: &str, tx_logger: Sender<String>) {
        let (tx_guess, rx_guess): (Sender<(String, u32)>, Receiver<(String, u32)>) = mpsc::channel();
        let num_threads = 4;
        let chunk_size = GUESS_WORDS.split("\n").count() / num_threads;

        loop {
            tx_logger.send("looping".to_string()).unwrap();

            GUESS_WORDS
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .chunks(chunk_size)
            .map(|dict| dict.to_vec())
            .map(|dict| {
                    let worker_guesser = tx_guess.clone();
                    let target_clone = target.to_string();
                    let thread_logger = tx_logger.clone();
                    let letter_vals = self.letter_vals.clone();
                    let been_guessed = self.been_guessed.clone();

                    thread::spawn(move || {
                        let worker =  WordleSlave::new(target_clone, dict, letter_vals, been_guessed, thread_logger, worker_guesser);
                        worker.run();
                    })
                })
                .for_each(|handle| handle.join().unwrap());

            let (guess, score) = rx_guess
                .iter()
                .max_by(|(_, score1), (_, score2)| score1.cmp(score2))
                .unwrap();


            match self.guess(&guess) {
                Some(guess) => {
                    if self.num_guesses > 6 {
                        println!("{}  {}", guess, self.num_guesses);
                    }
                    // guesses_required.push(self.num_guesses as u32);
                    break;
                },
                None => {
                    // Should be an event fired by lib.rs and just logged here
                    // if self.num_guesses > 10 {
                    //     println!("GIVING UP on {} !!!!!!!!!!!!!!!!", self.target);
                    //     // println!("{:?}", self.letter_vals);
                    //     break;
                    // }

                }
            }


        }
    }

    pub fn guess(&mut self, guess: &str) -> Option<String> {
        if !WordleMaster::SOLN_WORDS().contains(&guess) {
            return None;
        }

        None
    }
}

#[derive(Debug, Clone)]
struct WordleSlave {
    target: String,
    dict: Vec<&'static str>,
    letter_vals: HashMap<char, Vec<u32>>,
    been_guessed: HashMap<char, bool>,
    logger: Sender<String>,
    guesser: Sender<(String, u32)>,
}

impl WordleSlave {
    fn new(target: String, dict: Vec<&'static str>, letter_vals: HashMap<char, Vec<u32>>, been_guessed: HashMap<char, bool>, logger: Sender<String>, guesser: Sender<(String, u32)>) -> Self {
        Self {
            target, dict, letter_vals, been_guessed, logger, guesser,
        }
    }

    fn been_guessed(&self, char: &char) -> bool {
        *self.been_guessed.get(char).unwrap_or(&false)
    }

    fn run(&self) -> () {
        let mut best_score = 0;
        let mut guess: String = String::new();
        println!("Thread Stuff: {:?}", self.letter_vals);
        for word in self.dict.iter() {
            let new_score = self.score(word);
            if new_score > best_score {
                guess = word.to_string();
                best_score = new_score;
            }
        }
        self.logger.send(format!("Thread finished: {}, {}", guess, best_score)).unwrap();
        self.guesser.send((guess, best_score)).unwrap();
    }

    fn score(&self, guess: &str) -> u32 {
        let mut score_map = HashMap::new();
        for (i, c) in guess.chars().enumerate() {
            let score = score_map.entry(c).or_insert(0);
            // if a letter has not been guessed, then only award value for its first occurrence
            if !self.been_guessed(&c) && score == &0 {
                println!("checking: {} of {}", c, guess);
                *score = self.letter_vals[&c][i];
            } else {
                // this char has been guessed before
                if self.letter_vals[&c][i] == 45 {
                    *score += 45;
                } else if *score > 0 {
                    let new_score = (self.letter_vals[&c][i] as f32 / 2.0).round() as u32;
                    *score += new_score;
                } else {
                    *score = self.letter_vals[&c][i];
                }
            }

            // if guess == self.target {
            //     println!("Char: {} Index: {} Total Score:{} Char Score at Ind:{}", c, i, score, self.letter_vals[&c][i]);
            // }
        }
        // if guess == self.target {
        //     println!("{:?}", self.letter_vals);
        //     println!("Total: {:?} {}", score_map, score_map.values().fold(0, |acc, score| acc + score));

        // }
        score_map.values().fold(0, |acc, score| acc + score)
    }

    // fn guess(&mut self, guess: &str) -> Option<String> {
    //     // println!("{}: {}", guess, self.score(guess));
    //     // self.num_guesses += 1;
    //     if self.target == guess {
    //         Some(guess.to_string())
    //     } else {
    //         self.update_scoring(guess);
    //         None
    //     }
    // }

    fn set_guessed(&mut self, char: &char) {
        if !self.been_guessed.contains_key(char) {
            self.been_guessed.insert(*char, true);
        }
    }

    fn update_scoring(&mut self, guess: &str) {
        let target_chars = self.target.chars().collect::<Vec<char>>();
        // update scoring
        guess
            .chars()
            .enumerate()
            .for_each(|(guess_index, guess_char)| {
                self.set_guessed(&guess_char);
                let scores = self.letter_vals.get_mut(&guess_char).unwrap();
                if target_chars[guess_index] == guess_char {
                    for letter_scores_index in 0..scores.len() {
                        if letter_scores_index == guess_index {
                            scores[letter_scores_index] = 45;
                        } else if scores[letter_scores_index] < 15 && scores[letter_scores_index] > 0 {
                            scores[letter_scores_index] = 15;
                        }
                    }
                } else if target_chars.contains(&guess_char) {
                    for letter_scores_index in 0..scores.len() {
                        if letter_scores_index == guess_index {
                            // println!("UPDATE SCORING - guess_char: {} guess_index{} this score:{}", guess_char, guess_index, scores[letter_scores_index]);
                            scores[letter_scores_index] = 0;
                        } else if scores[letter_scores_index] < 15 && scores[letter_scores_index] > 0 {
                            scores[letter_scores_index] = 15;
                        }
                    }
                } else {
                    // clear out the score entirely
                    *scores = vec![0; 5];
                }
            });
    }
}
