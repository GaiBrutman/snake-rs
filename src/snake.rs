use std::collections::VecDeque;
use piston_window::*;
use crate::block::Block;

const SNAKE_SIZE: f64 = 20.0;
const GREEN: types::Color = [0.0, 1.0, 0.0, 1.0];

pub struct Snake {
    blocks: VecDeque<Block>,
    vel_x: i32,
    vel_y: i32,
}

impl Snake {
    pub fn new() -> Snake {
        let mut blocks: VecDeque<Block> = VecDeque::new();
        blocks.push_front(Block::new(0.0, 0.0, SNAKE_SIZE, GREEN));
        blocks.push_front(Block::new(SNAKE_SIZE, 0.0, SNAKE_SIZE, GREEN));
        blocks.push_front(Block::new(SNAKE_SIZE, SNAKE_SIZE, SNAKE_SIZE, GREEN));
        blocks.push_front(Block::new(2.0 * SNAKE_SIZE, SNAKE_SIZE, SNAKE_SIZE, GREEN));

        Snake {
            blocks: blocks,
            vel_x: 0,
            vel_y: 0,
        }
    }

    pub fn set_vel(&mut self, vel_x: i32, vel_y: i32) {
        self.vel_x = vel_x;
        self.vel_y = vel_y;
    }

    fn remove_tail(&mut self) {
        self.blocks.pop_back();
    }

    pub fn update(&mut self) {
        let mut new_block = Block::new(0.0, 0.0, SNAKE_SIZE, GREEN);

        match self.blocks.get_mut(0) {
            None => {},
            Some(head) => {
                new_block.x = head.x + self.vel_x as f64 * SNAKE_SIZE;
                new_block.y = head.y + self.vel_y as f64 * SNAKE_SIZE;
            }
        }

        self.blocks.push_front(new_block);
        self.remove_tail();
    }

    pub fn show(&self, context: Context, graphics: &mut G2d) {
        for block in self.blocks.iter() {
            block.show(context, graphics);
        }
    }
}