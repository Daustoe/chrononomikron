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
    for (entity, pos, chasing) in q_entities.iter() {
        let (target,target_pos) = q_positions.get(chasing.target).unwrap();
        //let target_pos = TODO need to find how to get all entities w
    }
}