/// This enum defines the different Tile Types we can encounter on our `Map`.
/// 
/// Each type might have different attributes, i.e. a `Wall` is impassable while `Grass` can be walked on.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Stalactite,
    Stalagmite, 
    Floor, 
    DownStairs,
    Road,
    Grass,
    ShallowWater,
    DeepWater,
    WoodFloor,
    Bridge, 
    Gravel, 
    UpStairs
}

/// Determines which TileTypes can be walked on.
pub fn tile_walkable(tt: TileType) -> bool {
    match tt {
        TileType::Floor | TileType::DownStairs | TileType::Road | TileType::Grass | TileType::ShallowWater |
        TileType::WoodFloor | TileType::Bridge | TileType::Gravel | TileType::UpStairs
            => true,
        _ => false
    }
}

/// Determines which TileTypes an Entity with a Viewshed can see through.
pub fn tile_opaque(tt: TileType) -> bool {
    match tt {
        TileType::Wall  | TileType::Stalactite | TileType::Stalagmite => true,
        _ => false
    }
}

/// Determines walking movement cost muliplier on TileTypes.
pub fn tile_cost(tt: TileType) -> f32 {
    match tt {
        TileType::Road => 0.8,
        TileType::Grass => 1.1,
        TileType::ShallowWater => 1.2,
        _ => 1.0
    }
}