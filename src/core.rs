use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::{draw_block, draw_rect};

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Clone, Copy)]
struct Block {
    x: u32,
    y: u32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        let mut body = LinkedList::new();
        body.push_front(Block { x, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x + 2, y });

        Snake {
            direction: Direction::Down,
            body,
            tail: None,
        }
    }

    pub fn head_position(&self) -> (u32, u32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn is_alive(&self, w: u32, h: u32, pos: (u32, u32)) -> bool {
        let (pos_x, pos_y) = pos;
        if self.is_tail_overlap(pos_x, pos_y) {
            return false;
        }

        if pos_x <= 0 || pos_x >= w - 1 || pos_y <= 0 || pos_y >= h - 1 {
            return false;
        }
        true
    }
    pub fn change_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn is_tail_overlap(&self, pos_x: u32, pos_y: u32) -> bool {
        for block in &self.body {
            if pos_x == block.x && pos_y == block.y {
                return true;
            }
        }
        false
    }

    pub fn grow(&mut self) {
        let temp = self.tail.clone().unwrap();
        self.body.push_back(temp)
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (u32, u32) {
        let (pos_x, pos_y) = self.head_position();

        let mut dir = self.direction;
        match direction {
            Some(d) => dir = d,
            None => {}
        }

        match dir {
            Direction::Up => (pos_x, pos_y - 1),
            Direction::Down => (pos_x, pos_y + 1),
            Direction::Left => (pos_x - 1, pos_y),
            Direction::Right => (pos_x + 1, pos_y),
        }
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y) = self.head_position();

        let new_block = match &self.direction {
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
        };
        self.body.push_front(new_block);
        let removed = self.body.pop_back().unwrap();
        self.tail = Some(removed)
    }
}
