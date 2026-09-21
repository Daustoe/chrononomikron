use bevy::prelude::*;
use serde::Deserialize;
use std::cmp;
use rltk::Point;
use crate::Viewshed;

/// This component assigns a position in the game world to it's attached Entity.
#[derive(Component, Debug, Clone, Eq, Default, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component, Debug)]
pub struct Health {
    pub health: i32
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct Mana {
    pub mana: i32
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

/// This compoenent informs us that the Entity can be rendered and displayed.
#[derive(Component, Clone, Deserialize, Debug, Copy)]
pub struct Renderable {
    pub glyph: char,
    //#[serde(deserialize_with = "deserialize_hex_color")]
    pub fg: LinearRgba,
    //#[serde(deserialize_with = "deserialize_hex_color")]
    pub bg: LinearRgba
}

/// This component informs us that the Entity can take Actions and should be on the Turn Scheduler.
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

/// This is a bundle of components that are commonly needed for any Entity that can take actions.
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