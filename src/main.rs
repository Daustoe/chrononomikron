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
mod spawners;
use spawners::asset_loader::*;
use spawners::player::*;
use spawners::{spawn_system, SpawnNpc};
use systems::TimeManager;
use systems::ai::default_move_system::{MoveMode, Movement, default_move_ai_system};
use systems::time_plugin;

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
        .add_plugins((DefaultPlugins, TerminalPlugins, EntropyPlugin::<WyRand>::default(), time_plugin))
        .insert_resource(ClearColor(Color::BLACK))
        //.init_resource::<TimeManager>()
        .insert_state(RunState::AwaitingInput)
        .add_message::<WantsToMove>()
        .add_message::<SpawnNpc>()
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
        .add_systems(Update, spawn_system)
        .run();
}

fn setup(mut commands: Commands, mut global_rng: GlobalRngEntity<WyRand>, mut queue: ResMut<TimeManager>) {
    commands.spawn(Terminal::new([160, 100])
            .with_border(BoxStyle::SINGLE_LINE)
    );
    commands.spawn(TerminalCamera::new());
    let mut builder = test_builder(0, 160, 100);
    builder.build_map();
    let start_pos = builder.build_data.starting_position.unwrap();
    let player_entity = commands.spawn(PlayerBundle::new(start_pos)).id();
    queue.push(player_entity);
    commands.insert_resource(builder.build_data.map);
    commands.queue(|world: &mut World| {
        world.write_message(SpawnNpc { position: Position {x: 80, y: 50}, def_key: NPC::Villager });
    });
}

fn spawn_villager(
    mut commands: Commands,
    definitions: Res<NpcDefinitions>
) {
    let Some(_villager) = definitions.npcs.get(&NPC::Villager)
    else {
        return;
    };

    commands.queue(|world: &mut World| {
        world.write_message(SpawnNpc { position: Position {x: 80, y: 50}, def_key: NPC::Villager });
    });
}