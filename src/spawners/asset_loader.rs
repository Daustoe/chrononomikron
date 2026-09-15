use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use crate::{Renderable, Movement};

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum NPC {
    Villager,
    Goblin
}

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

#[derive(Resource, Debug)]
pub struct NpcDefinitions {
    pub npcs: HashMap<NPC, NpcDefinition>,
}

pub fn load_npc_definitions(mut commands: Commands) {
    let text = std::fs::read_to_string("assets/definitions/npcs.ron")
        .expect("Failed to read NPC definitions");

    let npcs: HashMap<NPC, NpcDefinition> =
        ron::from_str(&text)
            .expect("Failed to parse NPC definitions");

    commands.insert_resource(NpcDefinitions { npcs });
}
