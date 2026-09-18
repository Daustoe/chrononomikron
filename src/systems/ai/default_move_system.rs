use bevy::prelude::*;
use serde::Deserialize;
use crate::{Map, Position, WantsToMove, tile_walkable, MyTurn};

/// This Enum defines the types of Movements an Entity can have in MoveMode.
/// 
/// - Static: Does not move.
/// - Random: Picks a random direction to move each turn.
/// - RandomWaypoint: Picks a random valid location and moves towards it each turn.
#[derive(Debug, Deserialize, Clone)]
pub enum Movement {
    Static,
    Random,
    RandomWaypoint{ path: Option<Vec<usize>> }
}

/// This component informs us that the Entity uses the default movement system.
#[derive(Component, Debug)]
pub struct MoveMode {
    pub mode: Movement
}

/// This system is defines what movement AI is default for an Actor.
/// 
/// The default MoveMode is set by the NpcDefinition asset in the RON file. 
/// It gives the Actor something to do if they have no other action to do in their AI systems.
pub fn default_move_ai_system (
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut q_entities: Query<(Entity, &mut MoveMode, &Position), With<MyTurn>>,
    mut wants_move: MessageWriter<WantsToMove>,
) {
    for (entity, mut mode, pos) in q_entities.iter_mut() {
        match &mut mode.mode {
            Movement::Static => {},
            Movement::Random => {
                let mut x = pos.x;
                let mut y = pos.y;
                let move_roll = crate::rng::roll_dice(1, 5);
                match move_roll {
                    1 => x -= 1,
                    2 => x += 1,
                    3 => y -= 1,
                    4 => y += 1,
                    _ => {}
                }

                if x > 0 && x < map.width-1 && y > 0 && y < map.height-1 {
                    let dest_idx = map.xy_idx(x, y);
                    if !map.is_blocked(dest_idx) {
                        wants_move.write(WantsToMove { entity, destination: Position { x, y }});
                    }
                }
            },
            Movement::RandomWaypoint { path } => {
                if let Some(path) = path {
                    // We have a target - go there
                    if path.len() > 1 {
                        if !map.is_blocked(path[1] as usize) {
                            let (x, y) = map.idx_xy(path[1]);
                            wants_move.write(WantsToMove { entity, destination: Position { x, y}});
                            path.remove(0);
                            commands.entity(entity).remove::<MyTurn>();
                        }
                    } else {
                        mode.mode = Movement::RandomWaypoint { path: None };
                    }
                } else {
                    let target_x = crate::rng::roll_dice(1, map.width-2);
                    let target_y = crate::rng::roll_dice(1, map.height-2);
                    let idx = map.xy_idx(target_x, target_y);
                    if tile_walkable(map.tiles[idx]) {
                        let path = rltk::a_star_search(
                            map.xy_idx(pos.x, pos.y),
                            map.xy_idx(target_x, target_y),
                            &mut *map
                        );
                        if path.success && path.steps.len()>1 {
                            mode.mode = Movement::RandomWaypoint { path: Some(path.steps) };
                        }
                    }
                }
            }
        }
    }
}