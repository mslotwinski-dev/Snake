use crate::snake::Snake;
use rand::Rng;

pub struct Board {
    width: i32,
    height: i32,
    snake: Snake,
    food: Vec<(i32, i32)>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let mut board = Board {
            width,
            height,
            snake: Snake::new(width / 2, height / 2),
            food: vec![],
        };
        for _ in 0..5 {
            board.spawn_food();
        }
        board
    }

    pub fn spawn_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            if !self.snake.body().contains(&(x, y)) {
                self.food.push((x, y));
                break;
            }
        }
    }

    pub fn update(&mut self) -> bool {
        self.snake.move_forward();
        let head = self.snake.head();

        if head.0 < 0 || head.0 >= self.width || head.1 < 0 || head.1 >= self.height {
            return false;
        }

        if self.snake.body()[1..].contains(&head) {
            return false;
        }

        for i in 0..self.food.len() {
            if head == self.food[i] {
                self.snake.grow();
                self.food.remove(i);
                self.spawn_food();
                break;
            }
        }

        if self.food.is_empty() {
            println!("🏆 Wygrałeś! Plansza cała zajęta.");
            return false;
        }

        true
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn snake_mut(&mut self) -> &mut Snake {
        &mut self.snake
    }

    pub fn food(&self) -> Vec<(i32, i32)> {
        self.food.clone()
    }
}
