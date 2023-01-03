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
                    if let Some(next_coords) = snake.next_coord(&key) {
                        if board.is_food(next_coords) {
                            board = next_board(&board, &snake);
                            snake = snake.move_to(&key, true);
                        } else {
                            snake = snake.move_to(&key, false);
                        }
                    }
                    board.paint_next(&snake);
                }
                Err(collision_error) => {
                    println!(
                        "collosion occurred at {}/{}",
                        collision_error.coords.width, collision_error.coords.height
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

fn next_board(board: &Board, snake: &Snake) -> Board {
    let mut food_field = random_field_in_borders(board.height, board.width);
    while snake.is_occupied(food_field) {
        food_field = random_field_in_borders(board.height, board.width);
    }
    return Board {
        width: board.width,
        height: board.height,
        food: food_field,
    };
}

fn random_field_in_borders(width: i32, height: i32) -> Coords {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..(height - 1));
    let y = rng.gen_range(1..(width - 1));
    return Coords {
        width: x,
        height: y,
    };
}
