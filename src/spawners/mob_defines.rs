use bevy::prelude::*;
use crate::{ActingEntityBundle, Position, MoveMode, NpcDefinition, Actor, Viewshed};

#[derive(Bundle, Debug)]
pub struct MonsterBundle {
    pub actor_bundle: ActingEntityBundle,
    pub movement: MoveMode
}

impl MonsterBundle {
    pub fn new (position: Position, npc: NpcDefinition ) -> Self {
        Self {
            actor_bundle: ActingEntityBundle {
                renderable: npc.renderable.expect("No Available Renderable option for Monster!"),
                position, 
                viewshed: Viewshed::default(),
                actor: Actor{}
            },
            movement: MoveMode {mode: npc.movement}
        }
    }
}