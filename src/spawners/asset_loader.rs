use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use crate::{Renderable, Movement};

/// Defined here are the different NPC types that we expect to find in our 
/// RON file.
#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum NPC {
    Villager,
    Goblin
}

/// This struct defines what variables should be a part of the NPC definitions
/// in the RON file. 
/// 
/// # Attributes
/// -`name`: String of the NPC name
/// -`renderable`: Optional `Renderable` Component if we want to display this NPC
/// -`blocks_tile`: boolean that defines if this Entity blocks other tiles from moving to it's current tile
/// -`movement`: `Movement` Component that defines this NPC's default `MoveMode`
/// -`gold`: Optional `String` in the dice roll format that determines a random abount of gold this NPC will have
/// -`health`: Optional `i32` for how much health this NPC starts with
/// -`mana`: Optional `i32` for how much mana this NPC starts with
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct NpcDefinition {
    pub name: String,
    #[serde(default)]
    pub renderable: Option<Renderable>,
    pub blocks_tile: bool,
    pub movement: Movement,
    #[serde(default)]
    pub gold: Option<String>,
    #[serde(default)]
    pub health: Option<i32>,
    #[serde(default)]
    pub mana: Option<i32>
}

/// This ECS Resource contains a HashMap list of all the NPC definitions 
/// loaded in from the RON file.
#[derive(Resource, Debug)]
pub struct NpcDefinitions {
    pub npcs: HashMap<NPC, NpcDefinition>,
}

/// This function is responsible for loading, parsing, and initializing the `NpcDefinitions` Resource.
pub fn load_npc_definitions(mut commands: Commands) {
    let text = std::fs::read_to_string("assets/definitions/npcs.ron")
        .expect("Failed to read NPC definitions");

    let npcs: HashMap<NPC, NpcDefinition> =
        ron::from_str(&text)
            .expect("Failed to parse NPC definitions");

    commands.insert_resource(NpcDefinitions { npcs });
}
