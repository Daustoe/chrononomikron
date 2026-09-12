use bevy::prelude::*;
use std::cmp;

#[derive(Component, Debug, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub glyph: char,
    pub fg: LinearRgba,
    pub bg: LinearRgba
}

#[derive(Component, Clone)]
pub struct Player {}