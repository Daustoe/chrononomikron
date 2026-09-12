use bevy::prelude::*;
mod tiletype;
use std::collections::HashSet;
pub use tiletype::{TileType, tile_walkable, tile_cost, tile_opaque};
use rltk::{BaseMap, Algorithm2D, Point};
mod themes;
pub use themes::*;
use crate::RunState;

#[derive(Resource, Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub depth : i32,
    pub bloodstains : HashSet<usize>,
    pub view_blocked : HashSet<usize>,
    pub blocked : Vec<(bool, bool)>,
    pub tile_content : Vec<Vec<(Entity, bool)>>,
    pub name : String,
    pub outdoors : bool,
    //pub light : Vec<rltk::RGB>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i].0 = !tile_walkable(*tile);
        }
    }

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
        let idx = self.xy_idx(x, y);
        !self.is_blocked(idx)
    }

    pub fn index_entity(&mut self, entity: Entity, idx: usize, blocks_tile: bool) {
        self.tile_content[idx].push((entity, blocks_tile));
        if blocks_tile {
            self.blocked[idx].1 = true;
        }
    }

    pub fn is_blocked(&self, idx: usize) -> bool {
        self.blocked[idx].0 || self.blocked[idx].1
    }

    pub fn set_blocked(&mut self, idx: usize, blocked: bool) {
        self.blocked[idx] = (self.blocked[idx].0, blocked);
    }

    pub fn for_each_tile_content<F>(&mut self, idx: usize, mut f: F) where F: FnMut(Entity) {
        for entity in self.tile_content[idx].iter() {
            f(entity.0);
        }
    }

    pub fn move_entity(&mut self, entity: Entity, moving_from: usize, moving_to: usize) {
        let mut entity_blocks = false;
        self.tile_content[moving_from].retain(|(e, blocks)| {
            if *e == entity {
                entity_blocks = *blocks;
                false
            } else {
                true
            }
        });
        self.tile_content[moving_to].push((entity, entity_blocks));

        let mut from_blocked = false;
        let mut to_blocked = false;
        self.tile_content[moving_from].iter().for_each(|(_,blocks)| if *blocks { from_blocked = true; } );
        self.tile_content[moving_to].iter().for_each(|(_,blocks)| if *blocks { to_blocked = true; } );
        self.blocked[moving_from].1 = from_blocked;
        self.blocked[moving_to].1 = to_blocked;
    }

    pub fn for_each_tile_content_with_gamemode<F>(&mut self, idx: usize, mut f: F) -> RunState 
    where F: FnMut(Entity)->Option<RunState>
    {
        for entity in self.tile_content[idx].iter() {
            if let Some(rs) = f(entity.0) {
                return rs;
            }
        }
        RunState::AwaitingInput
    }

    pub fn remove_entity(&mut self, entity: Entity, idx: usize) {
        self.tile_content[idx].retain(|(e, _)| *e != entity );
        let mut from_blocked = false;
        self.tile_content[idx].iter().for_each(|(_,blocks)| if *blocks { from_blocked = true; });
        self.blocked[idx].1 = from_blocked;
    }

    pub fn get_tile_content_clone(&self, idx:usize) -> Vec<Entity> {
        self.tile_content[idx].iter().map(|(e,_)| *e).collect()
    }

    pub fn clear(&mut self) {
        self.blocked.iter_mut().for_each(|b| { b.0 = false; b.1 = false; });
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> Map {
        let map_tile_count = (width*height) as usize;
        Map {
            tiles: vec![TileType::Wall; map_tile_count],
            width,
            height,
            revealed_tiles : vec![false; map_tile_count],
            visible_tiles : vec![false; map_tile_count],
            depth: new_depth,
            bloodstains: HashSet::new(),
            view_blocked : HashSet::new(),
            blocked: vec![(false, false); map_tile_count],
            tile_content: vec![Vec::new(); map_tile_count],
            name : name.to_string(),
            outdoors : true
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        if idx > 0 && idx < self.tiles.len() {
            tile_opaque(self.tiles[idx]) || self.view_blocked.contains(&idx)
        } else {
            true
        }
    }

    fn get_available_exits(&self, idx:usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        const DIAGONAL_COST : f32 = 1.5;
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let tt = self.tiles[idx as usize];
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, tile_cost(tt))) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, tile_cost(tt))) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, tile_cost(tt))) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, tile_cost(tt))) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, tile_cost(tt) * DIAGONAL_COST)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, tile_cost(tt) * DIAGONAL_COST)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, tile_cost(tt) * DIAGONAL_COST)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, tile_cost(tt) * DIAGONAL_COST)); }

        exits
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}