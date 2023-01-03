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

    pub fn paint_next(&self, snake: &Snake) {
        clear();
        println!("food: w={}/h={}", self.food.width, self.food.height);
        self.print_horizontal_border();
        let mut height = 0;
        while height < self.height {
            let mut width = 0;
            print!("X");
            while width < self.width {
                if snake.is_occupied(Coords { width, height }) {
                    print!("*");
                } else if self.is_food(Coords { width, height}) {
                    print!("o");
                } else {
                    print!(" ");
                }
                width = width + 1;
            }
            print!("X\n");
            height = height + 1;
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

    pub fn is_food(&self, coords: Coords) -> bool {
        return self.food.width == coords.width && self.food.height == coords.height;
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}
