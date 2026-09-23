use bevy::prelude::*;
use crate::Health;

#[derive(Message)]
pub struct Damage {
    pub source: Option<Entity>,
    pub target: Entity,
    pub amount: i32,
    pub damage_type: DamageType,
}

#[derive(Clone, Copy)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
}

pub fn apply_damage_system (
    mut messages: MessageReader<Damage>,
    mut query: Query<&mut Health>,
) {
    for damage in messages.read() {
        let Ok(mut health) = query.get_mut(damage.target) else {
            continue;
        };
        health.health = (health.health - damage.amount).max(0);
        println!("{:?} attacks {:?} for {} damage.", damage.source.unwrap(), damage.target, damage.amount);
        println!("{:?} has {} health left", damage.target, health.health);
    }
}