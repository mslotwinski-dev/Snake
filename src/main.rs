mod board;
mod snake;

use crate::board::Board;
use crate::snake::Direction;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute};
use std::{thread, time};

fn main() {
    let mut board = Board::new(30, 30);

    let _ = terminal::enable_raw_mode();

    loop {
        if event::poll(time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => board.snake_mut().set_direction(Direction::Up),
                    KeyCode::Down => board.snake_mut().set_direction(Direction::Down),
                    KeyCode::Left => board.snake_mut().set_direction(Direction::Left),
                    KeyCode::Right => board.snake_mut().set_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        let alive = board.update();
        if !alive {
            execute!(
                std::io::stdout(),
                terminal::Clear(ClearType::All),
                cursor::MoveTo(0, 0)
            )
            .unwrap();
            println!("💀 Game Over! Wynik: {}", board.snake().body().len() - 1);
            break;
        }

        execute!(
            std::io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
        board.render();

        thread::sleep(time::Duration::from_millis(200));
    }

    let _ = terminal::disable_raw_mode();
}
