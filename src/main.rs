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
use systems::*;
mod spawners;
pub use spawners::*;

use crate::systems::ai::{
    adjacent_ai_system::*,
    chase_ai_system::*,
    approach_ai_system::*,
    visible_ai_system::*,
};
//use crate::systems::{ai::approach_ai_system::approach_ai_system, visibility_system::visibility_system};
#[macro_use]
extern crate lazy_static;

#[derive(States, Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
pub enum RunState {
    #[default]
    Setup,
    PlayerTurn,
    NextTurn,
    Ticking,
    Animating,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins, EntropyPlugin::<WyRand>::default()))
        .insert_resource(ClearColor(Color::BLACK))
        //.init_resource::<TimeManager>()
        .insert_state(RunState::default())
        .init_resource::<TimeManager>()
        .add_message::<WantsToMove>()
        .add_message::<SpawnNpc>()
        .add_message::<Damage>()
        .add_systems(Startup,
            (
                load_npc_definitions,
                setup
            ).chain(),)
        .add_systems(
            Update,
            (
                time_system.run_if(in_state(RunState::Ticking)),
                visibility_system,
                handle_input.run_if(in_state(RunState::PlayerTurn)),
                adjacent_ai_system.run_if(in_state(RunState::NextTurn)),
                visible_ai_system.run_if(in_state(RunState::NextTurn)),
                approach_ai_system.run_if(in_state(RunState::NextTurn)),
                chasing_ai_system.run_if(in_state(RunState::NextTurn)),
                default_move_ai_system.run_if(in_state(RunState::NextTurn)),
                movement_system,
                apply_damage_system,
                spawn_system
            ).chain(),
        )
        .add_systems(Update, render)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut queue: ResMut<TimeManager>,
    mut state: ResMut<NextState<RunState>>
) {
    commands.spawn(Terminal::new([80, 50])
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
        world.write_message(SpawnNpc { position: Position {x: 80, y: 40}, def_key: NPC::Villager });
    });
    state.set(RunState::PlayerTurn);
}