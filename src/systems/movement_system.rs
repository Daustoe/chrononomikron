//use bevy::ecs::{entity::Entity, system::Query};
use bevy::prelude::*;

use crate::{Position, Map, RunState, Viewshed};
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
    mut movers: Query<(Entity, &mut Position, &mut Viewshed)>,
    mut map: ResMut<Map>, 
    mut run_state: ResMut<NextState<RunState>>
) {
    for msg in events.read() {
        if let Ok((mov_ent, mut position, mut viewshed)) = movers.get_mut(msg.entity) {
            let start_idx = map.xy_idx(position.x, position.y);
            let dest_idx = map.xy_idx(msg.destination.x, msg.destination.y);
            map.move_entity(mov_ent, start_idx, dest_idx);
            position.x = msg.destination.x;
            position.y = msg.destination.y;
            viewshed.dirty = true;
            run_state.set(RunState::Ticking);
            // TODO: Remove MyTurn marker here?
        }
    }
}