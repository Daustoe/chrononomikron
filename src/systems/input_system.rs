use bevy::prelude::*;
use bevy_ascii_terminal::TerminalCamera;
use crate::{Position, Player, RunState, SpawnNpc, NPC, Map, get_screen_bounds};
use super::WantsToMove;

/// This system handles input from the player.
/// 
/// There are many avenues that a player choice could take, so this may need to be expanded
/// upon greatly in the future. For now, it will handle player movement and closing out
/// the game window. 
pub fn handle_input(
    mut q_player: Query<(Entity, &Position), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut exit: MessageWriter<AppExit>,
    mut wants_move: MessageWriter<WantsToMove>,
    mut next_state: ResMut<NextState<RunState>>,
    mut spawn: MessageWriter<SpawnNpc>,
    windows: Query<&Window>,
    camera_q: Query<(&TerminalCamera, &Transform, &GlobalTransform)>,
    mut map: ResMut<Map>,
) {
    if let Some(key) = kb_input.get_just_pressed().next() {
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

    if mouse_input.just_released(MouseButton::Left) {
        let window = windows.single().unwrap();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        println!("Cursor Position: {}", cursor_position);
        let (terminal_camera, transf, g_transf) = camera_q.single().unwrap();
        let (player_entity, player_pos) = q_player.single_mut().unwrap();

        println!("transform: {:?}", transf);
        //println!("global transform: {:?}", g_transf);
        //println!("camera: {:?}", camera);
        println!("viewport pos: {:?}", terminal_camera.cursor_viewport_pos());
        
        if let Some(grid_position) = terminal_camera.cursor_world_pos() {
            println!("Player Pos: {:?}", player_pos);
            println!("grid pos: {:?}", grid_position);
            let (min_x, max_x, min_y, max_y) = get_screen_bounds(*player_pos);
            println!("[{},{}] [{},{}]", min_x, min_y, max_x, max_y);
            let mapped_pos_x = grid_position.x as i32 + min_x;
            let mapped_pos_y = (50 - grid_position.y as i32) + min_y;
            println!("Mapped positions: [{},{}]", mapped_pos_x, mapped_pos_y);
            let idx = map.xy_idx(mapped_pos_x, mapped_pos_y);
            //map.for_each_tile_content(idx, |other_entity| {
                //println!("Entity found: {:?}", other_entity);
            //});
            println!("World Pos: {:?}", grid_position);

        }
    }
    if mouse_input.just_released(MouseButton::Right) {
        println!("Right button pressed");
    }
}