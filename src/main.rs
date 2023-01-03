mod board;
mod snake;
use crate::board::Board;
use console::Term;
use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct Coords {
    pub width: i32,
    pub height: i32,
}
const BOARD_COORDS: Coords = Coords {
    width: 30,
    height: 15,
};

fn main() -> Result<(), CollisionError> {
    let stdout = Term::buffered_stdout();
    let mut board = Board::new(BOARD_COORDS);

    loop {
        if let Ok(key) = stdout.read_key() {
            board = board.next(&key)?;
        }
    }
}

#[derive(Debug)]
pub struct CollisionError {
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

