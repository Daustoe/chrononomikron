use bevy::prelude::*;
use crate::{Position, Health, Player};

#[derive(Component)]
pub struct OnDeath {}

pub fn death_system(
    mut commands: Commands, 
    q_entities: Query<(Entity, &Position, &Health)>,
    q_player: Query<Entity, With<Player>>,
) {
    // Collect list of entities that are dying
    let mut dead: Vec<Entity> = Vec::new();
    for (entity, _pos, health) in q_entities.iter() {
        if health.health < 1 {
            let player_entity = q_player.single().unwrap();
            if player_entity == entity {
                println!("You Die!");
                // TODO: Implement game over screen
            } else {
                dead.push(entity);
            }
        }
    }

    for victim in dead {
        commands.entity(victim).despawn();
    }
}