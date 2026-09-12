use bevy::ecs::{entity::Entity, prelude::Component, system::Query};
use bevy::prelude::*;

use crate::components::Position;

#[derive(Component)]
struct EntityMoved {}

#[derive(Component)]
pub struct ApplyMove {
    pub dest_idx: usize
}

#[derive(Component)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

pub fn movement_system(
    mut commands: Commands,
    wants_move: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &mut Position)>
) {
    for (want_move_entity, wants_to_move) in wants_move.iter() {
        if let Ok((_mov_ent, mut position)) = movers.get_mut(wants_to_move.entity) {
            position.x = wants_to_move.destination.x;
            position.y = wants_to_move.destination.y;
        }
        commands.entity(want_move_entity).remove::<Children>().despawn();
    }

    
}