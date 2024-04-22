use std::io::{stdout, Write};

use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::point::Point;
use crate::player::Player;

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = GRID_WIDTH / 2;

pub struct Matrix { pub data: [[i32; GRID_WIDTH]; GRID_HEIGHT] }

impl Matrix {
    pub fn new() -> Matrix {
        Matrix { data: [[0; GRID_WIDTH]; GRID_HEIGHT] }
    }

    pub fn update(&mut self, player: &Player, apple_pos: &Option<Point>) {
        // Reset grid to clear stale data
        self.data = Matrix::new().data;

        let apple = apple_pos.as_ref().unwrap();
        self.data[apple.y as usize][apple.x as usize] = 6;
        self.data[player.position.y as usize][player.position.x as usize] = 1;
        for pos in player.tail.iter() {
            self.data[pos.y as usize][pos.x as usize] = 2;
        }
    }

    pub fn print(&self) {
      // Figure out how to clear screen and return cursor to top of grid
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