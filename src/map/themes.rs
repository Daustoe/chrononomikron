use super::{Map, TileType};
use bevy::color::LinearRgba;
use bevy_ascii_terminal::color::css::*;

/// This function determines what Themed tilesets to use for TileTypes
/// 
/// #Arguments
/// -`idx`: x*y index of the map
/// -`map`: reference to the `Map` resource.
/// 
/// #Returns
/// Tuple of:
/// -`char`: character glyph to be used to render
/// -`LinearRgba`: foreground RGB color
/// -`LinearRgba`: background RGB color
pub fn tile_glyph(idx: usize, map : &Map) -> (char, LinearRgba, LinearRgba) {
    let (glyph, mut fg, mut bg) = match map.depth {
        7 => {
            let x = idx as i32 % map.width;
            if x > map.width-16 {
                get_tile_glyph_default(idx, map)
            } else {
                get_mushroom_glyph(idx, map)
            }
        }
        5 => {
            let x = idx as i32 % map.width;
            if x < map.width/2 {
                get_limestone_cavern_glyph(idx, map)
            } else {
                get_tile_glyph_default(idx, map)
            }
        }
        4 => get_limestone_cavern_glyph(idx, map),
        3 => get_limestone_cavern_glyph(idx, map),
        2 => get_forest_glyph(idx, map),
        _ => get_tile_glyph_default(idx, map)
    };

    if map.bloodstains.contains(&idx) { bg = LinearRgba::rgb(0.75, 0.0, 0.0); }
    if !map.visible_tiles[idx] {
        fg = grayscale(fg);
        bg = LinearRgba::rgb(0.0, 0.0, 0.0); // Don't show stains out of visual range
    } //else if !map.outdoors {
        //fg = fg * map.light[idx];
        //bg = bg * map.light[idx];
    //}

    (glyph, fg, bg)
}

/// This function converts a LinearRgba color to grayscale.
/// 
/// #Arguments
/// `color`: LinearRgba to convert
/// 
/// #Returns
/// `LinearRgba`: color with greyscale transformation applied
fn grayscale(color: LinearRgba) -> LinearRgba {
    let luminance =
        0.2126 * color.red
        + 0.7152 * color.green
        + 0.0722 * color.blue;

    LinearRgba::new(luminance, luminance, luminance, color.alpha)
}

fn get_forest_glyph(idx:usize, map: &Map) -> (char, LinearRgba, LinearRgba) {
    let glyph;
    let fg;
    let bg = LinearRgba::rgb(0.0, 0.0, 0.0);

    match map.tiles[idx] {
        TileType::Wall => { glyph = '♣'; fg = LinearRgba::rgb(0.0, 0.6, 0.0); }
        TileType::Bridge => { glyph = '.'; fg = CHOCOLATE; }
        TileType::Road => { glyph = '≡'; fg = YELLOW; }
        TileType::Grass => { glyph = '"'; fg = GREEN; }
        TileType::ShallowWater => { glyph = '~'; fg = CYAN; }
        TileType::DeepWater => { glyph = '~'; fg = BLUE; }
        TileType::Gravel => { glyph = ';'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
        TileType::DownStairs => { glyph = '>'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::UpStairs => { glyph = '<'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        _ => { glyph = '"'; fg = LinearRgba::rgb(0.0, 0.7, 0.0); }
    }

    (glyph, fg, bg)
}

fn get_mushroom_glyph(idx:usize, map: &Map) -> (char, LinearRgba, LinearRgba) {
    let glyph;
    let fg;
    let bg = LinearRgba::rgb(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Wall => { glyph = '♠'; fg = LinearRgba::rgb(1.0, 0.0, 1.0); }
        TileType::Bridge => { glyph = '.'; fg = GREEN; }
        TileType::Road => { glyph = '≡'; fg = CHOCOLATE; }
        TileType::Grass => { glyph = '"'; fg = GREEN; }
        TileType::ShallowWater => { glyph = '~'; fg = CYAN; }
        TileType::DeepWater => { glyph = '~'; fg = BLUE; }
        TileType::Gravel => { glyph = ';'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
        TileType::DownStairs => { glyph = '>'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::UpStairs => { glyph = '<'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        _ => { glyph = '"'; fg = LinearRgba::rgb(0.0, 0.6, 0.0); }
    }

    (glyph, fg, bg)
}

fn get_limestone_cavern_glyph(idx:usize, map: &Map) -> (char, LinearRgba, LinearRgba) {
    let glyph;
    let fg;
    let bg = LinearRgba::rgb(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Wall => { glyph = '▒'; fg = LinearRgba::rgb(0.7, 0.7, 0.7); }
        TileType::Bridge => { glyph = '.'; fg = CHOCOLATE; }
        TileType::Road => { glyph = '≡'; fg = YELLOW; }
        TileType::Grass => { glyph = '"'; fg = GREEN; }
        TileType::ShallowWater => { glyph = '░'; fg = CYAN; }
        TileType::DeepWater => { glyph = '▓'; fg = LinearRgba::rgb(0.2, 0.2, 1.0); }
        TileType::Gravel => { glyph = ';'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
        TileType::DownStairs => { glyph = '>'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::UpStairs => { glyph = '<'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::Stalactite => { glyph = '╨'; fg = LinearRgba::rgb(0.7, 0.7, 0.7); }
        TileType::Stalagmite => { glyph = '╥'; fg = LinearRgba::rgb(0.7, 0.7, 0.7); }
        _ => { glyph = '\''; fg = LinearRgba::rgb(0.4, 0.4, 0.4); }
    }

    (glyph, fg, bg)
}

fn get_tile_glyph_default(idx: usize, map : &Map) -> (char, LinearRgba, LinearRgba) {
    let glyph;
    let fg;
    let bg = LinearRgba::rgb(0., 0., 0.);

    match map.tiles[idx] {
        TileType::Floor => { glyph = '.'; fg = LinearRgba::rgb(0.0, 0.5, 0.5); }
        TileType::WoodFloor => { glyph = '░'; fg = CHOCOLATE; }
        TileType::Wall => {
            let x = idx as i32 % map.width;
            let y = idx as i32 / map.width;
            glyph = wall_glyph(&*map, x, y);
            fg = LinearRgba::rgb(0., 1.0, 0.);
        }
        TileType::DownStairs => { glyph = '>'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::UpStairs => { glyph = '<'; fg = LinearRgba::rgb(0., 1.0, 1.0); }
        TileType::Bridge => { glyph = '.'; fg = CHOCOLATE; }
        TileType::Road => { glyph = '≡'; fg = GRAY; }
        TileType::Grass => { glyph = '"'; fg = GREEN; }
        TileType::ShallowWater => { glyph = '~'; fg = CYAN; }
        TileType::DeepWater => { glyph = '~'; fg = BLUE; }
        TileType::Gravel => { glyph = ';'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
        TileType::Stalactite => { glyph = '╨'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
        TileType::Stalagmite => { glyph = '╥'; fg = LinearRgba::rgb(0.5, 0.5, 0.5); }
    }

    (glyph, fg, bg)
}

#[allow(unused_variables)]
fn wall_glyph(map : &Map, x: i32, y:i32) -> char {

    // if x < 1 || x >= map.width-2 || y < 1 || y >= map.height-2 as i32 { return '#'; }
    // let mut mask : u8 = 0;

    // if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    // if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    // if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    // if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    // match mask {
    //     0 => { 'I' } // Pillar because we can't see neighbors
    //     1 => { '║' } // Wall only to the north
    //     2 => { '║' } // Wall only to the south
    //     3 => { '║' } // Wall to the north and south
    //     4 => { '═' } // Wall only to the west
    //     5 => { '╝' } // Wall to the north and west
    //     6 => { '╗' } // Wall to the south and west
    //     7 => { '╣' } // Wall to the north, south and west
    //     8 => { '═' } // Wall only to the east
    //     9 => { '╚' } // Wall to the north and east
    //     10 => { '╔' } // Wall to the south and east
    //     11 => { '╠' } // Wall to the north, south and east
    //     12 => { '═' } // Wall to the east and west
    //     13 => { '╩' } // Wall to the east, west, and south
    //     14 => { '╦' } // Wall to the east, west, and north
    //     15 => { '╬' }  // ╬ Wall on all sides
    //     _ => { '#' } // We missed one?
    // }
    return '#'
}

#[allow(dead_code)]
fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}