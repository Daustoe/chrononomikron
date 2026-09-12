use bevy::ecs::{entity::Entity, prelude::Component, system::Query};
use bevy::prelude::*;

use crate::components::Position;
use crate::Map;

#[derive(Component)]
struct EntityMoved {}

#[derive(Component)]
pub struct ApplyMove {
    pub dest_idx: usize
}

#[derive(Message)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

pub fn movement_system(
    mut events: MessageReader<WantsToMove>,
    mut movers: Query<(Entity, &mut Position)>,
    map: Res<Map>
) {
    for msg in events.read() {
        if let Ok((_mov_ent, mut position)) = movers.get_mut(msg.entity) {
            let dest_idx = map.xy_idx(msg.destination.x, msg.destination.y);
            if !map.is_blocked(dest_idx){
                position.x = msg.destination.x;
                position.y = msg.destination.y;
            }
        }
    }
}