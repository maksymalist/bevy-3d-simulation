
use bevy::prelude::{Color, Component};

#[derive(Clone, Component)]
pub struct Block {
    pub position: Vec<usize>,
    pub size: f32,
    pub color: Color,
    pub is_alive: bool,
}

impl Block {
    pub fn new(position: Vec<usize>, size: f32, color: Color, is_alive: bool) -> Self {
        Self {
            position,
            size,
            color,
            is_alive,
        }
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn revive(&mut self) {
        self.is_alive = true;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }
}