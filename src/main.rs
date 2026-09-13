use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_rand::prelude::*;

mod components;
use components::*;
mod map;
pub use map::*;
pub mod map_builders;
use map_builders::*;
pub mod constants;
pub mod rect;
pub mod rng;
mod systems;
use systems::input_system::*;
use systems::movement_system::*;
use systems::render_system::render;
mod asset_loader;
use asset_loader::*;

use crate::systems::visibility_system::visibility_system;
#[macro_use]
extern crate lazy_static;

#[derive(States, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    Ticking,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range : i32, item : Entity},
    MainMenu,
    SaveGame,
    NextLevel,
    PreviousLevel,
    TownPortal,
    ShowRemoveItem,
    GameOver,
    MagicMapReveal { row : i32 },
    MapGeneration,
    ShowCheatMenu,
    //ShowVendor { vendor: Entity, mode : VendorMode },
    TeleportingToOtherLevel { x: i32, y: i32, depth: i32 },
    ShowRemoveCurse,
    ShowIdentify
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins, EntropyPlugin::<WyRand>::default()))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_state(RunState::MainMenu)
        .add_message::<WantsToMove>()
        .add_systems(Startup,
            (
                load_npc_definitions,
                setup,
                spawn_villager,
            ).chain(),)
        .add_systems(Update, handle_input)
        .add_systems(Update, movement_system)
        .add_systems(Update, render)
        .add_systems(Update, visibility_system)
        .run();
}

fn setup(mut commands: Commands, mut global_rng: GlobalRngEntity<WyRand>) {
    commands.spawn(Terminal::new([160, 100])
            .with_border(BoxStyle::SINGLE_LINE)
            //.with_title(" [<fg=4d65b4>Chrononomikron</fg>]"),
    );
    commands.spawn(TerminalCamera::new());
    let mut builder = random_builder(0, 160, 100);
    builder.build_map();
    let start_pos = builder.build_data.starting_position.unwrap();
    commands.spawn((
        start_pos,
        Renderable {
            glyph: '@',
            fg: color::css::YELLOW,
            bg: color::css::BLACK
        },
        Player {},
        Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        },
    ));
    commands.insert_resource(builder.build_data.map);
}

fn spawn_villager(
    mut commands: Commands,
    definitions: Res<NpcDefinitions>
) {
    let Some(villager) = definitions.npcs.get("villager")
    else {
        return;
    };

    let Some(renderable) = &villager.renderable else {
        return;
    };

    commands.spawn((
        Position {x: 80, y: 50},
        *renderable,
    ));
}