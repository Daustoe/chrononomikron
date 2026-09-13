use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::{Position, Player};
use super::WantsToMove;

pub fn handle_input(
    mut q_player: Query<(Entity, &mut Position), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
    mut wants_move: MessageWriter<WantsToMove>
) {
    if let Ok((player_entity, pos)) = q_player.single_mut(){
        let mut new_pos = pos.clone();
        if input.just_pressed(KeyCode::Numpad1) {
            new_pos.x -= 1;
            new_pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad2) {
            new_pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad3) {
            new_pos.x += 1;
            new_pos.y += 1;
        }
        if input.just_pressed(KeyCode::Numpad4) {
            new_pos.x -= 1;
        }
        if input.just_pressed(KeyCode::Numpad5) {
            ();
        }
        if input.just_pressed(KeyCode::Numpad6) {
            new_pos.x += 1;
        }
        if input.just_pressed(KeyCode::Numpad7) {
            new_pos.x -= 1;
            new_pos.y -= 1;
        }
        if input.just_pressed(KeyCode::Numpad8) {
            new_pos.y -= 1;
        }
        if input.just_pressed(KeyCode::Numpad9) {
            new_pos.x += 1;
            new_pos.y -= 1
        }

        if new_pos != *pos {
            wants_move.write(WantsToMove{entity: player_entity, destination: new_pos});
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