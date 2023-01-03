mod board;
mod snake;
use crate::board::Board;
use crate::snake::Snake;
use console::{Key, Term};
use rand::Rng;
use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Coords {
    pub width: i32,
    pub height: i32,
}

fn main() {
    let stdout = Term::buffered_stdout();
    const WIDTH: i32 = 30;
    const HEIGHT: i32 = 15;
    let first_food_field = random_field_in_borders(WIDTH, HEIGHT);
    let mut board = Board::new(
        Coords {
            width: WIDTH,
            height: HEIGHT,
        },
        Some(first_food_field),
    );
    let mut snake = Snake::new(Coords {
        width: 1,
        height: 1,
    });

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
    coords: Coords,
    message: String,
}

impl CollisionError {
    fn new(coords: Coords, message: String) -> Self {
        return CollisionError { coords, message };
    }
}

impl fmt::Display for CollisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "width: {}, height: {}, message: {}",
            self.coords.width, self.coords.height, self.message
        )
    }
}

fn assert_no_collision(snake: &Snake, board: &Board, key: &Key) -> Result<(), CollisionError> {
    let next_pos = snake.next_coord(key);
    match next_pos {
        Some(next_pos) => {
            if snake.is_occupied(next_pos) {
                return Err(CollisionError::new(
                    next_pos,
                    String::from("You ate yourself"),
                ));
            } else if board.is_border(next_pos) {
                return Err(CollisionError::new(
                    next_pos,
                    String::from("You hit the border"),
                ));
            }
        }
        None => (),
    }
    return Ok(());
}

