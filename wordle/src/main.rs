use rand::Rng;
use hashbrown::HashMap;

fn main() {
    let lines = include_str!("../words.txt").to_string();
    let dict = lines.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let dict_copy = dict.clone();
    let mut guesses_required: Vec<u32> = Vec::new();
    let total_rounds=100;
    for current_round in 0..total_rounds {
        let mut wordle = Wordle::new(dict.clone());
        println!("Round {} Target: {}", current_round, wordle.target);
        loop {
            let mut guess: String = String::new();
            let mut best_score = 0;
            dict_copy.iter().for_each(|&word| {
                let new_score = wordle.score(word);
                if new_score > best_score {
                    guess = word.to_string();
                    best_score = new_score;
                }
            });

            println!("\t{} - Guess: {} Score: {}", wordle.num_guesses, &guess, wordle.score(&guess));
            match &wordle.guess(&guess) {
                Some(guess) => {
                    println!("Success! {} in {} tries", guess, wordle.num_guesses);
                    guesses_required.push(wordle.num_guesses as u32);
                    break;
                },
                None => {
                    if wordle.num_guesses > 20 {
                        println!("GIVING UP");
                        break;
                    }

                }
            }
        }
    }
    let total_guesses = guesses_required.iter().sum::<u32>();
    println!("Avg Guesses: {}", total_guesses as f32 / total_rounds as f32);
}

#[derive(Debug)]
struct Wordle {
    target: String,
    dict: Vec<String>,
    guesses: Vec<String>,
    num_guesses: usize,
    letter_vals: HashMap<char, Vec<u32>>,
}

impl Wordle {
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

        Self {
           target: target,
           guesses: Vec::new(),
           dict: dict.iter().map(|x| x.to_string()).collect(),
           num_guesses: 0,
           letter_vals: letter_vals,
       }
    }

    fn set_target(&mut self, target: &str) {
        self.target = target.to_string();
    }

    fn score(&self, guess: &str) -> u32 {
        let mut score_map = HashMap::new();
        for (i, c) in guess.chars().enumerate() {
            let score = score_map.entry(c).or_insert(0);
            if *score <= self.letter_vals[&c][i] {
                *score = self.letter_vals[&c][i] + (*score as f32 / 2 as f32) as u32;
            } else if *score >= 45 && self.letter_vals[&c][i] == 45{
                *score += 45;
            }
            // else if *score == self.letter_vals[&c][i] {
            //     *score = (*score as f32 / 2 as f32) as u32;
            // }
            // if guess == "hollo" {
            //     println!("{} {} {} {}", c, i, score, self.letter_vals[&c][i]);
            // }
        }
        // if guess == "hollo" {
        //     println!("Total: {:?} {}", score_map, score_map.values().fold(0, |acc, score| acc + score));
        // }
        score_map.values().fold(0, |acc, score| acc + score)
    }

    fn guess(&mut self, guess: &str) -> Option<String> {
        if !self.dict.contains(&guess.to_string()){
            return None;
        }
        self.num_guesses += 1;
        if self.target == guess {
            Some(guess.to_string())
        } else {
            self.update_scoring(guess);
            None
        }
    }

    fn update_scoring(&mut self, guess: &str) {
        let target_chars = self.target.chars().collect::<Vec<char>>();
        // update scoring
        guess
            .chars()
            .enumerate()
            .for_each(|(i, elem)| {
                let scores = self.letter_vals.get_mut(&elem).unwrap();
                if target_chars[i] == elem {
                    // go through scoring. If the value is below 25, raise it to 25
                    // but set the matched value to 45. This way, letters in the
                    // word twice can't be downgraded below 25
                    for score_i in 0..scores.len() {
                        if score_i == i {
                            scores[score_i] = 45;
                        } else if scores[score_i] < 25 && scores[score_i] > 0 {
                            scores[score_i] = 25;
                        }
                    }
                } else if target_chars.contains(&elem) {
                    for score_i in 0..scores.len() {
                        if score_i == i {
                            scores[score_i] = 0;
                        } else if scores[score_i] < 25 && scores[score_i] > 0 {
                            scores[score_i] = 25;
                        }
                    }
                } else {
                    *scores = vec![0; 5];
                }
            });
        // println!("{:?}", self.letter_vals);
    }

}

#[test]
fn test_score_correct_word() {
    let wordle = Wordle::new(vec!["hello"]);
    assert_eq!(wordle.score("hello"), 43);
}

#[test]
fn test_guess_correct_word() {
    let mut wordle = Wordle::new(vec!["hello"]);
    assert_eq!(wordle.guess("hello"), Some("hello".to_string()));
    assert_eq!(wordle.num_guesses, 1);
}

#[test]
fn test_guess_incorrect_word() {
    let mut wordle = Wordle::new(vec!["hello", "world"]);
    wordle.set_target("hello");
    assert_eq!(wordle.guess("world"), None);
    assert_eq!(wordle.num_guesses, 1);
}

#[test]
fn test_score_duplicate_guess() {
    let mut wordle = Wordle::new(vec!["hello", "pizza"]);
    wordle.set_target("hello");
    assert_eq!(wordle.guess("pizza"), None);
    assert_eq!(wordle.num_guesses, 1);
    // You should not get points for guessing a letter already guessed
    // if it is missing
    assert_eq!(wordle.score("pizza"), 0);
}

#[test]
fn test_score_does_nothing_for_words_not_in_dict() {
    let mut wordle = Wordle::new(vec!["hello"]);
    assert_eq!(wordle.num_guesses, 0);
    assert_eq!(wordle.guess("not in dict"), None);
    assert_eq!(wordle.num_guesses, 0);
}

#[test]
fn test_score_a_round() {
    let mut wordle = Wordle::new(vec!["hello", "pizza", "world", "jello"]);
    wordle.set_target("hello");
    assert_eq!(wordle.score("world"), 38);
    assert_eq!(wordle.score("jello"), 41);
    assert_eq!(wordle.guess("world"), None);
    assert_eq!(wordle.score("jello"), 106);
    assert_eq!(wordle.guess("jello"), None);
    assert_eq!(wordle.score("jello"), 180);

}

