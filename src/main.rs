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
use systems::ai::default_move_system::{MoveMode, Movement, default_move_ai_system};
pub mod player;
pub use player::*;

use crate::systems::visibility_system::visibility_system;
#[macro_use]
extern crate lazy_static;

#[derive(States, Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub enum RunState {
    #[default]
    AwaitingInput,
    Ticking,
    Animating,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins, EntropyPlugin::<WyRand>::default()))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_state(RunState::AwaitingInput)
        .add_message::<WantsToMove>()
        .add_systems(Startup,
            (
                load_npc_definitions,
                setup,
                spawn_villager,
            ).chain(),)
        .add_systems(Update, handle_input.run_if(in_state(RunState::AwaitingInput)))
        .add_systems(Update, movement_system.run_if(in_state(RunState::Ticking)))
        .add_systems(Update, render)
        .add_systems(Update, visibility_system)
        .add_systems(Update, default_move_ai_system.run_if(in_state(RunState::Ticking)))
        .run();
}

fn setup(mut commands: Commands, mut global_rng: GlobalRngEntity<WyRand>) {
    commands.spawn(Terminal::new([160, 100])
            .with_border(BoxStyle::SINGLE_LINE)
    );
    commands.spawn(TerminalCamera::new());
    let mut builder = test_builder(0, 160, 100);
    builder.build_map();
    let start_pos = builder.build_data.starting_position.unwrap();
    commands.spawn(PlayerBundle::new(start_pos));
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
        MoveMode { mode: villager.movement.clone() },
    ));
}