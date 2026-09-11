use super::{Map, TileType, Position};
use super::constants::SHOW_MAPGEN_VISUALIZER;
use super::rect::Rect;
//use bevy::ecs::prelude::*;

mod starting_points;
use starting_points::*;
mod room_sorter;
use room_sorter::*;
mod simple_map;
use simple_map::SimpleMapBuilder;
mod bsp_dungeon;
use bsp_dungeon::BspDungeonBuilder;
mod bsp_interior;
use bsp_interior::BspInteriorBuilder;
mod common;
use common::*;
mod room_draw;
use room_draw::RoomDrawer;
mod corridors;
use corridors::*;
mod room_exploder;
use room_exploder::RoomExploder;
mod room_corner_rounding;
use room_corner_rounding::RoomCornerRounder;
mod room_based_starting_position;
use room_based_starting_position::RoomBasedStartingPosition;
mod room_based_stairs;
use room_based_stairs::RoomBasedStairs;
mod distant_exit;
use distant_exit::DistantExit;
mod cellular_automata;
use cellular_automata::CellularAutomataBuilder;
mod drunkard;
use drunkard::*;
mod maze;
use maze::MazeBuilder;
mod dla;
use dla::*;
mod voronoi;
use voronoi::*;
mod prefab_builder;
use prefab_builder::*;
mod cull_unreachable;
use cull_unreachable::CullUnreachable;
mod waveform_collapse;
use waveform_collapse::*;
mod door_placement;
use door_placement::DoorPlacement;
mod themed;
use themed::forest::*;
use themed::limestone_cavern::*;
use themed::mushroom_forest::*;
use themed::dwarf_fort_builder::*;
mod ending_position;
use ending_position::*;

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history: Vec<Map>,
    pub width: i32,
    pub height: i32
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap
}

impl BuilderChain {
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain { 
            starter: None, 
            builders: Vec::new(), 
            build_data: BuilderMap { 
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height, name), 
                starting_position: None, 
                rooms: None, 
                corridors: None, 
                history: Vec::new(), 
                width, 
                height 
            } 
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one initial builder!")
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("Can't run a map builder chain without an initial build system!"),
            Some(starter) => {
                starter.build_map(&mut self.build_data);
            }
        }

        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data : &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data : &mut BuilderMap);
}

fn random_start_position() -> (XStart, YStart) {
    let x;
    let xroll = crate::rng::roll_dice(1, 3);
    match xroll {
        1 => x = XStart::LEFT,
        2 => x = XStart::CENTER,
        _ => x = XStart::RIGHT
    }

    let y;
    let yroll = crate::rng::roll_dice(1, 3);
    match yroll {
        1 => y = YStart::BOTTOM,
        2 => y = YStart::CENTER,
        _ => y = YStart::TOP
    }

    (x, y)
}

fn random_room_builder(builder : &mut BuilderChain) {
    let build_roll = crate::rng::roll_dice(1, 3);
    match build_roll {
        1 => builder.start_with(SimpleMapBuilder::new()),
        2 => builder.start_with(BspDungeonBuilder::new()),
        _ => builder.start_with(BspInteriorBuilder::new())
    }

    // BSP Interior still makes holes in the walls
    if build_roll != 3 {
        // Sort by one of the 5 available algorithms
        let sort_roll = crate::rng::roll_dice(1, 5);
        match sort_roll {
            1 => builder.with(RoomSorter::new(RoomSort::LEFTMOST)),
            2 => builder.with(RoomSorter::new(RoomSort::RIGHTMOST)),
            3 => builder.with(RoomSorter::new(RoomSort::TOPMOST)),
            4 => builder.with(RoomSorter::new(RoomSort::BOTTOMMOST)),
            _ => builder.with(RoomSorter::new(RoomSort::CENTRAL)),
        }

        builder.with(RoomDrawer::new());

        let corridor_roll = crate::rng::roll_dice(1, 4);
        match corridor_roll {
            1 => builder.with(DoglegCorridors::new()),
            2 => builder.with(NearestCorridors::new()),
            3 => builder.with(StraightLineCorridors::new()),
            _ => builder.with(BspCorridors::new())
        }

        // let cspawn_roll = crate::rng::roll_dice(1, 2);
        // if cspawn_roll == 1 {
        //     builder.with(CorridorSpawner::new());
        // }

        let modifier_roll = crate::rng::roll_dice(1, 6);
        match modifier_roll {
            1 => builder.with(RoomExploder::new()),
            2 => builder.with(RoomCornerRounder::new()),
            _ => {}
        }
    }

    let start_roll = crate::rng::roll_dice(1, 2);
    match start_roll {
        1 => builder.with(RoomBasedStartingPosition::new()),
        _ => {
            let (start_x, start_y) = random_start_position();
            builder.with(StartingPosition::new(start_x, start_y));
        }
    }

    let exit_roll = crate::rng::roll_dice(1, 2);
    match exit_roll {
        1 => builder.with(RoomBasedStairs::new()),
        _ => builder.with(DistantExit::new())
    }

    // let spawn_roll = crate::rng::roll_dice(1, 2);
    // match spawn_roll {
    //     1 => builder.with(RoomBasedSpawner::new()),
    //     _ => builder.with(VoronoiSpawning::new())
    // }
}

fn random_shape_builder(builder: &mut BuilderChain) {
    let builder_roll = crate::rng::roll_dice(1, 16);
    match builder_roll {
        1 => builder.start_with(CellularAutomataBuilder::new()),
        2 => builder.start_with(DrunkardsWalkBuilder::open_area()),
        3 => builder.start_with(DrunkardsWalkBuilder::open_halls()),
        4 => builder.start_with(DrunkardsWalkBuilder::winding_passages()),
        5 => builder.start_with(DrunkardsWalkBuilder::fat_passages()),
        6 => builder.start_with(DrunkardsWalkBuilder::fearful_symmetry()),
        7 => builder.start_with(MazeBuilder::new()),
        8 => builder.start_with(DLABuilder::walk_inwards()),
        9 => builder.start_with(DLABuilder::walk_outwards()),
        10 => builder.start_with(DLABuilder::central_attractor()),
        11 => builder.start_with(DLABuilder::insectoid()),
        12 => builder.start_with(VoronoiCellBuilder::pythagoras()),
        13 => builder.start_with(VoronoiCellBuilder::manhattan()),
        _ => builder.start_with(PrefabBuilder::constant(prefab_builder::prefab_levels::WFC_POPULATED)),
    }

    // Set the start to the center and cull
    builder.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.with(CullUnreachable::new());

    // Now set the start to a random starting area
    let (start_x, start_y) = random_start_position();
    builder.with(StartingPosition::new(start_x, start_y));

    // Setup an exit and spawn mobs
    //builder.with(VoronoiSpawning::new());
    builder.with(DistantExit::new());
}

pub fn random_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    println!("Inside random_builder!");
    let mut builder = BuilderChain::new(new_depth, width, height, "New Map");
    let type_roll = crate::rng::roll_dice(1, 2);
    match type_roll {
        1 => random_room_builder(&mut builder),
        _ => random_shape_builder(&mut builder)
    }

    if crate::rng::roll_dice(1, 3)==1 {
        builder.with(WaveformCollapseBuilder::new());

        // Now set the start to a random starting area
        let (start_x, start_y) = random_start_position();
        builder.with(StartingPosition::new(start_x, start_y));

        // Setup an exit and spawn mobs
        //builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }

    if crate::rng::roll_dice(1, 20)==1 {
        builder.with(PrefabBuilder::sectional(prefab_builder::prefab_sections::UNDERGROUND_FORT));
    }

    builder.with(DoorPlacement::new());
    builder.with(PrefabBuilder::vaults());

    builder
}

pub fn level_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    match new_depth {
        //1 => town_builder(new_depth, width, height),
        2 => forest_builder(new_depth, width, height),
        3 => limestone_cavern_builder(new_depth, width, height),
        4 => limestone_deep_cavern_builder(new_depth, width, height),
        5 => limestone_transition_builder(new_depth, width, height),
        6 => dwarf_fort_builder(new_depth, width, height),
        7 => mushroom_entrance(new_depth, width, height),
        8 => mushroom_builder(new_depth, width, height),
        9 => mushroom_exit(new_depth, width, height),
        _ => random_builder(new_depth, width, height)
    }
}