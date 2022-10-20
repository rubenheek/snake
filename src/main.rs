use std::collections::VecDeque;

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

const GRID_DIM: [usize; 2] = [30, 30];
const CELL_SIZE: f32 = 16.;
const TICK: f32 = 0.1;

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new();
    while game.try_update().is_ok() {
        game.draw();
        next_frame().await;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: VecDeque<[usize; 2]>,
    dir: Dir,
    stopwatch: f32,
    apple: [usize; 2],
}

impl Game {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        for i in 0..5 {
            snake.push_back([i, 0]);
        };
        Self {
            snake,
            dir: Dir::Right,
            apple: [10, 10],
            stopwatch: 0.,
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        draw_rectangle(0., 0., GRID_DIM[0] as f32 * CELL_SIZE, GRID_DIM[1] as f32 * CELL_SIZE, GREEN);
        for &[x, y] in self.snake.iter() {
            draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, RED);
        }
        draw_rectangle(self.apple[0] as f32 * CELL_SIZE, self.apple[1] as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, BLUE);
    }

    fn gen_apple(&mut self) {
        let mut rng = thread_rng();
        loop {
            let new_apple: [usize; 2] = [rng.gen_range(0..GRID_DIM[0]), rng.gen_range(0..GRID_DIM[1])];
            let overlaps = self.snake.iter().any(|&part| part == new_apple);
            if !overlaps {
                self.apple = new_apple;
                println!("{:?}", self.apple);
                break;
            }
        }
    }

    fn try_move(&mut self) -> Result<(), ()> {
        let &[x, y] = self.snake.back().unwrap();
        let next_head = match self.dir {
            Dir::Up if y > 1 => [x, y - 1],
            Dir::Down if y < GRID_DIM[1] - 1 => [x, y + 1],
            Dir::Right if x < GRID_DIM[0] - 1 => [x + 1, y],
            Dir::Left if x > 1 => [x - 1, y],
            _ => return Err(())
        };
        let overlaps = self.snake.iter().any(|&part| part == next_head);
        if overlaps {
            return Err(());
        }
        self.snake.push_back(next_head);
        if next_head == self.apple {
            self.gen_apple();
        } else {
            self.snake.pop_front();
        }
        Ok(())
    }

    fn try_update(&mut self) -> Result<(), ()> {
        if let Some(key) = get_last_key_pressed() {
            self.dir = match key {
                KeyCode::Up if self.dir != Dir::Down => Dir::Up,
                KeyCode::Down if self.dir != Dir::Up => Dir::Down,
                KeyCode::Left if self.dir != Dir::Right => Dir::Left,
                KeyCode::Right if self.dir != Dir::Left => Dir::Right,
                _ => self.dir,
            };
        }

        self.stopwatch += get_frame_time();
        if self.stopwatch > TICK {
            self.stopwatch = 0.;
            self.try_move()
        } else {
            Ok(())
        }
    }
}
