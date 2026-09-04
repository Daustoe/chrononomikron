use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
    println!("Hello, world!");
}

fn setup(mut commands: Commands) {
    commands.spawn(Terminal::from_rexpaint_file("assets/GUI-MAIN.xp")
            .unwrap()
            .with_border(BoxStyle::SINGLE_LINE)
            .with_title(" [<fg=4d65b4>Chrononomikron</fg>]"),
    );
    commands.spawn(TerminalCamera::new());
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
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