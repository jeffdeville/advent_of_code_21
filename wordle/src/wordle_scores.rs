// struct WordleScores {
//     letter_scores: HashMap<char, Vec<i32>>,
// }

// impl WordleScores {
//     pub fn new() -> Self {
//         let mut letter_scores: HashMap<char,Vec<i32>> = HashMap::new();
//         letter_scores.insert('b',vec![3; 5]);
//         letter_scores.insert('a',vec![10; 5]);
//         letter_scores.insert('d',vec![5; 5]);
//         letter_scores.insert('c',vec![3; 5]);
//         letter_scores.insert('f',vec![3; 5]);
//         letter_scores.insert('e',vec![10; 5]);
//         letter_scores.insert('h',vec![3; 5]);
//         letter_scores.insert('g',vec![5; 5]);
//         letter_scores.insert('j',vec![1; 5]);
//         letter_scores.insert('i',vec![10; 5]);
//         letter_scores.insert('l',vec![10; 5]);
//         letter_scores.insert('k',vec![2; 5]);
//         letter_scores.insert('n',vec![10; 5]);
//         letter_scores.insert('m',vec![3; 5]);
//         letter_scores.insert('p',vec![3; 5]);
//         letter_scores.insert('o',vec![10; 5]);
//         letter_scores.insert('r',vec![10; 5]);
//         letter_scores.insert('q',vec![1; 5]);
//         letter_scores.insert('t',vec![10; 5]);
//         letter_scores.insert('s',vec![10; 5]);
//         letter_scores.insert('v',vec![3; 5]);
//         letter_scores.insert('u',vec![10; 5]);
//         letter_scores.insert('x',vec![1; 5]);
//         letter_scores.insert('w',vec![3; 5]);
//         letter_scores.insert('z',vec![1; 5]);
//         letter_scores.insert('y',vec![3; 5]);

//         WordleScores { letter_scores }
//     }

//     pub fn zeros() -> Self {
//         let mut letter_scores: HashMap<char,Vec<i32>> = HashMap::new();
//         for char in "abcdefghijklmnopqrstuvwxyz".chars() {
//             letter_scores.insert(char, vec![0; 5]);
//         }
//         WordleScores { letter_scores }
//     }

//     pub fn set_char_column_score(&mut self, char: char, col_index: usize, score: i32) -> () {
//         self.letter_scores.get_mut(&char).unwrap()[col_index] = score;
//     }
// }
