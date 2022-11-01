pub struct Cursor2d {
    pub x: usize,
    pub y: usize,
    pub contents: Vec<Vec<char>>,
}

impl Cursor2d {
    #[must_use]
    pub fn new(contents: Vec<Vec<char>>) -> Self {
        Self {
            x: 0,
            y: 0,
            contents,
        }
    }

    #[must_use]
    pub fn get(&self) -> char {
        self.contents[self.y][self.x]
    }

    #[must_use]
    pub fn get_next(&self) -> char {
        self.contents[self.y][self.x + 1]
    }

    #[must_use]
    pub fn get_next_vertical(&self) -> char {
        self.contents[self.y + 1][self.x]
    }

    #[must_use]
    pub fn get_prev(&self) -> char {
        self.contents[self.y][self.x - 1]
    }

    #[must_use]
    pub fn get_prev_vertical(&self) -> char {
        self.contents[self.y - 1][self.x]
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