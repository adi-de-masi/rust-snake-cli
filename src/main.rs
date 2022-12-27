use console::Term;
mod snake;
use crate::snake::Snake;

fn main() {
    let stdout = Term::buffered_stdout();
    let board = Board::new(30, 15);
    let mut snake = Snake::new((2, 2));

    loop {
        if let Ok(key) = stdout.read_key() {
            snake._move(key);
            repaint_board(&snake, &board);
        }
    }
}


struct Board {
    width: i32,
    height: i32,
}

impl Board {
    fn new(width: i32, height: i32) -> Self {
        Board { width, height }
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
