// #[derive(Debug)]
// pub struct Wordle {
//     workers: Vec<WordleSlave>,
// }


// impl Wordle {
//     pub fn new() -> Self {
//         Self {
//             guesses: Vec::new(),
//             num_guesses: 0,
//             workers: Vec::new(),
//         }
//     }

//     #[inline]
//     pub fn GUESS_WORDS() -> Vec<&'static str> {
//         GUESS_WORDS.split("\n").map(|x| x.trim()).collect::<Vec<&str>>()
//     }

//     fn split_dict() -> Vec<&'static str>{
//         GUESS_WORDS
//             .split("\n")
//             .map(|x| x.trim())
//             .collect::<Vec<&str>>()
//             .chunks(chunk_size)
//             .map(|dict| dict.to_vec())
//     }

//     pub fn run(&mut self, target: &str, tx_logger: Sender<String>) {
//         let num_threads = 4;
//         let chunk_size = GUESS_WORDS.split("\n").count() / num_threads;
//         println!("Target Word: {}", target);
//         loop {
//             let (tx_guess, rx_guess): (Sender<(String, u32)>, Receiver<(String, u32)>) = mpsc::channel();

//             split_dict()
//                 .map(|dict| {
//                     let worker_guesser = tx_guess.clone();
//                     let target_clone = target.to_string();
//                     let thread_logger = tx_logger.clone();
//                     let strategy = self.game.choose_strategy();
//                     // let letter_vals = self.letter_vals.clone();
//                     // let been_guessed = self.been_guessed.clone();

//                     thread::spawn(move || {
//                         let worker =  WordleSlave::new(target, dict, wordle_game, wordle_scores, thread_logger, worker_guesser);
//                         // let worker =  WordleSlave::new(target_clone, dict, letter_vals, been_guessed, thread_logger, worker_guesser);
//                         worker.run();
//                     })
//                 })
//                 .for_each(|handle| handle.join().unwrap());

//             drop(tx_guess);
//             let (guess, _score) = rx_guess
//                 .iter()
//                 .max_by(|(_, score1), (_, score2)| score1.cmp(score2))
//                 .unwrap();

//             // println!("Best guess: {} with score {}", guess, score);

//             if self.guess(target, &guess).is_some() { break }
//         }
//     }

//     pub fn guess(&mut self, target: &str, guess: &str) -> Option<String> {
//         if !Wordle::GUESS_WORDS().contains(&guess) {
//             println!("Hmmm... {}", guess);

//             return None;
//         }
//         self.num_guesses += 1;

//         println!("Target: {}, Guess: {}, GuessNum: {}", target, guess, self.num_guesses);
//         if target == guess {
//             println!("    {}", self.num_guesses);
//             return Some(guess.to_string());
//         }
//         self.update_scoring(target, guess);
//         None
//     }


//     fn update_scoring(&mut self, target: &str, guess: &str) { }
// }

