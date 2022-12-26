use std::{error::Error, thread, time};

use console::{Key, Term};

fn main() {
    let stdout = Term::buffered_stdout();
    let delay = time::Duration::from_secs(1);
    let board = Board::new(30, 15, 3);
    let mut snake = vec![(2, 2), (2, 3)];

    'game_loop: loop {
        if let Ok(key) = stdout.read_key() {
            snake = moveSnake(snake, key);
            repaint_board(&board, &snake);
            // thread::sleep(delay);
            // clear cli
            print!("{}[2J", 27 as char);
        }
    }
}

fn moveSnake(mut snake: Vec<(i32, i32)>, key: Key) -> Vec<(i32, i32)> {
    match key {
        Key::ArrowLeft => {
            if let Some(last) = snake.pop() {
                snake.push(last);
                snake.push((last.0, last.1 - 1));
            }
        }
        Key::ArrowRight => {
            if let Some(last) = snake.pop() {
                snake.push(last);
                snake.push((last.0, last.1 + 1));
            }
        }
        Key::ArrowUp => {
            if let Some(last) = snake.pop() {
                snake.push(last);
                snake.push((last.0 - 1, last.1));
            }
        }
        Key::ArrowDown => {
            if let Some(last) = snake.pop() {
                snake.push(last);
                snake.push((last.0 + 1, last.1));
            }
        }
        _ => (),
    }
    return snake;
}

struct Board {
    width: i32,
    height: i32,
    square_size: i32,
}

impl Board {
    fn new(width: i32, height: i32, square_size: i32) -> Self {
        Board {
            width,
            height,
            square_size,
        }
    }
}

fn repaint_board(board: &Board, snake: &Vec<(i32, i32)>) {
    print_horizontal_border(board);
    let mut i = 0;
    while i < board.height {
        let mut j = 0;
        print!("X");
        while j < board.width {
            if is_snake(&snake, (i, j)) {
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

fn print_horizontal_border(board: &Board) {
    print!(" ");
    (0..board.width).for_each(|_| {
        print!("X");
    });
    print!("\n");
}

fn is_snake(snake: &Vec<(i32, i32)>, coord: (i32, i32)) -> bool {
    let test = snake
        .iter()
        .find(|&current_coord| current_coord.0 == coord.0 && current_coord.1 == coord.1);
    return test != None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_snake_true() {
        let snake = vec![(0, 0), (1, 1), (1, 2)];
        let coord = (1, 1);
        assert_eq!(true, super::is_snake(&snake, coord));
    }

    #[test]
    fn is_snake_false() {
        let snake = vec![(0, 0), (1, 1), (1, 2)];
        let coord = (3, 1);
        assert_eq!(false, super::is_snake(&snake, coord));
    }
}
