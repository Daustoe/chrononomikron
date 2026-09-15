use bevy::prelude::*;
use std::collections::VecDeque;

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

#[derive(Component, Debug)]
pub struct MyTurn;

#[derive(Resource, Debug)]
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
    app.add_systems(Update, time_system);
}

pub fn time_system(
    mut time_manager: ResMut<TimeManager>
) {
    
}