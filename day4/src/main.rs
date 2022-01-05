#[derive(Debug)]
struct Board {
  board: [[(u32, bool); 5]; 5]
}

impl Board {
  fn new(numbers: &[u32]) -> Self {
    let mut board = Self {
      board: [[(0, false); 5]; 5]
    };

    for i in 0..5 {
      for j in 0..5 {
        board.board[i][j] = (numbers[i*5+j], false);
      }
    }

    board
  }
  fn mark_num(&mut self, num: u32) {
    for i in 0..5 {
      for j in 0..5 {
        if self.board[i][j].0 == num {
          self.board[i][j].1 = true;
        }
      }
    }
  }

  fn is_bingo(&self) -> bool {
    for i in 0..5 {
      if (0..5).all(|j| self.board[i][j].1) {
        return true;
      }
      if (0..5).all(|j| self.board[j][i].1) {
        return true;
      }
    }
    false
  }

  fn sum_unmarked(&self) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..5 {
      for j in 0..5 {
        if !self.board[i][j].1 {
          sum += self.board[i][j].0;
        }
      }
    }
    sum
  }
}

fn create_boards(boards: &str) -> Vec<Board> {
  let mut board_vec = boards
    .split("\n\n")
    .map(|board_lines| {
      let board_nums: Vec<u32> = board_lines
        .split_ascii_whitespace()
        .flat_map(|x| {
          x
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        })
        .collect();
      Board::new(&board_nums)
    })
    .collect();
    board_vec
}

fn main() {
  let (nums, boards) = include_str!("../input.txt").split_once("\n\n").unwrap();
  let mut bingo_boards: Vec<Board> = create_boards(boards);

  let parsed_nums = nums
    .split(",")
    .map(|x| x.parse::<u32>().unwrap())
    .collect::<Vec<u32>>();
  part_a(&parsed_nums, bingo_boards);

  let mut bingo_boards: Vec<Board> = create_boards(boards);
  part_b(&parsed_nums, bingo_boards);
}

fn part_a(parsed_nums: &Vec<u32>, mut bingo_boards: Vec<Board>) {
  for num in parsed_nums {
    bingo_boards.iter_mut().for_each(|board| {
      board.mark_num(*num);
    });
    let board = bingo_boards.iter().find(|board| board.is_bingo());
    if let Some(winner) = board {
      println!("BINGO!!! {:?}: \n\n{}", winner, winner.sum_unmarked() * num);
      break;
    }
  }
}

fn part_b(parsed_nums: &Vec<u32>, mut bingo_boards: Vec<Board>) {

  for num in parsed_nums {
    let not_won_yet: Vec<usize> = bingo_boards
      .iter()
      .enumerate()
      .filter_map(|(i, board)| if board.is_bingo() { None } else {Some(i) })
      .collect();
    bingo_boards.iter_mut().for_each(|board| { board.mark_num(*num) });

    if bingo_boards.iter().filter(|board| !board.is_bingo()).count() == 0 {
      let pos = not_won_yet[0];
      let score = num * bingo_boards[pos].sum_unmarked();
      println!("BINGO!!! {:?}: \n\n{}", bingo_boards[pos], score);
      break;
    }

  }
}
