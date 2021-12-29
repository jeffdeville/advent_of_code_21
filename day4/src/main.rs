fn main() {
    let file = File::open("day1/input.txt").expect("file not found");
    let reader = BufReader::new(file);
    // create a Vec<BingoCard>
    // read the first line into the deck
    // then, in a while loop
    // read the blank
    // read 5 lines into rows, and make a bingo card
    //
}

#[derive(Debug)]
struct BingoCard {
    board: Vec<Vec<u128>>,
    marks: Vec<Vec<bool>>
}

impl BingoCard {
    fn new(rows: Vec<Vec<u128>>) -> BingoCard {
        // questioning whether there's a way to initialize this
        // value and then lock it. But don't worry about it

        BingoCard {
            board: rows,
            marks: vec![vec![false; 5]; 5]
        }
    }

    fn play_round(&mut self, target: u128) -> () {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == target {
                    self.marks[row][col] = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for dim1 in 0..5 {
            if self.marks[dim1] == vec![true, true, true, true, true] {
                return true;
            }
            // because this grid is square, I can do this:
            if self.marks.iter().map(|row| row[dim1]).fold(true, |acc, val| acc & val) == true {
                return true;
            }
        }

        false
    }
}

#[test]
fn test_bingo_mechanics() {
    let rows = vec![
        vec![22, 13, 17, 11,  0],
        vec![8,  2, 23,  4, 24],
        vec![21, 9, 14, 16,  7],
        vec![6, 10, 3, 18,  5],
        vec![1, 12, 20, 15, 19],
    ];
    let mut bingo = BingoCard::new(rows.clone());
    assert!(!bingo.is_bingo());

    // bingo horizontally
    bingo.play_round(23);
    bingo.play_round(4);
    bingo.play_round(24);
    bingo.play_round(2);
    bingo.play_round(8);

    assert!(bingo.is_bingo());

    let mut bingo = BingoCard::new(rows.clone());
    assert!(!bingo.is_bingo());
    // bingo vertically
    bingo.play_round(11);
    bingo.play_round(4);
    bingo.play_round(16);
    bingo.play_round(18);
    bingo.play_round(15);

    assert!(bingo.is_bingo());
}
