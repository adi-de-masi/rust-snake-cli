use console::Key;

pub struct Snake {
    pub coords: Vec<(i32, i32)>,
}

impl Snake {
    pub fn new(start_pos: (i32, i32)) -> Self {
        return Snake {
            coords: vec![(start_pos)],
        };
    }
    pub fn move_to(&self, key: &Key) -> Snake {
        let mut snake_coords = self.coords.clone();
        let next = self.next_coord(key);
        match next {
            Some(next) => {
                snake_coords.push(next);
                snake_coords.remove(0);
            }
            None => (),
        }
        return Snake {
            coords: snake_coords,
        };
    }
    pub fn next_coord(&self, direction: &Key) -> Option<(i32, i32)> {
        let last = self.coords.get(self.coords.len() - 1);
        match last {
            Some(last) => {
                match direction {
                    Key::ArrowLeft => {
                        return Some((last.0, last.1 - 1));
                    }
                    Key::ArrowRight => {
                        return Some((last.0, last.1 + 1));
                    }
                    Key::ArrowUp => {
                        return Some((last.0 - 1, last.1));
                    }
                    Key::ArrowDown => {
                        return Some((last.0 + 1, last.1));
                    }
                    _ => return None,
                }
            }
            None => {
                return None
            }
        }
    }

    pub fn is_occupied(&self, pos: (i32, i32)) -> bool {
        let test = self
            .coords
            .iter()
            .find(|&current_coord| current_coord.0 == pos.0 && current_coord.1 == pos.1);
        return test != None;
    }
}

#[cfg(test)]
mod tests {
    use console::Key;
    #[test]
    fn it_constructs() {
        let snake = crate::snake::Snake::new((2, 2));
        assert_eq!((2, 2), snake.coords[0]);
    }
    #[test]
    fn it_moves_left() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake = snake.move_to(&Key::ArrowLeft);
        assert_eq!(vec!((2, 1)), snake.coords);
    }
    #[test]
    fn it_moves_right() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake = snake.move_to(&Key::ArrowRight);
        assert_eq!(vec!((2, 3)), snake.coords);
    }
    #[test]
    fn it_moves_up() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake = snake.move_to(&Key::ArrowUp);
        assert_eq!(vec!((1, 2)), snake.coords);
    }
    #[test]
    fn it_moves_down() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake = snake.move_to(&Key::ArrowDown);
        assert_eq!(vec!((3, 2)), snake.coords);
    }
    #[test]
    fn is_occupied() {
        let snake = crate::snake::Snake::new((2, 2));
        assert!(snake.is_occupied((2, 2)));
    }
    #[test]
    fn is_not_occupied() {
        let snake = crate::snake::Snake::new((2, 2));
        assert_eq!(false, snake.is_occupied((1, 1)));
    }
}
