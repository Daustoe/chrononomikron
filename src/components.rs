use bevy::prelude::*;
use serde::Deserialize;
use std::cmp;
use rltk::Point;

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

#[derive(Component, Clone)]
pub struct Player {}

#[derive(Component, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec::<Position>, 
    pub range: i32,
    pub dirty: bool
}

// Example of deserializer for LinearRgba
// fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<LinearRgba, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     println!("Using deserializer");
//     let hex = String::deserialize(deserializer)?;

//     let hex = hex.strip_prefix('#').unwrap_or(&hex);

//     if hex.len() != 6 {
//         return Err(serde::de::Error::custom(
//             "color must be a 6-digit hex value",
//         ));
//     }

//     let r = u8::from_str_radix(&hex[0..2], 16)
//         .map_err(serde::de::Error::custom)?;

//     let g = u8::from_str_radix(&hex[2..4], 16)
//         .map_err(serde::de::Error::custom)?;

//     let b = u8::from_str_radix(&hex[4..6], 16)
//         .map_err(serde::de::Error::custom)?;

//     Ok(LinearRgba::from(Srgba::new(
//         r as f32 / 255.0,
//         g as f32 / 255.0,
//         b as f32 / 255.0,
//         1.0,
//     )))
// }