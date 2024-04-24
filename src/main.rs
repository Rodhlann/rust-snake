mod matrix;
mod player;
mod point;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use matrix::{Matrix, GRID_HEIGHT, GRID_WIDTH};
use player::{Player, DIRS};
use point::Point;

use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn check_game_over(player: &Player) -> bool {
    let current_pos = &player.position;
    // Player has reached edge of grid
    if current_pos.x >= GRID_WIDTH as i32
        || current_pos.x < 0
        || current_pos.y >= GRID_HEIGHT as i32
        || current_pos.y < 0
    {
        return true;
    }
    // Player will intersect with their tail
    if player.tail.contains(current_pos) {
        return true;
    }
    false
}

fn check_for_user_input() -> Option<char> {
    if let Ok(true) = poll(Duration::from_millis(50)) {
        // Read the event
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = read()
        {
            match code {
                KeyCode::Char(c) => {
                    if c == 'c' && modifiers == KeyModifiers::CONTROL {
                        return Some(c);
                    }
                    return Some(c);
                }
                _ => {
                    return None;
                }
            }
        }
    }
    None
}

fn main() {
    let mut dir = DIRS.1;
    let mut grid = Matrix::new();
    let mut apple_pos: Option<Point> = None;
    let mut player = Player {
        position: Point { x: 1, y: 1 },
        points: 0,
        new_point: false,
        tail: Vec::new(),
    };

    // Enable terminal raw mode
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    loop {
        match check_for_user_input() {
            // Char c is only returned in the case of CTRL+C input, kill program
            Some('c') => {
                break;
            }
            Some(char) => {
                match char {
                    'w' => dir = DIRS.0,
                    'd' => dir = DIRS.1,
                    's' => dir = DIRS.2,
                    'a' => dir = DIRS.3,
                    _ => (),
                };
            }
            None => (),
        }

        player.update(&dir);

        if check_game_over(&player) {
            print!("Game Over!!");
            break;
        }

        let current_pos = &player.position;

        // Check for apple intersection, iterate player points
        if let Some(pos) = &apple_pos {
            if pos == current_pos {
                player.points += 1;
                player.new_point = true;
                apple_pos = None;
            }
        }

        if apple_pos.is_none() {
            // Set new apple_pos if apple_pos == None, or if the player's current
            // position is the same as the new apple_pos
            while apple_pos.is_none()
                || apple_pos.as_ref().unwrap() == current_pos
                || player.tail.contains(apple_pos.as_ref().unwrap())
            {
                apple_pos = Some(Point {
                    x: thread_rng().gen_range(0..matrix::GRID_WIDTH as i32),
                    y: thread_rng().gen_range(0..matrix::GRID_HEIGHT as i32),
                })
            }
        }

        grid.update(&player, &apple_pos);
        grid.print();

        thread::sleep(Duration::from_millis(300));
    }

    // Return terminal to original state
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    stdout().flush().unwrap()
}
