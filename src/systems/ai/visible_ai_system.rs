use bevy::prelude::*;
use crate::{MyTurn, Position, Map, WantsToMove, ai::WantsToApproach, ai::Reaction, Viewshed, Player};

pub fn visible_ai_system (
    mut commands: Commands,
    mut map: ResMut<Map>,
    q_entities: Query<(Entity, &Position, &Viewshed), With<MyTurn>>,
    q_player: Query<Entity, With<Player>>
) {
    for (entity, pos, viewshed) in q_entities.iter() {
        if entity != q_player.single().unwrap() {
            let my_idx = map.xy_idx(pos.x, pos.y);
            let mut reactions: Vec<(usize, Reaction, Entity)> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                if my_idx != idx {
                    evaluate(idx, &mut map, &mut reactions);
                }
            }
            for reaction in reactions.iter() {
                match reaction.1 {
                    Reaction::Attack => {
                        // let range = rltk::DistanceAlg::Pythagoras.distance2d(
                        //     rltk::Point::new(pos.x, pos.y), 
                        //     rltk::Point::new(reaction.0 as i32 % map.width, reaction.0 as i32 / map.width)
                        // );
                        let (pos_x, pos_y) = map.idx_xy(reaction.0);
                        commands.entity(entity).insert(WantsToApproach {position: Position {x: pos_x, y: pos_y}});
                        // TODO: insert Chasing Component as well once that is in here
                    },
                    Reaction::Flee => {
                        // TODO: implement flee_ai_system and add component here
                    }
                    _ => {}
                }
            }
        }
    }
}

fn evaluate(idx: usize, map: &mut Map, reactions: &mut Vec<(usize, Reaction, Entity)>) {
    // TODO: This will eventually handle determining how this entity reacts to other entities 
    // around it. 
    map.for_each_tile_content(idx, |other_entity| {
        reactions.push((
            idx,
            Reaction::Attack,
            other_entity
        ));
    });
}