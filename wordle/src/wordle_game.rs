// pub enum LetterInfo {
//     Unknown,
//     Missing,
//     ExistsSomewhere(Vec<usize>),
//     ExistsAt(Vec<usize>, Vec<usize>)
// }

// pub struct WordleGame {
//     pub game: HashMap<char, LetterInfo>,
//     pub guesses: Vec<String>,
//     pub target: String
// }

// impl WordleGame {
//     pub fn new(target: String) -> Self {
//         let mut default_game = HashMap::new();
//         for char in "abcdefghijklmnopqrstuvwxyz".chars() {
//             default_game.insert(char, LetterInfo::Unknown);
//         }

//         WordleGame {
//             target,
//             game: default_game,
//             guesses: Vec::new(),
//         }
//     }

//     pub fn unknowns(&self) -> impl Iterator<Item = char> + '_{
//         self.game.iter().filter_map(|(k, v)| {
//             if let LetterInfo::Unknown = v { Some(*k) } else { None }
//         })
//     }

//     pub fn exists_ats(&self) -> impl Iterator<Item = (char, Vec<usize>, Vec<usize>)> + '_{
//         self.game.iter().filter_map(|(char, letter_info)| {
//             match letter_info {
//                 LetterInfo::ExistsAt(exists, may_exist) => Some((*char, exists.clone(), may_exist.clone())),
//                 _ => None
//             }
//         })
//     }

//     pub fn exists_somewheres(&self) -> impl Iterator<Item = (char, Vec<usize>)> + '_{
//         self.game.iter().filter_map(|(char, letter_info)| {
//             match letter_info {
//                 LetterInfo::ExistsSomewhere(indices) => Some((*char, indices.clone())),
//                 _ => None
//             }
//         })
//     }

//     pub fn choose_strategy(&self) -> GuessStrategy {
//         if self.exists_somewheres().count() >= 3 {
//             GuessStrategy::Mode3
//         } else if self.exists_ats().fold(0, |acc, (exist_ats, _)| exist_ats.len() + acc).count() >= 3 {
//             GuessStrategy::Mode2
//         } else {
//             GuessStrategy::Mode1
//         }
//     }
// }

// #[test]
// fn test_wordle_game_new() {
//     let game = WordleGame::new("guess".to_string());
//     assert_eq!(game.target, "guess");
// }

// #[test]
// fn test_wordle_game_choose_strategy() {
//     let game = WordleGame::new("guess".to_string());
//     assert_eq!(game.choose_strategy(), "random");
// }

// #[test]
// fn test_wordle_game_update_game() {

// }

