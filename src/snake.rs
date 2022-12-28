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
    pub fn _move(&mut self, key: Key) {
        let next = self.next_coord(key);
        match next {
            Some(next) => self.coords.push(next),
            None => (),
        }
    }
    pub fn next_coord(&mut self, direction: Key) -> Option<(i32, i32)> {
        if let Some(last) = self.coords.pop() {
            self.coords.push(last);
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
        return None;
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
        snake._move(Key::ArrowLeft);
        assert_eq!(vec!((2, 2), (2, 1)), snake.coords);
    }
    #[test]
    fn it_moves_right() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake._move(Key::ArrowRight);
        assert_eq!(vec!((2, 2), (2, 3)), snake.coords);
    }
    #[test]
    fn it_moves_up() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake._move(Key::ArrowUp);
        assert_eq!(vec!((2, 2), (1, 2)), snake.coords);
    }
    #[test]
    fn it_moves_down() {
        let mut snake = crate::snake::Snake::new((2, 2));
        snake._move(Key::ArrowDown);
        assert_eq!(vec!((2, 2), (3, 2)), snake.coords);
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
