use bevy::prelude::*;
mod tiletype;
use std::collections::HashSet;
pub use tiletype::{TileType, tile_walkable};

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub depth : i32,
    pub bloodstains : HashSet<usize>,
    pub view_blocked : HashSet<usize>,
    pub name : String,
    pub outdoors : bool,
    //pub light : Vec<rltk::RGB>,
}
