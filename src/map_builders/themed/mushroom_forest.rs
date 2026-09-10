use super::{BuilderChain, XStart, YStart, StartingPosition, CullUnreachable, EndingPosition, XEnd, YEnd, CellularAutomataBuilder, PrefabBuilder, WaveformCollapseBuilder};
use crate::map_builders::prefab_builder::prefab_sections::{UNDERGROUND_FORT, DROW_ENTRY};

pub fn mushroom_entrance(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into The Mushroom Grove");
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(StartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(EndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    //chain.with(VoronoiSpawning::new());
    chain.with(PrefabBuilder::sectional(UNDERGROUND_FORT));
    chain
}

pub fn mushroom_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into The Mushroom Grove");
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(StartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(EndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    //chain.with(VoronoiSpawning::new());
    chain
}

pub fn mushroom_exit(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into The Mushroom Grove");
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(StartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(EndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    //chain.with(VoronoiSpawning::new());
    chain.with(PrefabBuilder::sectional(DROW_ENTRY));
    chain
}