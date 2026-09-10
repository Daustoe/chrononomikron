use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub glyph: char,
    pub fg: LinearRgba,
    pub bg: LinearRgba
}

#[derive(Component, Clone)]
pub struct Player {}