use bevy::prelude::*;
use crate::{ActingEntityBundle, Position};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn(PlayerBundle::default());
}

#[derive(Component, Clone, Debug)]
pub struct Player {}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub actor_bundle: ActingEntityBundle,
    pub player: Player
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            actor_bundle: ActingEntityBundle::new(LinearRgba::WHITE, '@'),
            player: Player {}
        }
    }
}

impl PlayerBundle {
    pub fn new(start_position: Position) -> Self {
        let mut actingbundle = ActingEntityBundle::new(LinearRgba::WHITE, '@');
        actingbundle.position = start_position;
        Self {
            actor_bundle: actingbundle,
            player: Player {}
        }
    }
}