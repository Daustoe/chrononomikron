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
        .add_plugins((DefaultPlugins, TerminalPlugins, EntropyPlugin::<WyRand>::new()))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_state(RunState::MainMenu)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, render)
        .run();
}

fn setup(mut commands: Commands) {
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
}

fn handle_input(
    mut q_player: Query<(Entity, &mut Position), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Ok((_entity, mut pos)) = q_player.single_mut(){
        if input.just_pressed(KeyCode::Numpad1) {
            pos.x -= 1;
            pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad2) {
            pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad3) {
            pos.x += 1;
            pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad4) {
            pos.x -= 1;
        }
        if input.just_pressed(KeyCode::Numpad5) {
            ();
        }
        if input.just_pressed(KeyCode::Numpad6) {
            pos.x += 1;
        }
        if input.just_pressed(KeyCode::Numpad7) {
            pos.x -= 1;
            pos.y -= 1;
        }
        if input.just_pressed(KeyCode::Numpad8) {
            pos.y -= 1;
        }
        if input.just_pressed(KeyCode::Numpad9) {
            pos.x += 1;
            pos.y -= 1
        }
    }
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if win.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            win.mode = WindowMode::Windowed;
        } else {
            win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }
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