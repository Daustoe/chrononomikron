use bevy::prelude::*;
use std::collections::VecDeque;
use crate::{Player, RunState};

#[derive(Component)]
pub struct Speed {
    pub speed: i32
}

#[derive(Component)]
pub struct Energy {
    pub energy: i32
}

#[derive(Component, Debug, Default)]
pub struct Actor;

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

fn setup (mut commands: Commands) {
    let manager = TimeManager::new();
    commands.insert_resource(manager);
}

pub fn time_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.init_resource::<TimeManager>();
    app.init_resource::<Clock>();
    app.add_systems(Update, time_system.run_if(in_state(RunState::Ticking)));
}

pub fn time_system(
    mut commands: Commands,
    mut queue: ResMut<TimeManager>,
    query: Query<&Player>,
    mut run_state: ResMut<NextState<RunState>>,
    //mut clock: ResMut<Clock>,
    //mut actors: Query<(Entity, &Actor, Option<&Player>)>
) {
    let Some(entity) = queue.pop() else {
        return;
    };


    if let Ok(_player) = query.get(entity) {
        run_state.set(RunState::AwaitingInput);
    }
    commands.entity(entity).insert(MyTurn);
    
    queue.push(entity);
    
}