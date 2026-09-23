use bevy::prelude::*;
use crate::{MyTurn, Position, Map, WantsToMove};
use std::collections::HashMap;

#[derive(Component)]
pub struct Chasing {
    pub target: Entity
}

pub fn chasing_ai_system (
    mut commands: Commands,
    mut map: ResMut<Map>,
    q_entities: Query<(Entity, &Position, &Chasing), With<MyTurn>>,
    q_positions: Query<(Entity, &Position)>,
    mut wants_move: MessageWriter<WantsToMove>,
) {
    let mut targets: HashMap<Entity, Position> = HashMap::new();
    let mut end_chase: Vec<Entity> = Vec::new();
    for (entity, _pos, chasing) in q_entities.iter() {
        let target_pos = q_positions.get_inner(chasing.target);
        if let Ok(target_pos) = target_pos {
            targets.insert(entity, *target_pos.1);
        } else {
            end_chase.push(entity);
        }
    }

    for done in end_chase.iter() {
        commands.entity(*done).remove::<Chasing>();
    }
    end_chase.clear();

    let mut turn_done: Vec<Entity> = Vec::new();
    for (entity, pos, _chasing) in q_entities.iter() {
        turn_done.push(entity);
        let target_pos = targets[&entity];
        let path = rltk::a_star_search(
            map.xy_idx(pos.x, pos.y), 
            map.xy_idx(target_pos.x, target_pos.y), 
            &mut *map
        );
        if path.success && path.steps.len()>1 && path.steps.len()<15 {
            let (x, y) = map.idx_xy(path.steps[1]);
            wants_move.write(WantsToMove { entity, destination: Position {x, y} });
            turn_done.push(entity);
        } else {
            end_chase.push(entity);
        }
    }

    for done in end_chase.iter() {
        commands.entity(*done).remove::<Chasing>();
    }
    for done in turn_done.iter() {
        commands.entity(*done).remove::<MyTurn>();
    }
}