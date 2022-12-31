mod snake;
mod board;
use console::{Key, Term};
use std::fmt;
use std::fmt::Debug;
use crate::snake::Snake;
use crate::board::Board;

fn main() {
    let stdout = Term::buffered_stdout();
    let board = Board::new(30, 15);
    let mut snake = Snake::new((2, 2));

    loop {
        if let Ok(key) = stdout.read_key() {
            let collisions = assert_no_collision(&snake, &board, &key);
            match collisions {
                Ok(()) => {
                    snake = snake.move_to(&key);
                    board.repaint(&snake);
                }
                Err(collision_error) => {
                    println!(
                        "collosion occurred at {}/{}",
                        collision_error.coords.0, collision_error.coords.1
                    );
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct CollisionError {
    coords: (i32, i32),
}

impl fmt::Display for CollisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.coords.0, self.coords.1)
    }
}

fn assert_no_collision(snake: &Snake, board: &Board, key: &Key) -> Result<(), CollisionError> {
    let next_pos = snake.next_coord(key);
    match next_pos {
        Some(next_pos) => {
            if snake.is_occupied(next_pos) || board.is_border(next_pos) {
                return Err(CollisionError { coords: next_pos });
            }
        }
        None => (),
    }
    return Ok(());
}

