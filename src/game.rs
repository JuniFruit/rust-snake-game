use crate::core::{Direction, Snake};
use crate::draw::{draw_block, draw_rect};
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

const FOOR_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const UPDATE_TIME: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    food_placed: bool,
    snake: Snake,
    food_x: u32,
    food_y: u32,
    width: u32,
    height: u32,
    is_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            food_placed: true,
            snake: Snake::new(2, 2),
            food_x: 5,
            food_y: 4,
            width,
            height,
            is_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn handle_key_press(&mut self, key: Key) {
        if self.is_over {
            return self.restart();
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }
        match dir {
            Some(d) => self.snake.change_direction(d),
            _ => (),
        }
    }

    pub fn restart(&mut self) {
        self.is_over = false;
        self.snake = Snake::new(2, 2);
        self.food_placed = true;
        self.food_x = 5;
        self.food_y = 4;
        self.waiting_time = 0.0
    }

    fn place_food(&mut self) {
        let mut rnd_x = thread_rng().gen_range(1..self.width - 1);
        let mut rnd_y = thread_rng().gen_range(1..self.height - 1);

        while self.snake.is_tail_overlap(rnd_x, rnd_y) {
            rnd_x = thread_rng().gen_range(1..self.width - 1);
            rnd_y = thread_rng().gen_range(1..self.height - 1);
        }
        self.food_placed = true;
        self.food_x = rnd_x;
        self.food_y = rnd_y;
    }
    fn is_eating(&mut self) {
        let (snake_pos_x, snake_pos_y) = self.snake.head_position();

        if (snake_pos_x == self.food_x) && (snake_pos_y == self.food_y) {
            self.food_placed = false;
            self.snake.grow();
        }
    }

    fn move_snake(&mut self, direction: Option<Direction>) {
        if self
            .snake
            .is_alive(self.width, self.height, self.snake.next_head(direction))
        {
            self.snake.move_forward(direction);
            self.is_eating();
        } else {
            self.is_over = true;
        }
        self.waiting_time = 0.0
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.is_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !(self.food_placed) {
            self.place_food()
        }
        if self.waiting_time > UPDATE_TIME {
            self.move_snake(None)
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        if self.food_placed {
            draw_block(FOOR_COLOR, self.food_x, self.food_y, ctx, g)
        }

        self.draw_borders(ctx, g);

        if self.is_over {
            draw_rect(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g)
        }
    }

    fn draw_borders(&self, ctx: &Context, g: &mut G2d) {
        draw_rect(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        draw_rect(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        draw_rect(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        draw_rect(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);
    }
}
