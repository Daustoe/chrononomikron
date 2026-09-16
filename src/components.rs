use bevy::prelude::*;
use serde::Deserialize;
use std::cmp;
use rltk::Point;
use crate::Viewshed;

#[derive(Component, Debug, Clone, Eq, Default, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Position> for Point {
    fn from(position: Position) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

impl From<Point> for Position {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

#[derive(Component, Clone, Deserialize, Debug, Copy)]
pub struct Renderable {
    pub glyph: char,
    //#[serde(deserialize_with = "deserialize_hex_color")]
    pub fg: LinearRgba,
    //#[serde(deserialize_with = "deserialize_hex_color")]
    pub bg: LinearRgba
}

#[derive(Component, Debug)]
pub struct Actor {}

impl Default for Viewshed {
    fn default() -> Self {
        Self {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        }
    }
}

#[derive(Bundle, Debug)]
pub struct ActingEntityBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub viewshed: Viewshed, 
    pub actor: Actor
}

impl ActingEntityBundle {
    pub fn new(fg: LinearRgba, glyph: char) -> Self {
        Self {
            renderable: Renderable { glyph, fg, bg: LinearRgba::BLACK },
            position: Position::default(),
            viewshed: Viewshed::default(),
            actor: Actor{}
        }
    }
}