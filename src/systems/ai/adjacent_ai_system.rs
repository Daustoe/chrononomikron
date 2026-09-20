use bevy::prelude::*;
use crate::{Map, Position, MyTurn, Player, ai::Reaction, RunState};

pub fn adjacent_ai_system (
    mut commands: Commands, 
    mut map: ResMut<Map>,
    q_entities: Query<(Entity, &Position), With<MyTurn>>,
    q_player: Query<Entity, With<Player>>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    let mut turn_done: Vec<Entity> = Vec::new();
    for (entity, pos) in q_entities.iter() {
        if entity != q_player.single().unwrap() {
            let mut reactions: Vec<(Entity, Reaction)> = Vec::new();
            let idx = map.xy_idx(pos.x, pos.y);
            let w = map.width;
            let h = map.height;

            // Add possible reactions to adjacents for each direction
            if pos.x > 0 { evaluate(idx-1, &mut map, &mut reactions); }
            if pos.x < w-1 { evaluate(idx+1, &mut map, &mut reactions); }
            if pos.y > 0 { evaluate(idx-w as usize, &mut map, &mut reactions); }
            if pos.y < h-1 { evaluate(idx+w as usize, &mut map, &mut reactions); }
            if pos.y > 0 && pos.x > 0 { evaluate((idx-w as usize)-1, &mut map, &mut reactions); }
            if pos.y > 0 && pos.x < w-1 { evaluate((idx-w as usize)+1, &mut map, &mut reactions); }
            if pos.y < h-1 && pos.x > 0 { evaluate((idx+w as usize)-1, &mut map, &mut reactions); }
            if pos.y < h-1 && pos.x < w-1 { evaluate((idx+w as usize)+1, &mut map, &mut reactions); }
        
            let mut done = false;
            for reaction in reactions.iter() {
                if let Reaction::Attack = reaction.1 {
                    println!("{:?} wants to attack!", entity);
                    done = true;
                }
            }
            if done { turn_done.push(entity); }
        }
    }

    // Remove turn marker for those that are done
    for done in turn_done.iter() {
        commands.entity(*done).remove::<MyTurn>();
        run_state.set(RunState::Ticking); 
        //TODO: The runstate above should be moved to Melee attack system once implemented.
    }
}

fn evaluate(idx: usize, map: &mut Map, reactions: &mut Vec<(Entity, Reaction)>) {
    map.for_each_tile_content(idx, |other_entity| {
        // TODO check for factions to determine reaction
        reactions.push((
            other_entity,
            Reaction::Attack,
        ));
    });
}
