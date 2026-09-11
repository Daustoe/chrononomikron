use super::{MetaMapBuilder, BuilderMap, TileType};

/// This is another test documentation line.
pub struct DistantExit {}

impl MetaMapBuilder for DistantExit {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

/// Distant Exit is an implementation of [`MetaMapBuilder`] that allows the user to add
/// an Exit Tile at the furthest distance from a starting point.
/// 
/// #Examples
/// ```
/// let mut builder = BuilderChain::new(new_depth, width, height, "Test Map");
/// builder.start_with(DrunkardsWalkBuilder::open_area());
/// builder.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
/// builder.with(DistantExit::new());
/// builder.build_map();
/// ```
impl DistantExit {
    #[allow(dead_code)]
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        let starting_pos = build_data
            .starting_position
            .as_ref()
            .expect("DistanceExit requires starting_position! Add a starting_position MetaBuilder to your BuilderMap.")
            .clone();
        let start_idx = build_data.map.xy_idx(
            starting_pos.x, 
            starting_pos.y
        );
        build_data.map.populate_blocked();
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = rltk::DijkstraMap::new(build_data.map.width as usize, build_data.map.height as usize, &map_starts , &build_data.map, 1000.0);
        let mut exit_tile = (0, 0.0f32);
        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start != std::f32::MAX {
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        let stairs_idx = exit_tile.0;
        build_data.map.tiles[stairs_idx] = TileType::DownStairs; // Exit found and replaced with DownStairs
        // There may be a future where we want to have this TileType be variable!
        build_data.take_snapshot();
    }
}