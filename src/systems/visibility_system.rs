use bevy::prelude::*;
use crate::{Viewshed, Position, Map, Player};
use rltk::{Point, field_of_view};

pub fn visibility_system(
    mut map: ResMut<Map>,
    //vis_blockers: Query<(Entity, BlocksVisibility)>,
    mut q_player: Query<Entity, With<Player>>,
    mut entities_viewing: Query<(Entity, &mut Viewshed, &mut Position)>
) {
    for (ent, mut view, pos) in entities_viewing.iter_mut() {
        if view.dirty {
            //let point: Position = Position::new(pos.x, pos.y);
            let point_vec = field_of_view(Point::new(pos.x, pos.y), view.range, &*map);
            view.visible_tiles = point_vec.into_iter().map(Into::into).collect();
            view.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            // If this is the player, reveal what they can see
            if ent == q_player.single().unwrap() {
                for t in map.visible_tiles.iter_mut() { *t = false };
                for vis in view.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }
}