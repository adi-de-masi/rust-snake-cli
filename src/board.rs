use crate::snake::Snake;
use crate::CollisionError;
use crate::Coords;
use console::Key;
use rand::Rng;

pub struct Board {
    pub width: i32,
    pub height: i32,
    food: Coords,
    snake: Snake,
}

impl Board {
    pub fn new(dimensions: Coords) -> Self {
        Board {
            width: dimensions.width,
            height: dimensions.height,
            food: random_field_in_borders(dimensions),
            snake: Snake::new(Coords {
                width: 1,
                height: 1,
            }),
        }
    }

    pub fn next(mut self, key: &Key) -> Result<Board, CollisionError> {
        self.assert_no_collision(&key)?;
        if let Some(next_coords) = self.snake.next_coord(&key) {
            if self.is_food(next_coords) {
                self.snake = self.snake.move_to(&key, true);
                self.food = self.next_food();
            } else {
                self.snake = self.snake.move_to(&key, false);
            }
        }
        self.paint();
        return Ok(self);
    }

    fn next_food(&self) -> Coords {
        let borders = Coords {
            width: self.width,
            height: self.height,
        };
        let mut food_field = random_field_in_borders(borders);
        while self.snake.is_occupied(food_field) {
            food_field = random_field_in_borders(borders);
        }
        return food_field;
    }

    fn assert_no_collision(&self, key: &Key) -> Result<(), CollisionError> {
        let next_pos = self.snake.next_coord(key);
        match next_pos {
            Some(next_pos) => {
                if self.snake.is_occupied(next_pos) {
                    return Err(CollisionError::new(
                        next_pos,
                        String::from("You ate yourself"),
                    ));
                } else if self.is_border(next_pos) {
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

    fn is_border(&self, pos: Coords) -> bool {
        return self.height == pos.height
            || self.width == pos.width
            || pos.width == 0
            || pos.height == 0;
    }

    fn paint(&self) {
        clear();
        println!("food: w={}/h={}", self.food.width, self.food.height);
        self.print_horizontal_border();
        let mut height = 0;
        while height < self.height {
            let mut width = 0;
            print!("X");
            while width < self.width {
                if self.snake.is_occupied(Coords { width, height }) {
                    print!("*");
                } else if self.is_food(Coords { width, height }) {
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

fn random_field_in_borders(dimensions: Coords) -> Coords {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..(dimensions.height - 1));
    let y = rng.gen_range(1..(dimensions.width - 1));
    return Coords {
        width: y,
        height: x,
    };
}
