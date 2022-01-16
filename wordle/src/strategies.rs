use super::*;
use itertools::Itertools;

pub trait Strategy {
    fn build_scores(&self, game: &WordleGame) -> WordleScores;
    fn score(&self, word: &str, game: &WordleGame, scores: &WordleScores) -> u32;
}

// This needs to return something that implements a trait
// pub fn choose_strategy(game: &WordleGame) -> Box<dyn Strategy + Send + Sync> {
pub fn choose_strategy(game: &WordleGame) -> Box<dyn Strategy>  {
    let num_letters_positioned = game.exists_ats()
        .fold(0, |acc, (_, exist_ats, _)| exist_ats.len() + acc);
    if num_letters_positioned >= 3 {
            // println!("Using strategy 3");
            return Box::new(Mode3Strategy{});
    } else if num_letters_positioned + game.exists_somewheres().count() >= 3 {
        // println!("Using strategy 2");
        return Box::new(Mode2Strategy{});
    } else {
        // println!("Using strategy 1");
        return Box::new(Mode1Strategy{});
    }
}

#[derive(Clone)]
struct Mode1Strategy {}
impl Strategy for Mode1Strategy {
    fn build_scores(&self, game: &WordleGame) -> WordleScores {
        use LetterInfo::*;
        let mut scores = WordleScores::new();
        for (letter, letter_info) in game.game.iter() {
            match letter_info {
                Unknown => continue,
                _ => *scores.letter_scores.get_mut(letter).unwrap() = vec![0; 5],
            }
        }
        scores
    }

    fn score(&self, word: &str, _game: &WordleGame, scores: &WordleScores) -> u32 {
        let letter_scores = word
            .chars()
            .enumerate()
            .map(|(ind, c)| (c, scores.letter_scores.get(&c).unwrap()[ind]));
        letter_scores
            .unique_by(|(char, _)| *char)
            .map(|(_, val)| val)
            .sum()
    }
}
unsafe impl Send for Mode1Strategy {}

#[derive(Clone)]
struct Mode2Strategy {}
impl Strategy for Mode2Strategy {
    fn score(&self, word: &str, game: &WordleGame, scores: &WordleScores) -> u32 {
        let letter_scores = word
            .chars()
            .enumerate()
            .map(|(ind, c)| (c, scores.letter_scores.get(&c).unwrap()[ind]));

        let mut score: HashMap<char, u32> = HashMap::new();
        for (char, value) in letter_scores {
            match &game.game[&char] {
                LetterInfo::ExistsSomewhere(possible_slots) => {
                    if !score.contains_key(&char) || possible_slots.len() <= 2 {
                        score.insert(char, value);
                    } else if score.contains_key(&char) && possible_slots.len() > 2 {
                        *score.get_mut(&char).unwrap() += value;
                    }
                }
                _ => {
                    score.insert(char, value);
                }
            }
        }
        score.values().sum()
    }

    fn build_scores(&self, game: &WordleGame) -> WordleScores {
        use LetterInfo::*;
        let mut scores = WordleScores::new();
        for (letter, letter_info) in game.game.iter() {
            match letter_info {
                Missing => {
                    *scores.letter_scores.get_mut(letter).unwrap() = vec![0; 5];
                }
                ExistsSomewhere(avail_slots) => {
                    let slots = scores.letter_scores.get_mut(letter).unwrap();
                    for slot in 0..5 {
                        slots[slot] = if avail_slots.contains(&slot) { 45 } else { 0 };
                    }
                }
                ExistsAt(_, _) => *scores.letter_scores.get_mut(letter).unwrap() = vec![0; 5],
                _ => {}
            }
        }

        scores
    }
}

#[derive(Clone)]
struct Mode3Strategy {}

impl Mode3Strategy {
    fn is_solution_possible(word: &String, game: &WordleGame) -> bool {
        word.chars()
            .enumerate()
            .all(|(ind, char)| match &game.game[&char] {
                LetterInfo::ExistsSomewhere(possible_slots) => possible_slots.contains(&ind),
                LetterInfo::Missing => false,
                _ => true,
            })
    }
}
impl Strategy for Mode3Strategy {
    fn score(&self, word: &str, game: &WordleGame, scores: &WordleScores) -> u32 {
        if !Mode3Strategy::is_solution_possible(&word.to_string(), game) {
            return 0;
        }

        word
            .chars()
            .enumerate()
            .map(|(ind, char)| scores.letter_scores[&char][ind])
            .sum::<u32>()
    }



    fn build_scores(&self, game: &WordleGame) -> WordleScores {
        let mut scores = WordleScores::zeros();

        game.unknowns().for_each(|char| {
            scores.letter_scores.insert(char, vec![1; 5]);
        });
        game.exists_ats().for_each(|(c, locations, may_exist)| {
            locations
                .iter()
                .for_each(|col_index| scores.set_char_column_score(c, *col_index, 45));
            may_exist
                .iter()
                .for_each(|col_index| scores.set_char_column_score(c, *col_index, 10));
        });
        game.exists_somewheres().for_each(|(c, locations)| {
            locations
                .iter()
                .for_each(|col_index| scores.set_char_column_score(c, *col_index, 10));
        });
        scores
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mode1_strategy_build_scores() {
        let mut game = WordleGame::new("guess".to_string());
        *game.game.get_mut(&'c').unwrap() = LetterInfo::Missing;
        *game.game.get_mut(&'d').unwrap() = LetterInfo::ExistsSomewhere(vec![0]);
        *game.game.get_mut(&'e').unwrap() = LetterInfo::ExistsAt(vec![0], vec![1, 2]);
        let strat = Mode1Strategy{};
        let scores = strat.build_scores(&game);
        assert_eq!(scores.letter_scores[&'a'], vec![10; 5]);
        assert_eq!(scores.letter_scores[&'c'], vec![0; 5]);
        assert_eq!(scores.letter_scores[&'d'], vec![0; 5]);
        assert_eq!(scores.letter_scores[&'e'], vec![0; 5]);
    }

    #[test]
    fn test_mode1_strategy_score() {
        let game = WordleGame::new( "guess".to_string() );
        let strat = Mode1Strategy{};
        let scores = strat.build_scores(&game);
        assert_eq!(
            strat.score(&"guess".to_string(), &game, &scores),
            35
        );
    }

    #[test]
    fn test_mode2_strategy_build_scores() {
        let mut game = WordleGame::new("guess".to_string());

        *game.game.get_mut(&'c').unwrap() = LetterInfo::ExistsSomewhere(vec![0, 1, 2]);
        *game.game.get_mut(&'d').unwrap() = LetterInfo::ExistsAt(vec![0], vec![1, 2]);
        *game.game.get_mut(&'e').unwrap() = LetterInfo::ExistsAt(vec![1, 2], vec![4, 5]);
        let scores = Mode2Strategy{}.build_scores(&game);

        assert_eq!(scores.letter_scores[&'b'], vec![3; 5]);
        assert_eq!(scores.letter_scores[&'c'], vec![45, 45, 45, 0, 0]);
        assert_eq!(scores.letter_scores[&'d'], vec![0; 5]);
        assert_eq!(scores.letter_scores[&'e'], vec![0; 5]);
    }

    #[test]
    fn test_mode2_strategy_score() {
        let mut game = WordleGame::new("guess".to_string());
        let strat = Mode2Strategy{};
        let scores = strat.build_scores(&game);
        // let scores = vec![('g', 1), ('u', 2), ('e', 4), ('s', 5), ('s', 5)];

        let score = Mode2Strategy{}.score("guess", &game, &scores);
        assert_eq!(score, 12);

        *game.game.get_mut(&'s').unwrap() = LetterInfo::ExistsSomewhere(vec![1, 2]);
        let scores = strat.build_scores(&game);
        let score = Mode2Strategy{}.score("guess", &game, &scores);
        assert_eq!(score, 12);

        *game.game.get_mut(&'s').unwrap() = LetterInfo::ExistsSomewhere(vec![1, 2, 3]);
        let scores = strat.build_scores(&game);
        let score = Mode2Strategy{}.score("guess", &game, &scores);
        assert_eq!(score, 17);
    }

    #[test]
    fn test_mode3_build_scores() {
        let mut game_board = WordleGame::new("guess".to_string());
        *game_board.game.get_mut(&'b').unwrap() = LetterInfo::Missing;
        *game_board.game.get_mut(&'c').unwrap() = LetterInfo::ExistsSomewhere(vec![0, 1, 2]);
        *game_board.game.get_mut(&'d').unwrap() = LetterInfo::ExistsAt(vec![0], vec![1, 2]);
        *game_board.game.get_mut(&'e').unwrap() = LetterInfo::ExistsAt(vec![1, 2], vec![4]);
        *game_board.game.get_mut(&'f').unwrap() = LetterInfo::Unknown;
        let strat = Mode3Strategy{};
        let scores = strat.build_scores(&game_board);

        assert_eq!(scores.letter_scores[&'b'], vec![0; 5]);
        assert_eq!(scores.letter_scores[&'c'], vec![10, 10, 10, 0, 0]);
        assert_eq!(scores.letter_scores[&'d'], vec![45, 10, 10, 0, 0]);
        assert_eq!(scores.letter_scores[&'e'], vec![0, 45, 45, 0, 10]);
        assert_eq!(scores.letter_scores[&'f'], vec![1; 5]);
    }

    #[test]
    fn test_mode3_is_solution_possible() {
        let guess = "guess";
        let mut game_board = WordleGame::new(guess.to_string());

        assert_eq!(
            Mode3Strategy::is_solution_possible(&guess.to_string(), &game_board),
            true
        );

        game_board
            .game
            .insert('g', LetterInfo::ExistsSomewhere(vec![1, 2]));
        assert_eq!(
            Mode3Strategy::is_solution_possible(&guess.to_string(), &game_board),
            false
        );

        game_board
            .game
            .insert('g', LetterInfo::ExistsAt(vec![0], vec![]));
        assert_eq!(
            Mode3Strategy::is_solution_possible(&guess.to_string(), &game_board),
            true
        );

        game_board.game.insert('u', LetterInfo::Missing);
        assert_eq!(
            Mode3Strategy::is_solution_possible(&guess.to_string(), &game_board),
            false
        );
    }

    #[test]
    fn test_mode3_score() {
        // only score words that fully match, so this is a different algo I think.
        // assert_eq!(Mode3Strategy.score(&scores, &game_board), 100);
        let mut game = WordleGame::new("guess".to_string());
        let strat = Mode2Strategy{};
        let scores = strat.build_scores(&game);
        assert_eq!(
            strat.score("chats", &game, &scores),
            5
        );

        game
            .game
            .insert('c', LetterInfo::ExistsSomewhere(vec![0, 1, 2]));
        let scores = strat.build_scores(&game);
        assert_eq!(
            strat.score("chats", &game, &scores),
            14
        );

        game
            .game
            .insert('c', LetterInfo::ExistsAt(vec![0], vec![]));
        let scores = strat.build_scores(&game);
        assert_eq!(
            strat.score("chats", &game, &scores),
            49
        );

        game.game.insert('c', LetterInfo::Missing);
        let scores = strat.build_scores(&game);
        assert_eq!(
            strat.score("chats", &game, &scores),
            0
        );
    }
}
