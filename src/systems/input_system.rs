use bevy::prelude::*;
use crate::{Position, Player, RunState, SpawnNpc, NPC};
use super::WantsToMove;

/// This system handles input from the player.
/// 
/// There are many avenues that a player choice could take, so this may need to be expanded
/// upon greatly in the future. For now, it will handle player movement and closing out
/// the game window. 
pub fn handle_input(
    mut q_player: Query<(Entity, &Position), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
    mut wants_move: MessageWriter<WantsToMove>,
    mut next_state: ResMut<NextState<RunState>>,
    mut spawn: MessageWriter<SpawnNpc>
) {
    if let Some(key) = input.get_just_pressed().next() {
        let (player_entity, pos) = q_player.single_mut().unwrap();
        let mut new_pos = Position {x: pos.x, y: pos.y};
        match key {
            KeyCode::Numpad1 => { 
                new_pos.x -= 1;
                new_pos.y += 1;
            },
            KeyCode::Numpad2 => {
                new_pos.y += 1;
            },
            KeyCode::Numpad3 => {
                new_pos.x += 1;
                new_pos.y += 1;
            },
            KeyCode::Numpad4 => {
                new_pos.x -= 1;
            },
            KeyCode::Numpad6 => {
                new_pos.x += 1;
            },
            KeyCode::Numpad7 => {
                new_pos.x -= 1;
                new_pos.y -= 1;
            },
            KeyCode::Numpad8 => {
                new_pos.y -= 1;
            },
            KeyCode::Numpad9 => {
                new_pos.x += 1;
                new_pos.y -= 1;
            },
            KeyCode::Escape => {
                exit.write(AppExit::Success);
            },
            KeyCode::KeyS => {
                let s_pos = Position {x: 80, y: 50};
                spawn.write(SpawnNpc{position: s_pos, def_key: NPC::Villager});
            },
            _ => ()
        }
        if new_pos != *pos {
            wants_move.write(WantsToMove{entity: player_entity, destination: new_pos});
            next_state.set(RunState::Ticking);
        }
    }
}