use crate::snake::Snake;
use crate::Coords;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub food: Coords,
}

impl Board {
    pub fn new(dimensions: Coords, food: Option<Coords>) -> Self {
        Board {
            width: dimensions.width,
            height: dimensions.height,
            food: food.unwrap_or(Coords {
                width: 0,
                height: 0,
            }),
        }
    }

    pub fn is_border(&self, pos: Coords) -> bool {
        return self.height == pos.height
            || self.width == pos.width
            || pos.width == 0
            || pos.height == 0;
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
