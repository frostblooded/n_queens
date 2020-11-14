use rand::seq::SliceRandom;
use rand::Rng;
use std::io::stdin;

#[derive(Debug)]
struct Board {
    queens: Vec<u32>,
    main_diag_collis: Vec<u32>,
    secondary_diag_collis: Vec<u32>,
    row_collis: Vec<u32>,
    n: u32,
}

impl Board {
    fn new(n: u32) -> Self {
        let mut board = Board {
            queens: vec![],
            main_diag_collis: vec![],
            secondary_diag_collis: vec![],
            row_collis: vec![],
            n,
        };

        board.init();
        board
    }

    fn init(&mut self) {
        self.main_diag_collis = vec![0; (2 * self.n - 1) as usize];
        self.secondary_diag_collis = vec![0; (2 * self.n - 1) as usize];
        self.row_collis = vec![0; self.n as usize];
        self.init_queens();

        for i in 0..self.n {
            let queen_row = self.queens[i as usize];

            let main_diag_index = i - queen_row + self.n - 1;
            self.main_diag_collis[main_diag_index as usize] += 1;

            let secondary_diag_index = i + queen_row;
            self.secondary_diag_collis[secondary_diag_index as usize] += 1;

            self.row_collis[queen_row as usize] += 1;
        }
    }

    fn init_queens(&mut self) {
        self.queens = vec![];

        for i in 0..self.n {
            self.queens.push(self.get_min_row_for_queen(i as usize));
        }
    }

    fn get_queen_main_diag_index(&self, queen_idx: usize) -> usize {
        self.get_pos_main_diag_index(queen_idx, self.queens[queen_idx] as usize)
    }

    fn get_queen_secondary_diag_index(&self, queen_idx: usize) -> usize {
        self.get_pos_secondary_diag_index(queen_idx, self.queens[queen_idx] as usize)
    }

    fn get_pos_main_diag_index(&self, col_idx: usize, row_idx: usize) -> usize {
        col_idx - row_idx + (self.n - 1) as usize
    }

    fn get_pos_secondary_diag_index(&self, col_idx: usize, row_idx: usize) -> usize {
        col_idx + row_idx
    }

    fn get_pos_collisions(&self, col_idx: usize, row_idx: u32) -> u32 {
        self.main_diag_collis[self.get_pos_main_diag_index(col_idx, row_idx as usize)]
            + self.secondary_diag_collis
                [self.get_pos_secondary_diag_index(col_idx, row_idx as usize)]
            + self.row_collis[row_idx as usize]
    }

    fn get_queen_collisions(&self, queen_idx: usize) -> u32 {
        self.get_pos_collisions(queen_idx, self.queens[queen_idx])
    }

    fn get_all_max_queens(&self) -> Vec<usize> {
        let mut max_val = None;
        let mut max_queens = vec![];

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
        }

        max_queens
    }

    fn get_max_queen(&self) -> usize {
        *self
            .get_all_max_queens()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }

    fn change_collision_for_queen(&mut self, idx: usize, val: i32) {
        let main_diag_idx = self.get_queen_main_diag_index(idx);
        self.main_diag_collis[main_diag_idx] =
            (self.main_diag_collis[main_diag_idx] as i32 + val) as u32;

        let secondary_diag_idx = self.get_queen_secondary_diag_index(idx);
        self.secondary_diag_collis[secondary_diag_idx] =
            (self.secondary_diag_collis[secondary_diag_idx] as i32 + val) as u32;

        self.row_collis[self.queens[idx] as usize] =
            (self.row_collis[self.queens[idx] as usize] as i32 + val) as u32;
    }

    fn move_queen(&mut self, queen_idx: usize, row_idx: u32) {
        self.change_collision_for_queen(queen_idx, -1);
        self.queens[queen_idx] = row_idx;
        self.change_collision_for_queen(queen_idx, 1);
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

        for i in 0..self.n {
            if self.row_collis[i as usize] > 1 {
                return false;
            }
        }

        true
    }

    fn get_min_row_for_queen(&self, queen_idx: usize) -> u32 {
        let mut min_row = 0;
        let mut min_row_val = None;

        for i in 0..self.n {
            let val = self.get_pos_collisions(queen_idx, i);

            if min_row_val.is_none() || min_row_val.unwrap() > val {
                min_row = i;
                min_row_val = Some(val);
            }
        }

        min_row
    }

    fn solve(&mut self) {
        let mut i: u32 = 0;

        loop {
            let max_queen = self.get_max_queen();
            let min_row = self.get_min_row_for_queen(max_queen);
            self.move_queen(max_queen, min_row);

            i += 1;

            if i >= 2 * self.n {
                self.init();
                self.solve();
            }

            if self.should_stop() {
                break;
            }
        }
    }

    pub fn to_pretty_string(&self) -> String {
        let mut res = String::new();

        for i in 0..self.n {
            for j in 0..self.n {
                if self.queens[i as usize] == j {
                    res.push('*');
                } else {
                    res.push('_');
                }

                res.push(' ');
            }

            res.push('\n');
        }

        res
    }
}

fn main() {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("Couldn't read from stdin");

    let n: u32 = buf.trim().parse().expect("Couldn't parse input to number");

    let mut board = Board::new(n);
    board.solve();

    dbg!(&board);

    // println!("{}", board.to_pretty_string());
}
