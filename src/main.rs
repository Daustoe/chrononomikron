use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
use bevy_rand::prelude::*;

mod components;
use components::*;
mod map;
pub use map::*;
pub mod spatial;
pub mod map_builders;
use map_builders::*;
pub mod constants;
pub mod rect;
pub mod rng;
mod systems;
use systems::input_system::*;
use systems::movement_system::*;
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
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, movement_system)
        .add_systems(Update, render)
        .run();
}

fn setup(mut commands: Commands, mut global_rng: GlobalRngEntity<WyRand>) {
    commands.spawn(Terminal::new([160, 100])
            .with_border(BoxStyle::SINGLE_LINE)
            //.with_title(" [<fg=4d65b4>Chrononomikron</fg>]"),
    );
    commands.spawn(TerminalCamera::new());
    commands.spawn((
        Position {
            x: 80,
            y: 50,
        },
        Renderable {
            glyph: '@',
            fg: color::css::YELLOW,
            bg: color::css::BLACK
        },
        Player {}
    ));
    let mut builder = random_builder(0, 160, 100);
    //builder.start_with(SimpleMapBuilder::new());
    //builder.with(StartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.build_map();
    commands.insert_resource(builder.build_data.map);

    //global_rng.rng_commands().with_target_rngs(targets)
}


fn render(
    mut q_term: Query<&mut Terminal>,
    _q_player: Query<Entity, With<Player>>,
    map: Res<Map>,
    q_entities: Query<(&Renderable, &Position)>
) {
    let mut term = match q_term.single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    term.clear();
    term.set_pivot(Pivot::LeftTop);

    for x in 0..map.width {
        for y in 0..map.height {
            let tile_data = tile_glyph(map.xy_idx(x, y), &map);
            let Some(tile) = term.try_tile_mut(IVec2::from_array([x, y])) else {
                continue;
            };
            tile.glyph = tile_data.0;
            tile.fg_color = tile_data.1;
            tile.bg_color = tile_data.2;
        }
    }

    for (r, pos) in q_entities.iter() {
        let Some(tile) = term.try_tile_mut(IVec2::from_array([pos.x, pos.y])) else {
            continue;
        };
        tile.glyph = r.glyph;
        tile.fg_color = r.fg;
        tile.bg_color = r.bg;
    }
}