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
        for word in self.dict.iter() {
            let new_score = self.score(word);
            if new_score > best_score {
                guess = word.to_string();
                best_score = new_score;
            }
        }
        // self.logger.send(format!("Thread finished: {}, {}", guess, best_score)).unwrap();
        self.guesser.send((guess, best_score)).unwrap();
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
        }

        score_map.values().fold(0, |acc, score| acc + score)
    }
}
