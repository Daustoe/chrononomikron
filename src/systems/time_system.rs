use bevy::prelude::*;
use std::collections::VecDeque;
use crate::{Player, RunState};

#[allow(dead_code)]
#[derive(Component)]
pub struct Speed {
    pub speed: i32
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Energy {
    pub energy: i32
}

#[allow(dead_code)]
#[derive(Resource, Default)]
struct Clock {
    now: i32,
}

#[derive(Component, Debug)]
pub struct MyTurn;

#[derive(Resource, Debug, Default)]
pub struct TimeManager {
    pub queue: VecDeque<Entity>
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            queue: VecDeque::new()
        }
    }

    pub fn push(&mut self, entity: Entity) {
        self.queue.push_back(entity);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        let entity = self.queue.pop_front()?;
        self.queue.push_back(entity);
        Some(entity)
    }

    pub fn remove(&mut self, entity: Entity) {
        self.queue.retain(|e| *e != entity);
    }
}

pub fn time_system(
    mut commands: Commands,
    mut queue: ResMut<TimeManager>,
    player_query: Query<Entity, With<Player>>,
    mut run_state: ResMut<NextState<RunState>>,
    //mut clock: ResMut<Clock>,
    //mut actors: Query<(Entity, &Actor, Option<&Player>)>
) {
    let Some(entity) = queue.pop() else {
        return;
    };
    if entity == player_query.single().expect("No Player in ECS!"){
        run_state.set(RunState::PlayerTurn);
    } else {
        run_state.set(RunState::NextTurn)
    }
    commands.entity(entity).insert(MyTurn);  
}