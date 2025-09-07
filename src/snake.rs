#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Snake {
            body: vec![(x, y)],
            dir: Direction::Right,
        }
    }

    pub fn head(&self) -> (i32, i32) {
        *self.body.first().unwrap()
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if self.dir == Direction::Up && dir == Direction::Down
            || self.dir == Direction::Down && dir == Direction::Up
            || self.dir == Direction::Left && dir == Direction::Right
            || self.dir == Direction::Right && dir == Direction::Left
        {
            return;
        }
        self.dir = dir;
    }

    pub fn move_forward(&mut self) {
        let (x, y) = self.head();
        let new_head = match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        self.body.insert(0, new_head);
        self.body.pop();
    }

    pub fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }

    pub fn body(&self) -> &Vec<(i32, i32)> {
        &self.body
    }
}
