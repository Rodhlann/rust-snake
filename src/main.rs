mod point;
mod matrix;
mod player;

use point::Point;
use matrix::{Matrix, GRID_HEIGHT, GRID_WIDTH};
use player::{Player, DIRS};

use std::{thread, time::Duration};
use rand::{thread_rng, Rng};

fn check_game_over(player: &Player) {
    let current_pos = &player.position;
    
    // Player has reached edge of grid
    if current_pos.x >= GRID_WIDTH as i32
        || current_pos.x < 0
        || current_pos.y >= GRID_HEIGHT as i32
        || current_pos.y < 0 
    {
        panic!("Game Over!")
    }

    // Player will intersect with their tail
    if player.tail.contains(&current_pos) {
        panic!("Game Over!")
    }
}

fn main() {
    let mut grid = Matrix::new();
    let mut apple_pos: Option<Point> = Some(Point { x: 1, y: 5 }); // TODO: replace with None after troubleshooting
    let mut player = Player {
        position: Point { x: 1, y: 1 },
        points: 0,
        tail: Vec::new()
    };

    loop {
        player.update(DIRS.0); // TODO: Figure out non-blocking user input for param here
        check_game_over(&player);

        let current_pos = &player.position;

        // TODO: Figure out how to break this out in to it's own function 
        // Check for apple intersection, iterate player points
        if !apple_pos.is_none() && apple_pos.as_ref().unwrap() == current_pos {
            player.points += 1;
            apple_pos = None;
        }
        if apple_pos.is_none() {
            while apple_pos.is_none() || apple_pos.as_ref().unwrap() == current_pos {
                apple_pos = Some(Point {
                    x: thread_rng().gen_range(0..matrix::GRID_WIDTH as i32),
                    y: thread_rng().gen_range(0..matrix::GRID_HEIGHT as i32)
                })
            }
        }     

        grid.update(&player, &apple_pos);
        grid.print();
        thread::sleep(Duration::from_millis(500));
    }
}
