use bevy::prelude::*;
use crate::{Position, };

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
    //mut query: Query<(&mut Health, &Resistances)>,
) {
    for damage in messages.read() {
        println!("{:?} wants to attack {:?}", damage.source.unwrap(), damage.target);
    }
}