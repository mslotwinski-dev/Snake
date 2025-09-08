use crate::{assets, board::Board, snake::Direction};
use macroquad::prelude::*;

const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct Game {
    board: Board,
    timer: f32,
    tick_time: f32,
    state: GameState,
    magic_font: Font,
    font: Font,
}

impl Game {
    pub async fn new() -> Self {
        let (magic_font, font) = assets::load_fonts().await;
        Self {
            board: Board::new(WIDTH, HEIGHT),
            timer: 0.0,
            tick_time: 0.15,
            state: GameState::Menu,
            magic_font,
            font,
        }
    }

    fn display_menu(&mut self) {
        let title = "SNAKE";
        let title_params = TextParams {
            font: Some(&self.magic_font),
            font_size: 80,
            color: RED,
            ..Default::default()
        };
        let dim = measure_text(title, title_params.font, title_params.font_size, 1.0);
        draw_text_ex(
            title,
            screen_width() / 2.0 - dim.width / 2.0,
            200.0,
            title_params,
        );

        // SUBTITLE
        let subtitle = "Nacisnij [SPACJA], aby zagrac";
        let subtitle_params = TextParams {
            font: Some(&self.font),
            font_size: 45,
            color: BLACK,
            ..Default::default()
        };
        let dim2 = measure_text(
            subtitle,
            subtitle_params.font,
            subtitle_params.font_size,
            1.0,
        );
        draw_text_ex(
            subtitle,
            screen_width() / 2.0 - dim2.width / 2.0,
            300.0,
            subtitle_params,
        );

        // INSTRUCTION
        let instruction = "Sterowanie: Strzalki lub WSAD";
        let instruction_params = TextParams {
            font: Some(&self.font),
            font_size: 30,
            color: DARKGRAY,
            ..Default::default()
        };
        let dim3 = measure_text(
            instruction,
            instruction_params.font,
            instruction_params.font_size,
            1.0,
        );
        draw_text_ex(
            instruction,
            screen_width() / 2.0 - dim3.width / 2.0,
            400.0,
            instruction_params,
        );

        if is_key_pressed(KeyCode::Space) {
            self.board = Board::new(WIDTH, HEIGHT);
            self.state = GameState::Playing;
        }
    }

    fn display_game_over(&mut self) {
        let text = "GAME OVER";
        let size = 120;
        let dim = measure_text(text, Some(&self.font), size, 1.0);
        draw_text_ex(
            text,
            screen_width() / 2.0 - dim.width / 2.0,
            200.0,
            TextParams {
                font: Some(&self.font),
                font_size: size,
                color: RED,
                ..Default::default()
            },
        );

        let text = &format!("Wynik: {}", self.board.snake().body().len() - 1);
        let size = 50;
        let dim = measure_text(text, Some(&self.font), size, 1.0);
        draw_text_ex(
            text,
            screen_width() / 2.0 - dim.width / 2.0,
            300.0,
            TextParams {
                font: Some(&self.font),
                font_size: size,
                color: BLACK,
                ..Default::default()
            },
        );

        let text = "Nacisnij [R], aby zagrac od nowa";
        let size = 40;
        let dim = measure_text(text, Some(&self.font), size, 1.0);
        draw_text_ex(
            text,
            screen_width() / 2.0 - dim.width / 2.0,
            400.0,
            TextParams {
                font: Some(&self.font),
                font_size: size,
                color: SKYBLUE,
                ..Default::default()
            },
        );

        if is_key_pressed(KeyCode::R) {
            self.board = Board::new(WIDTH, HEIGHT);
            self.state = GameState::Playing;
        }
    }

    async fn play(&mut self) {
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.board.snake_mut().set_direction(Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.board.snake_mut().set_direction(Direction::Down);
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.board.snake_mut().set_direction(Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.board.snake_mut().set_direction(Direction::Right);
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Menu;
        }

        self.timer += get_frame_time();

        if self.timer >= self.tick_time {
            self.timer -= self.tick_time;
            let alive = self.board.update();

            if !alive {
                self.state = GameState::GameOver;
            }
        }

        let progress = self.timer / self.tick_time;

        self.render(progress);
    }

    pub fn render(&mut self, progress: f32) {
        let x0 = screen_width() / 2.0 - 10.5 * WIDTH as f32;
        let y0 = screen_height() / 2.0 - 10.5 * HEIGHT as f32;

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                draw_rectangle(
                    x0 + 21.0 * x as f32,
                    y0 + 21.0 * y as f32,
                    20.0,
                    20.0,
                    LIGHTGRAY,
                );
            }
        }

        for (fx, fy) in self.board.food() {
            draw_circle(
                x0 + 21.0 * fx as f32 + 10.0,
                y0 + 21.0 * fy as f32 + 10.0,
                8.0,
                YELLOW,
            );
        }

        let body = self.board.snake().body();
        let prev = self.board.snake().prev_body();

        for i in 0..body.len() - 1 {
            let (x1, y1) = prev[i];
            let (x2, y2) = prev[i + 1];
            let (nx1, ny1) = body[i];
            let (nx2, ny2) = body[i + 1];

            draw_line(
                x0 + 21.0 * interp(x1, nx1, progress) + 10.0,
                y0 + 21.0 * interp(y1, ny1, progress) + 10.0,
                x0 + 21.0 * interp(x2, nx2, progress) + 10.0,
                y0 + 21.0 * interp(y2, ny2, progress) + 10.0,
                20.0,
                PINK,
            );
        }

        for i in 0..body.len() {
            let (x0_seg, y0_seg) = prev[i];
            let (x1_seg, y1_seg) = body[i];
            draw_circle(
                x0 + 21.0 * interp(x0_seg, x1_seg, progress) + 10.0,
                y0 + 21.0 * interp(y0_seg, y1_seg, progress) + 10.0,
                10.0,
                PINK,
            );
        }

        let (hx, hy) = self.board.snake().head();

        let (dx, dy) = match self.board.snake().dir() {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        };

        let fx = x0 + 21.0 * (hx as f32 + dx * (progress - 1.0));
        let fy = y0 + 21.0 * (hy as f32 + dy * (progress - 1.0));

        draw_circle(fx + 6.0, fy + 6.0, 4.0, WHITE);
        draw_circle(fx + 14.0, fy + 14.0, 4.0, WHITE);
        draw_circle(fx + 6.0, fy + 6.0, 2.0, BLACK);
        draw_circle(fx + 14.0, fy + 14.0, 2.0, BLACK);

        draw_text_ex(
            &format!("Wynik: {}", self.board.snake().body().len() - 1),
            60.0,
            60.0,
            TextParams {
                font: Some(&self.font),
                font_size: 50,
                color: BLACK,
                ..Default::default()
            },
        );
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(WHITE);

            match self.state {
                GameState::Menu => {
                    self.display_menu();
                }

                GameState::Playing => {
                    self.play().await;
                }

                GameState::GameOver => {
                    self.display_game_over();
                }
            }

            next_frame().await;
        }
    }
}

fn interp(a: i32, b: i32, t: f32) -> f32 {
    a as f32 * (1.0 - t) + b as f32 * t
}
