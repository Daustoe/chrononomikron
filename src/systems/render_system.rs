use bevy::prelude::*;
use bevy_ascii_terminal::*;
use crate::{Renderable, Position, Player, Map, tile_glyph, RunState};

/// This system is responsible for drawing entities and enviornment to the map
/// 
/// It queries for all Entities with the `Renderable` and `Position` components
/// and draws them to the screen. 
/// 
/// It also draws all visible tiles within the `Map` Resource.
pub fn render(
    mut q_term: Query<&mut Terminal>,
    q_player: Query<(Entity, &Position), With<Player>>,
    map: Res<Map>,
    q_entities: Query<(&Renderable, &Position)>,
    //current_state: Res<State<RunState>>
) {
    let mut term = match q_term.single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };
    //println!("Current State: {:?}", current_state);

    term.clear();
    term.set_pivot(Pivot::LeftTop);
    
    // get the bounds of the screen based on player position
    let (_player, player_pos) = q_player.single().unwrap();
    let (min_x, max_x, min_y, max_y) = get_screen_bounds(player_pos.clone());

    for (y, ty) in (min_y .. max_y).enumerate() {
        let y = y as i32;
        for (x, tx) in (min_x .. max_x).enumerate() {
            let x = x as i32;
            if tx > 0 && tx < map.width && ty > 0 && ty < map.height {
                let idx = map.xy_idx(tx, ty);
                if map.revealed_tiles[idx] {
                    let tile_data = tile_glyph(idx, &map);
                    let Some(tile) = term.try_tile_mut(IVec2::from_array([x+1, y+1])) else {
                        continue;
                    };
                    tile.glyph = tile_data.0;
                    tile.fg_color = tile_data.1;
                    tile.bg_color = tile_data.2;
                }
            }
        }
    }

    // Render Entities
    for (r, pos) in q_entities.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            let entity_screen_x = pos.x - min_x;
            let entity_screen_y = pos.y - min_y;
            if entity_screen_x > 0 && entity_screen_x < map.width && entity_screen_y > 0 && entity_screen_y < map.height {
                let Some(tile) = term.try_tile_mut(IVec2::from_array([entity_screen_x+1, entity_screen_y+1])) else {
                    continue;
                };
                tile.glyph = r.glyph;
                tile.fg_color = r.fg;
                tile.bg_color = r.bg;
            }
        }
    }
}

pub fn get_screen_bounds(player_pos: Position) -> (i32, i32, i32, i32){
    let (x_chars, y_chars) = (80, 50);

    let center_x = x_chars / 2;
    let center_y = y_chars / 2;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars;
    (min_x, max_x, min_y, max_y)
}
