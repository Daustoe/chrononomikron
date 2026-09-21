use bevy::prelude::*;
use crate::{ActingEntityBundle, Position, Health, Mana, BlocksTile};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player);
    }
}

/// Command spawns the default Player Bundle.
fn spawn_player(
    mut commands: Commands
) {
    commands.spawn(PlayerBundle::default());
}

/// This Component informs us which Entity is the Player entity.
#[derive(Component, Clone, Debug)]
pub struct Player {}

/// These Components represent what should be attached by default to a Player Entity.
#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub actor_bundle: ActingEntityBundle,
    pub player: Player, 
    pub health: Health,
    pub mana: Mana,
    pub blocks: BlocksTile
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            actor_bundle: ActingEntityBundle::new(LinearRgba::WHITE, '@'),
            player: Player {},
            health: Health { health: 10 },
            mana: Mana { mana: 0 },
            blocks: BlocksTile {}
        }
    }
}

impl PlayerBundle {
    pub fn new(start_position: Position) -> Self {
        let mut actingbundle = ActingEntityBundle::new(LinearRgba::WHITE, '@');
        actingbundle.position = start_position;
        Self {
            actor_bundle: actingbundle,
            player: Player {},
            health: Health { health: 10 },
            mana: Mana { mana: 0 },
            blocks: BlocksTile {}
        }
    }
}