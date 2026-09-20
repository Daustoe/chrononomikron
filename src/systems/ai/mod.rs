pub mod default_move_system;
pub mod adjacent_ai_system;
pub mod approach_ai_system;
pub mod chase_ai_system;
pub mod flee_ai_system;
pub mod visible_ai_system;

use approach_ai_system::*;
pub use chase_ai_system::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Reaction {
    Ignore,
    Attack,
    Flee
}