use console::{Key, Term};
use std::fmt;
use std::fmt::Debug;
mod snake;
use crate::snake::Snake;

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
                    repaint_board(&snake, &board);
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

struct Board {
    width: i32,
    height: i32,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        Board { width, height }
    }
    fn is_border(&self, pos: (i32, i32)) -> bool {
        return self.height == pos.0 || self.width == pos.1 || pos.0 == 0 || pos.1 == 0;
    }
}

fn repaint_board(snake: &Snake, board: &Board) {
    clear_board();
    print_horizontal_border(board);
    let mut i = 0;
    while i < board.height {
        let mut j = 0;
        print!("X");
        while j < board.width {
            if snake.is_occupied((i, j)) {
                print!("*");
            } else {
                print!(" ");
            }
            j = j + 1;
        }
        print!("X\n");
        i = i + 1;
    }
    print_horizontal_border(board);
}

fn clear_board() {
    print!("{}[2J", 27 as char);
}

fn print_horizontal_border(board: &Board) {
    print!(" ");
    (0..board.width).for_each(|_| {
        print!("X");
    });
    print!("\n");
}
