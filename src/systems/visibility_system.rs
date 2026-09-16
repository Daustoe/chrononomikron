use bevy::prelude::*;
use crate::{Position, Map, Player};
use rltk::{Point, field_of_view};

/// Component that tells the App this entity can view the world around it.
/// 
/// # Attributes
/// - `visible_tiles`   (Vec::<Position>)
/// - `range`           (i32)
/// - `dirty`           (bool)
#[derive(Component, Clone, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec::<Position>, 
    pub range: i32,
    pub dirty: bool
}

/// This ECS system determines which entities and tiles are visible to each entity that has
/// the Viewshed component attached to it.
/// 
/// Special considerations taken for the player entity, as any tiles visible to the player
/// should be added to the revealed tiles list contained by the map. This ties into the
/// render system that checks revealed tiles for what it should and shouldn't display.
pub fn visibility_system(
    mut map: ResMut<Map>,
    //vis_blockers: Query<(Entity, BlocksVisibility)>,
    q_player: Query<Entity, With<Player>>,
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