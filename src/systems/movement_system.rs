use bevy::ecs::{entity::Entity, system::Query};
use bevy::prelude::*;

use crate::components::Position;
use crate::Map;
use crate::RunState;

// #[derive(Component)]
// struct EntityMoved {}

// #[derive(Component)]
// pub struct ApplyMove {
//     pub dest_idx: usize
// }

/// This Message informs us that an Entity wants to move.
/// 
/// # Variables
/// - `entity` => `Entity` that wants to move
/// - `destination` => destination `Position` that it wants to move to
#[derive(Message)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}

/// This system handles Movement checks for Entities that want to move
/// 
/// Any time the `WantsToMove` message is sent, this system reads all of those events that occured
/// and proccesses whether they are valid movements or not. If they are it performs them for said
/// Entities.
pub fn movement_system(
    mut events: MessageReader<WantsToMove>,
    mut movers: Query<(Entity, &mut Position)>,
    map: Res<Map>, 
    mut run_state: ResMut<NextState<RunState>>
) {
    for msg in events.read() {
        if let Ok((_mov_ent, mut position)) = movers.get_mut(msg.entity) {
            let dest_idx = map.xy_idx(msg.destination.x, msg.destination.y);
            if !map.is_blocked(dest_idx){
                position.x = msg.destination.x;
                position.y = msg.destination.y;
            }
            run_state.set(RunState::Ticking);
        }
    }
}