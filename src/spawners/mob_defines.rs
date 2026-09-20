use bevy::prelude::*;
use crate::{ActingEntityBundle, Position, MoveMode, NpcDefinition, Actor, Viewshed, Health, Mana};

/// Represents the collection of Components needed for a Monster.
/// 
/// - `actor_bundle`: `ActingEntityBundle` needed for any Entity that takes Actions.
/// - `movement`: default movement AI
#[derive(Bundle, Debug)]
pub struct MonsterBundle {
    pub actor_bundle: ActingEntityBundle,
    pub movement: MoveMode,
    pub health: Health,
    pub mana: Mana,
}

impl MonsterBundle {
    /// Creates a new Entity with the attached Components needed
    /// 
    /// # Arguments
    /// - `actor_bundle` : Bundle loaded in from the NpcDefinition RON files
    /// - `movement` : Default MoveMode to be used.
    pub fn new (position: Position, npc: NpcDefinition ) -> Self {
        Self {
            actor_bundle: ActingEntityBundle {
                renderable: npc.renderable.expect("No Available Renderable option for Monster!"),
                position, 
                viewshed: Viewshed::default(),
                actor: Actor{}
            },
            movement: MoveMode {mode: npc.movement},
            health: Health { health: npc.health.unwrap() },
            mana: Mana { mana: npc.mana.unwrap() }
        }
    }
}