pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Cursor2d {
    pub x: usize,
    pub y: usize,
    pub contents: Vec<Vec<char>>,
    pub direction: Direction,
}

impl Cursor2d {
    #[must_use]
    pub fn new(contents: Vec<Vec<char>>) -> Self {
        Self {
            x: 0,
            y: 0,
            contents,
            direction: Direction::Right,
        }
    }

    #[must_use]
    pub fn get(&self) -> char {
        self.contents[self.y][self.x]
    }

    #[must_use]
    pub fn get_next(&self) -> char {
        match self.direction {
            Direction::Up => self.contents[self.y - 1][self.x],
            Direction::Down => self.contents[self.y + 1][self.x],
            Direction::Left => self.contents[self.y][self.x - 1],
            Direction::Right => self.contents[self.y][self.x + 1],
        }
    }

    #[must_use]
    pub fn get_prev(&self) -> char {
        match self.direction {
            Direction::Up => self.contents[self.y + 1][self.x],
            Direction::Down => self.contents[self.y - 1][self.x],
            Direction::Left => self.contents[self.y][self.x + 1],
            Direction::Right => self.contents[self.y][self.x - 1],
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn move_backward(&mut self) {
        match self.direction {
            Direction::Up => self.move_down(),
            Direction::Down => self.move_up(),
            Direction::Left => self.move_right(),
            Direction::Right => self.move_left(),
        }
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    pub fn turn_around(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Up,
            Direction::Left => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Left,
        }
    }

    pub fn turn_to(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn move_to_next(&mut self) {
        self.x += 1;
    }

    pub fn move_to_prev(&mut self) {
        self.x -= 1;
    }

    pub fn move_to_next_vertical(&mut self) {
        self.y += 1;
    }

    pub fn move_to_prev_vertical(&mut self) {
        self.y -= 1;
    }
}

impl Iterator for Cursor2d {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.contents[self.y].len() - 1 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        if self.y == self.contents.len() {
            None
        } else {
            Some(self.contents[self.y][self.x])
        }
    }
}