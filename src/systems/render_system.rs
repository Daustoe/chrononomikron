use bevy::prelude::*;
use bevy_ascii_terminal::*;
use crate::{Renderable, Position, Player, Map, tile_glyph};

/// This system is responsible for drawing entities and enviornment to the map
/// 
/// It queries for all Entities with the `Renderable` and `Position` components
/// and draws them to the screen. 
/// 
/// It also draws all visible tiles within the `Map` Resource.
pub fn render(
    mut q_term: Query<&mut Terminal>,
    _q_player: Query<Entity, With<Player>>,
    map: Res<Map>,
    q_entities: Query<(&Renderable, &Position)>,
) {
    let mut term = match q_term.single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    term.clear();
    term.set_pivot(Pivot::LeftTop);

    // Render Map
    for x in 0..map.width {
        for y in 0..map.height {
            if map.revealed_tiles[map.xy_idx(x, y)] {
                let tile_data = tile_glyph(map.xy_idx(x, y), &map);
                let Some(tile) = term.try_tile_mut(IVec2::from_array([x, y])) else {
                    continue;
                };
                tile.glyph = tile_data.0;
                tile.fg_color = tile_data.1;
                tile.bg_color = tile_data.2;
            }
        }
    }

    // Render Entities
    for (r, pos) in q_entities.iter() {
        let Some(tile) = term.try_tile_mut(IVec2::from_array([pos.x, pos.y])) else {
            continue;
        };
        tile.glyph = r.glyph;
        tile.fg_color = r.fg;
        tile.bg_color = r.bg;
    }
}