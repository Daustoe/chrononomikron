use bevy::ecs::{entity::Entity, prelude::Component, system::Query};
use bevy::prelude::*;

use crate::components::Position;

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
    mut movers: Query<(Entity, &mut Position)>
) {
    for msg in events.read() {
        if let Ok((_mov_ent, mut position)) = movers.get_mut(msg.entity) {
            position.x = msg.destination.x;
            position.y = msg.destination.y;
        }
    }
}