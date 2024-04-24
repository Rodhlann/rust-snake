use std::io::{stdout, Write};

use crossterm::cursor::MoveToPreviousLine;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::player::Player;
use grid_point::Point;

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = GRID_WIDTH / 2;

pub struct Matrix {
    pub data: [[char; GRID_WIDTH]; GRID_HEIGHT],
}

const NAKE: [char; 4] = ['N', 'A', 'K', 'E'];

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            data: [[' '; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn update(&mut self, player: &Player, apple_pos: &Option<Point>) {
        // Reset grid to clear stale data
        self.data = Matrix::new().data;

        let apple = apple_pos.as_ref().unwrap();
        self.data[apple.y as usize][apple.x as usize] = '*';
        self.data[player.position.y as usize][player.position.x as usize] = 'S';
        for (i, pos) in player.tail.iter().enumerate() {
            let tail_len = player.tail.len();
            if tail_len > 4 {
                if i == 0 {
                    self.data[pos.y as usize][pos.x as usize] = NAKE[0];
                } else if i == tail_len - 2 {
                    self.data[pos.y as usize][pos.x as usize] = NAKE[2];
                } else if i == tail_len - 1 {
                    self.data[pos.y as usize][pos.x as usize] = NAKE[3];
                } else {
                    self.data[pos.y as usize][pos.x as usize] = NAKE[1];
                }
                continue;
            }
            self.data[pos.y as usize][pos.x as usize] = NAKE[i];
        }
    }

    pub fn print(&self) {
        // Clear screen and return to start of output
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveToPreviousLine(GRID_HEIGHT as u16)
        )
        .expect("Failed to clear content");
        for _ in 0..GRID_WIDTH + 2 {
            print!("-");
        }
        print!("\r\n");
        for (y, row) in self.data.iter().enumerate() {
            print!("|");
            for (x, _col) in row.iter().enumerate() {
                print!("{}", &self.data[y][x]);
            }
            print!("|\r\n");
        }
        for _ in 0..GRID_WIDTH + 2 {
            print!("-");
        }
        print!("\r\n");
        stdout().flush().unwrap(); // Flush stdout to ensure output is printed immediately
    }
}
