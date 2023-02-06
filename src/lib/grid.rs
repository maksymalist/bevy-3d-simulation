use std::collections::HashMap;
use crate::lib::block::Block;
use bevy::prelude::{Color, Resource};

#[derive(Clone, Resource)]
pub struct Grid {
    pub grid: HashMap<Vec<usize>, Block>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub block_size: f32,
}

impl Grid {
    pub fn new(width: usize, height: usize, depth: usize, block_size: f32) -> Self {
        Self {
            grid: HashMap::new(),
            width,
            height,
            depth,
            block_size,
        }
    }
    pub fn generate_blocks (&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.depth {
                    let mut alive: bool = true;

                    if x+y+z % 2 == 0 {
                        alive = false;
                    }

                    let color = Color::rgb(
                        (x as f32 / self.width as f32) * 0.5,
                        (y as f32 / self.height as f32) * 0.5,
                        (z as f32 / self.depth as f32) * 0.5,
                    );

                    let block = Block::new(vec![x, y, z], self.block_size, color, alive);
                    self.grid.insert(vec![x, y, z], block);
                }
            }
        }
    }

    pub fn get_block(&self, position: Vec<usize>) -> Option<&Block> {
        self.grid.get(&position)
    }

    pub fn get_block_mut(&mut self, position: Vec<usize>) -> Option<&mut Block> {
        self.grid.get_mut(&position)
    }

    pub fn get_neighbors(&self, position: Vec<usize>) -> Vec<&Block> {
        let mut neighbors = Vec::new();

        let x = position[0];
        let y = position[1];
        let z = position[2];

        let positions = vec![
            vec![x - 1, y, z],
            vec![x + 1, y, z],
            vec![x, y - 1, z],
            vec![x, y + 1, z],
            vec![x, y, z - 1],
            vec![x, y, z + 1],
        ];

        for position in positions {
            if let Some(block) = self.get_block(position) {
                neighbors.push(block);
            }
        }

        neighbors
    }

    pub fn update(&mut self) {
        let mut new_grid = HashMap::new();

        for (position, block) in self.grid.iter() {
            let neighbors = self.get_neighbors(position.clone());
            let mut alive_neighbors = 0;

            for neighbor in neighbors {
                if neighbor.is_alive {
                    alive_neighbors += 1;
                }
            }

            let mut new_block = block.clone();

            if block.is_alive {
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    new_block.kill();
                }
            } else {
                if alive_neighbors == 3 {
                    new_block.revive();
                }
            }

            new_grid.insert(position.clone(), new_block);
        }

        self.grid = new_grid;
    }
}