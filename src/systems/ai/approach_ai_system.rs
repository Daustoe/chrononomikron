use bevy::prelude::*;
use crate::{Position, Map, MyTurn, WantsToMove};

#[derive(Component, Clone)]
pub struct WantsToApproach {
    pub position: Position
}

pub fn approach_ai_system (
    mut commands: Commands,
    mut map: ResMut<Map>,
    q_entities: Query<(Entity, &Position, &WantsToApproach), With<MyTurn>>,
    mut wants_move: MessageWriter<WantsToMove>,
) {
    let mut turn_done: Vec<Entity> = Vec::new();
    for (entity, pos, wants_approach) in q_entities.iter() {
        turn_done.push(entity);
        let path = rltk::a_star_search(
            map.xy_idx(pos.x, pos.y), 
            map.xy_idx(wants_approach.position.x, wants_approach.position.y), 
            &mut *map
        );
        if path.success && path.steps.len()>1 {
            let (new_x, new_y) = map.idx_xy(path.steps[1]);
            wants_move.write(WantsToMove {entity, destination: Position {x: new_x, y: new_y}});
            commands.entity(entity).remove::<WantsToApproach>();
        }
    }

    for done in turn_done.iter() {
        commands.entity(*done).remove::<MyTurn>();
    }
}