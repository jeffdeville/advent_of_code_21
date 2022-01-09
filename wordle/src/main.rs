use rand::Rng;
use hashbrown::HashMap;

fn main() {
    let lines = include_str!("../words.txt").to_string();
    let dict = lines.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let dict_copy = dict.clone();
    let solution_lines = include_str!("../wordlist_solutions.txt").to_string();
    let solution_dict = solution_lines.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    // let solution_dict = vec!["awake", "heath", "dwarf", "karma", "bench", "marry", "staff", "paper", "unfed", "crazy", "crate", "pound", "booby", "flume", "flick", "parry", "goner", "loopy", "spray", "kebab", "hatch", "hyper", "comet", "jaunt", "growl", "fling", "dozen", "boozy", "click", "pulpy", "fixer", "finer", "perch", "clock", "prove", "break", "conic", "brake", "craze", "picky", "gorge", "abbey", "proxy", "prick", "wince", "whack", "perky", "frame", "pause", "shake", "pupal", "dodge",];

    // let solution_dict = vec!["dwarf"]; // let solution_dict = vec!["fjord", "jaunt", "dozen", "boozy", "fixer"];

    let mut guesses_required: Vec<u32> = Vec::new();
    for current_round in 0..solution_dict.len() {
        let mut wordle = WordleMaster::new(dict.clone());
        wordle.set_target(solution_dict[current_round]);
        if current_round % 50 == 1 {
            println!("Current Average after {} rounds: {}", current_round + 1, guesses_required.iter().sum::<u32>() as f32 / guesses_required.len() as f32);
        }
        loop {
            // for each CPU, split the dictionary into equal sizes, and then score them. - This would require sharing the scoring stuff though.
            let mut guess: String = String::new();
            let mut best_score = 0;
            dict_copy.iter().for_each(|&word| {
                let new_score = wordle.score(word);
                if new_score > best_score {
                    guess = word.to_string();
                    best_score = new_score;
                }
            });

            // println!("\t{} - Guess: {} Score: {}", wordle.num_guesses, &guess, wordle.score(&guess));
            match &wordle.guess(&guess) {
                Some(guess) => {
                    if wordle.num_guesses > 6 {
                        println!("{}  {}", guess, wordle.num_guesses);
                    }
                    guesses_required.push(wordle.num_guesses as u32);
                    break;
                },
                None => {
                    if wordle.num_guesses > 10 {
                        println!("GIVING UP on {} !!!!!!!!!!!!!!!!", wordle.target);
                        // println!("{:?}", wordle.letter_vals);
                        break;
                    }

                }
            }
        }
    }
    let total_guesses = guesses_required.iter().sum::<u32>();
    println!("Avg Guesses: {}", total_guesses as f32 / solution_dict.len() as f32);
}

#[derive(Debug)]
struct WordleMaster {
    target: String,
    dict: Vec<String>,
    guesses: Vec<String>,
    num_guesses: u32,
    workers: Vec<WordleSlave>,
    letter_vals: HashMap<char, Vec<u32>>,
    been_guessed: HashMap<char, bool>,
}

impl WordleMaster {
    fn new(dict: Vec<&str>) -> Self {
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

        let mut been_guessed: HashMap<char, bool> = HashMap::new();

        Self {
            target, letter_vals, been_guessed,
            guesses: Vec::new(),
            dict: dict.iter().map(|x| x.to_string()).collect(),
            num_guesses: 0,
            workers: Vec::new(),
        }
    }

    fn set_target(&mut self, target: &str) {
        self.target = target.to_string();
        // just assumes this is only ever done once. Terrible
        self.workers.push(
            WordleSlave::new(&self.target, self.dict.clone(), self.letter_vals.clone(), self.been_guessed.clone())
        );
    }

    fn guess(&mut self, guess: &str) -> Option<String> {
        if !self.dict.contains(&guess.to_string()){
            return None;
        }
        self.num_guesses += 1;
        for worker in self.workers.iter_mut() {
            if worker.guess(guess).is_some() { return Some(guess.to_string()) }
        }
        None
    }

    fn score(&self, guess: &str) -> u32 {
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

#[test]
fn test_score_correct_word() {
    let mut wordle = WordleMaster::new(vec!["hello"]);
    wordle.set_target("hello");
    assert_eq!(wordle.score("hello"), 38);
}

#[test]
fn test_guess_correct_word() {
    let mut wordle = WordleMaster::new(vec!["hello"]);
    wordle.set_target("hello");
    assert_eq!(wordle.guess("hello"), Some("hello".to_string()));
    assert_eq!(wordle.num_guesses, 1);
}

#[test]
fn test_guess_incorrect_word() {
    let mut wordle = WordleMaster::new(vec!["hello", "world"]);
    wordle.set_target("hello");
    assert_eq!(wordle.guess("world"), None);
    assert_eq!(wordle.num_guesses, 1);
}

#[test]
fn test_score_duplicate_guess() {
    let mut wordle = WordleMaster::new(vec!["hello", "pizza"]);
    wordle.set_target("hello");
    assert_eq!(wordle.guess("pizza"), None);
    assert_eq!(wordle.num_guesses, 1);
    // You should not get points for guessing a letter already guessed
    // if it is missing
    assert_eq!(wordle.score("pizza"), 0);
}

#[test]
fn test_score_does_nothing_for_words_not_in_dict() {
    let mut wordle = WordleMaster::new(vec!["hello"]);
    wordle.set_target("hello");
    assert_eq!(wordle.num_guesses, 0);
    assert_eq!(wordle.guess("not in dict"), None);
    assert_eq!(wordle.num_guesses, 0);
}

#[test]
fn test_score_a_round() {
    let mut wordle = WordleMaster::new(vec!["hello", "pizza", "world", "jello"]);
    wordle.set_target("hello");
    assert_eq!(wordle.score("world"), 38);
    assert_eq!(wordle.score("jello"), 36);
    assert_eq!(wordle.guess("world"), None);
    assert_eq!(wordle.score("jello"), 86);
    assert_eq!(wordle.guess("jello"), None);
    assert_eq!(wordle.score("jello"), 180);

}

