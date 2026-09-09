use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: LinearRgba,
    pub bg: LinearRgba
}

#[derive(Component)]
pub struct Player {}