use hashbrown::HashMap;
use rand::Rng;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct WordleMaster {
    target: String,
    guesses: Vec<String>,
    pub num_guesses: u32,
    workers: Vec<WordleSlave>,
    letter_vals: HashMap<char, Vec<u32>>,
    been_guessed: HashMap<char, bool>,
}

impl WordleMaster {
    pub fn new(dict: Vec<&str>) -> Self {
        let mut rng = rand::thread_rng();
        let target = dict[rng.gen_range(0..dict.len())].to_string();
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

        let been_guessed: HashMap<char, bool> = HashMap::new();

        Self {
            target, letter_vals, been_guessed,
            guesses: Vec::new(),
            num_guesses: 0,
            workers: Vec::new(),
        }
    }

    pub fn run(&mut self, dict: &'static Vec<String>, target: &str, tx_logger: Option<Sender<String>>) {
        let target_clone = target.to_string();
        // self.target = target.to_string();
        let (tx_guess, rx_guess): (Sender<(String, u32)>, Receiver<(String, u32)>) = mpsc::channel();

        let letter_vals = self.letter_vals.clone();
        let been_guessed = self.been_guessed.clone();
        let num_threads = 4;
        loop {
            // for each CPU, split the dictionary into equal sizes, and then score them. - This would require sharing the scoring stuff though.
            // the problem is that when I split the dictionary, or even when I clone it, it's not really duping the dictionary
            // it must be creating slices to the old memory. I need the dict to disappear entirely.
            // to do this, I need to learn how strings and vectors will copy / clone
            // then figure out how to truly make that happen. Either that, or I need to assure the compiler that these threads
            // will not outlive my function or scope.
            dict.chunks(num_threads)
                .map(|dict_portion| {
                    let worker_guesser = tx_guess.clone();
                    let mut guess: String = String::new();
                    thread::spawn(move || {
                        let mut best_score = 0;
                        for word in dict_portion {

                            let new_score = 20;
                            if new_score > best_score {
                                guess = word.to_string();
                                best_score = new_score;
                            }
                        }
                        worker_guesser.send((guess, best_score)).unwrap();
                    })
                })
                .for_each(|handle| handle.join().unwrap());

            let (guess, score) = rx_guess
                .iter()
                .max_by(|(_, score1), (_, score2)| score1.cmp(score2))
                .unwrap();


            match self.guess(dict, &guess) {
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

    pub fn guess(&mut self, dict: &'static Vec<String>, guess: &str) -> Option<String> {
        if !dict.contains(&guess.to_string()){
            return None;
        }
        self.num_guesses += 1;
        for worker in self.workers.iter_mut() {
            if worker.guess(guess).is_some() { return Some(guess.to_string()) }
        }
        None
    }

    pub fn score(&self, guess: &str) -> u32 {
        self.workers.first().unwrap().score(guess)
    }
}

#[derive(Debug, Clone)]
struct WordleSlave {
    target: String,
    dict: Vec<String>,
    letter_vals: HashMap<char, Vec<u32>>,
    been_guessed: HashMap<char, bool>,
}

impl WordleSlave {
    fn new(target: &str, dict: Vec<String>, letter_vals: HashMap<char, Vec<u32>>, been_guessed: HashMap<char, bool>) -> Self {
        Self {
            target: target.to_string(),
            dict: dict,
            letter_vals: letter_vals,
            been_guessed: been_guessed,
        }
    }

    fn been_guessed(&self, char: &char) -> bool {
        *self.been_guessed.get(char).unwrap_or(&false)
    }

    fn set_target(&mut self, target: &str) {
        self.target = target.to_string();
    }

    fn score(&self, guess: &str) -> u32 {
        let mut score_map = HashMap::new();
        for (i, c) in guess.chars().enumerate() {
            let score = score_map.entry(c).or_insert(0);
            // if a letter has not been guessed, then only award value for its first occurrence
            if !self.been_guessed(&c) && score == &0 {
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

    fn guess(&mut self, guess: &str) -> Option<String> {
        // println!("{}: {}", guess, self.score(guess));
        // self.num_guesses += 1;
        if self.target == guess {
            Some(guess.to_string())
        } else {
            self.update_scoring(guess);
            None
        }
    }

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
