use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy::camera::Viewport;
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
use constants::*;

use crate::systems::ai::{
    adjacent_ai_system::*,
    chase_ai_system::*,
    approach_ai_system::*,
    visible_ai_system::*,
};
//use crate::systems::{ai::approach_ai_system::approach_ai_system, visibility_system::visibility_system};
#[macro_use]
extern crate lazy_static;

#[derive(Component)]
struct MapTerminal;

#[derive(Component)]
struct LogTerminal;

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
        .add_plugins((
            DefaultPlugins, 
            TerminalPlugins, 
            EntropyPlugin::<WyRand>::default()
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(TerminalMeshWorldScaling::World)
        //.init_resource::<TimeManager>()
        .insert_state(RunState::default())
        .init_resource::<TimeManager>()
        .add_message::<WantsToMove>()
        .add_message::<SpawnNpc>()
        .add_message::<Damage>()
        .add_systems(Startup,
            (
                load_npc_definitions,
                setup,
                index_player,
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
                death_system,
                spawn_system
            ).chain(),
        )
        .add_systems(PostUpdate, render)
        .run();
}

fn setup(
    mut commands: Commands, 
    window: Single<&Window>,
) {
    let window_size = window.physical_size().as_vec2();

    println!("Window size: {}", window_size);

    

    commands.spawn((
        Terminal::new(MAP_UI_DIMENSIONS),
            //.with_border(BoxStyle::DOUBLE_LINE),
        TerminalMeshPivot::LeftTop,
        MapTerminal,
        Transform::from_xyz(200.0, 100.0, 0.0).with_translation(Vec3::new(0.0, 0.0, 0.0)),

        
        //SetTerminalGridPosition(IVec2::new(0, 0))
    ));

    commands.spawn((
        Terminal::new([80, 10]).with_title("Console Log"),
        //TerminalMeshPivot::LeftTop,
        LogTerminal,
        //Transform::from_xyz(0.0, 35.0, 0.0),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: window_size.as_uvec2(),
                ..default()
            }),
            ..default()
        },
        TerminalCamera::default(),
    ));
   
    let mut builder = test_builder(0, 160, 90);
    builder.build_map();
    let start_pos = builder.build_data.starting_position.unwrap();
    println!("Starting Pos: {:?}", start_pos);
    commands.spawn(PlayerBundle::new(start_pos));
    commands.insert_resource(builder.build_data.map);
}

fn index_player(
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut queue: ResMut<TimeManager>,
    mut q_player: Query<(Entity, &Position), With<Player>>,
    mut state: ResMut<NextState<RunState>>,
    map_term: Query<&Terminal, With<MapTerminal>>
) {
    //map_term.
    let (player_entity, pos) = q_player.single_mut().unwrap();
    queue.push(player_entity);
    
    map.index_entity(player_entity, *pos, true);
    commands.queue(|world: &mut World| {
        world.write_message(SpawnNpc { position: Position {x: 80, y: 30}, def_key: NPC::Villager });
    });
    state.set(RunState::PlayerTurn);

}