use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

mod components;
use components::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, render)
        .run();
    println!("Hello, world!");
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
            pos.y -= 1;
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
    q_player: Query<Entity, With<Player>>,
    q_entities: Query<(&Renderable, &Position)>
) {
    let mut term = match q_term.single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    term.clear();
    term.set_pivot(Pivot::LeftTop);

    for (r, pos) in q_entities.iter() {
        let Some(tile) = term.try_tile_mut(pos) else {
            continue;
        }
    }
}