use crate::Coords;
use console::Key;

pub struct Snake {
    pub coords: Vec<Coords>,
}

impl Snake {
    pub fn new(start_pos: Coords) -> Self {
        return Snake {
            coords: vec![(start_pos)],
        };
    }
    pub fn move_to(&self, key: &Key, is_food: bool) -> Snake {
        let mut snake_coords = self.coords.clone();
        let next = self.next_coord(key);
        match next {
            Some(next) => {
                snake_coords.push(next);
                if !is_food {
                    snake_coords.remove(0);
                };
            }
            None => (),
        }
        return Snake {
            coords: snake_coords,
        };
    }
    pub fn next_coord(&self, direction: &Key) -> Option<Coords> {
        let last = self.coords.get(self.coords.len() - 1);
        match last {
            Some(last) => match direction {
                Key::ArrowLeft => {
                    return Some(Coords {
                        width: last.width - 1,
                        height: last.height,
                    });
                }
                Key::ArrowRight => {
                    return Some(Coords {
                        width: last.width + 1,
                        height: last.height,
                    });
                }
                Key::ArrowUp => {
                    return Some(Coords {
                        width: last.width,
                        height: last.height - 1,
                    });
                }
                Key::ArrowDown => {
                    return Some(Coords {
                        width: last.width,
                        height: last.height + 1,
                    });
                }
                _ => return None,
            },
            None => return None,
        }
    }

    pub fn is_occupied(&self, coords: Coords) -> bool {
        let test = self
            .coords
            .iter()
            .find(|&current_coord| current_coord.height == coords.height && current_coord.width == coords.width);
        return test != None;
    }
}

#[cfg(test)]
mod tests {
    use crate::Coords;
    use console::Key;
    #[test]
    fn it_constructs() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let snake = crate::snake::Snake::new(coords);
        assert_eq!(vec!(coords), snake.coords);
    }
    #[test]
    fn it_moves_left() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let mut snake = crate::snake::Snake::new(coords);
        snake = snake.move_to(&Key::ArrowLeft, false);
        assert_eq!(
            vec!(Coords {
                width: 1,
                height: 2
            }),
            snake.coords
        );
    }
    #[test]
    fn it_moves_right() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let mut snake = crate::snake::Snake::new(coords);
        snake = snake.move_to(&Key::ArrowRight, false);
        assert_eq!(
            vec!(Coords {
                width: 3,
                height: 2
            }),
            snake.coords
        );
    }
    #[test]
    fn it_moves_up() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let mut snake = crate::snake::Snake::new(coords);
        snake = snake.move_to(&Key::ArrowUp, false);
        assert_eq!(
            vec!(Coords {
                width: 2,
                height: 1
            }),
            snake.coords
        );
    }
    #[test]
    fn it_moves_down() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let mut snake = crate::snake::Snake::new(coords);
        snake = snake.move_to(&Key::ArrowDown, false);
        assert_eq!(
            vec!(Coords {
                width: 2,
                height: 3
            }),
            snake.coords
        );
    }
    #[test]
    fn it_grows() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let mut snake = crate::snake::Snake::new(coords);
        snake = snake.move_to(&Key::ArrowDown, true);
        assert_eq!(
            vec!(
                Coords {
                    width: 2,
                    height: 2
                },
                Coords {
                    width: 2,
                    height: 3
                }
            ),
            snake.coords
        );
    }
    #[test]
    fn is_occupied() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let snake = crate::snake::Snake::new(coords);
        assert!(snake.is_occupied(coords));
    }
    #[test]
    fn is_not_occupied() {
        let coords = Coords {
            width: 2,
            height: 2,
        };
        let snake = crate::snake::Snake::new(coords);
        assert_eq!(false, snake.is_occupied(Coords {width: 1, height: 1}));
    }
}
