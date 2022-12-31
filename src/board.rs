use crate::snake::Snake;

pub struct Board {
    width: i32,
    height: i32,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Board { width, height }
    }
    pub fn is_border(&self, pos: (i32, i32)) -> bool {
        return self.height == pos.0 || self.width == pos.1 || pos.0 == 0 || pos.1 == 0;
    }
    pub fn repaint(&self, snake: &Snake) {
        clear();
        self.print_horizontal_border();
        let mut i = 0;
        while i < self.height {
            let mut j = 0;
            print!("X");
            while j < self.width {
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
        self.print_horizontal_border();
    }
    fn print_horizontal_border(&self) {
        print!(" ");
        (0..self.width).for_each(|_| {
            print!("X");
        });
        print!("\n");
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}
