use bevy::prelude::*;
use crate::{Position, TimeManager, Map};
pub mod asset_loader;
mod mob_defines;
pub mod player;
pub use player::*;
pub use asset_loader::*;
pub use mob_defines::MonsterBundle;

#[derive(Message)]
pub struct SpawnNpc {
    pub position: Position,
    pub def_key: NPC
}

pub fn spawn_system (
    mut commands: Commands,
    mut npc_spawns: MessageReader<SpawnNpc>,
    definitions: Res<NpcDefinitions>, 
    mut queue: ResMut<TimeManager>,
    mut map: ResMut<Map>,
) {
    for msg in npc_spawns.read() {
        let Some(key) = definitions.npcs.get(&msg.def_key)
        else {
            return;
        };
        let entity = commands.spawn(MonsterBundle::new(msg.position, key.clone())).id();
        map.index_entity(entity, msg.position, true);
        queue.push(entity);
    }
}
