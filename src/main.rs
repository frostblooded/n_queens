use rand::seq::SliceRandom;
use std::io::stdin;

#[derive(Debug)]
struct Board {
    queens: Vec<u32>,
    main_diag_collis: Vec<u32>,
    secondary_diag_collis: Vec<u32>,
    n: u32,
}

impl Board {
    fn new(n: u32) -> Self {
        let mut board = Board {
            queens: vec![],
            main_diag_collis: vec![],
            secondary_diag_collis: vec![],
            n,
        };

        board.init();
        board
    }

    fn init(&mut self) {
        self.main_diag_collis = vec![0; (2 * self.n - 1) as usize];
        self.secondary_diag_collis = vec![0; (2 * self.n - 1) as usize];
        self.queens = Board::init_queens(self.n);

        for i in 0..self.n {
            let main_diag_index = i - self.queens[i as usize] + self.n - 1;
            self.main_diag_collis[main_diag_index as usize] += 1;

            let secondary_diag_index = i + self.queens[i as usize];
            self.secondary_diag_collis[secondary_diag_index as usize] += 1;
        }
    }

    fn init_queens(n: u32) -> Vec<u32> {
        let mut queens: Vec<u32> = vec![];

        for i in 0..n {
            queens.push(i);
        }

        queens
    }

    fn get_queen_main_diag_index(&self, queen_idx: usize) -> usize {
        queen_idx - self.queens[queen_idx] as usize + (self.n - 1) as usize
    }

    fn get_queen_secondary_diag_index(&self, queen_idx: usize) -> usize {
        queen_idx + self.queens[queen_idx] as usize
    }

    fn get_queen_collisions(&self, queen_idx: usize) -> u32 {
        self.main_diag_collis[self.get_queen_main_diag_index(queen_idx)]
            + self.secondary_diag_collis[self.get_queen_secondary_diag_index(queen_idx)]
    }

    fn get_all_max_min_queens(&self) -> (Vec<usize>, Vec<usize>) {
        let mut max_val = None;
        let mut max_queens = vec![];

        let mut min_val = None;
        let mut min_queens = vec![];

        for i in 0..(self.n as usize) {
            let curr_val = self.get_queen_collisions(i);

            if max_val.is_none() {
                max_val = Some(curr_val);
                max_queens = vec![i];
            } else if max_val.unwrap() == curr_val {
                max_queens.push(i);
            } else if max_val.unwrap() < curr_val {
                max_val = Some(curr_val);
                max_queens = vec![i];
            }

            if min_val.is_none() {
                min_val = Some(curr_val);
                min_queens = vec![i];
            } else if min_val.unwrap() == curr_val {
                min_queens.push(i);
            } else if min_val.unwrap() > curr_val {
                min_val = Some(curr_val);
                min_queens = vec![i];
            }
        }

        (max_queens, min_queens)
    }

    fn get_max_min_queen(&self) -> (usize, usize) {
        let (max_queens, min_queens) = self.get_all_max_min_queens();
        let max_queen = *max_queens.choose(&mut rand::thread_rng()).unwrap();
        let min_queen = *min_queens.choose(&mut rand::thread_rng()).unwrap();
        (max_queen, min_queen)
    }

    fn swap_queens(&mut self, idx1: usize, idx2: usize) {
        let main_diag_index = idx1 as u32 - self.queens[idx1 as usize] + self.n - 1;
        self.main_diag_collis[main_diag_index as usize] -= 1;

        let secondary_diag_index = idx1 as u32 + self.queens[idx1 as usize];
        self.secondary_diag_collis[secondary_diag_index as usize] -= 1;

        let main_diag_index = idx2 as u32 - self.queens[idx2 as usize] + self.n - 1;
        self.main_diag_collis[main_diag_index as usize] -= 1;

        let secondary_diag_index = idx2 as u32 + self.queens[idx2 as usize];
        self.secondary_diag_collis[secondary_diag_index as usize] -= 1;

        self.queens.swap(idx1, idx2);

        let main_diag_index = idx1 as u32 - self.queens[idx1 as usize] + self.n - 1;
        self.main_diag_collis[main_diag_index as usize] += 1;

        let secondary_diag_index = idx1 as u32 + self.queens[idx1 as usize];
        self.secondary_diag_collis[secondary_diag_index as usize] += 1;

        let main_diag_index = idx2 as u32 - self.queens[idx2 as usize] + self.n - 1;
        self.main_diag_collis[main_diag_index as usize] += 1;

        let secondary_diag_index = idx2 as u32 + self.queens[idx2 as usize];
        self.secondary_diag_collis[secondary_diag_index as usize] += 1;
    }

    fn should_stop(&self) -> bool {
        for i in 0..(self.n * 2 - 1) as usize {
            if self.main_diag_collis[i] > 1 {
                return false;
            }

            if self.secondary_diag_collis[i] > 1 {
                return false;
            }
        }

        true
    }

    fn solve(&mut self) {
        let mut i: u32 = 0;

        loop {
            let (max_index, min_index) = self.get_max_min_queen();
            self.swap_queens(min_index, max_index);

            i += 1;

            if i >= 10 * self.n {
                self.init();
                self.solve();
            }

            if self.should_stop() {
                break;
            }
        }
    }
}

fn main() {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");

    let n: u32 = buf.trim().parse().expect("Couldn't parse input to number");

    let mut board = Board::new(n);
    // dbg!(&board);

    board.solve();
    dbg!(&board);
}
