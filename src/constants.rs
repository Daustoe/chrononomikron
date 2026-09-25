use bevy::prelude::UVec2;
// General control constants
pub const SHOW_FPS: bool = true;
pub const SHOW_MAPGEN_VISUALIZER: bool = true;
pub const SHOW_BOUNDARIES: bool = false;
pub const REVEAL_ALL: bool = true;

// Event Logger constants
pub const MAX_LOG_DISPLAY: usize = 30;

// Window and Camera constants
pub const MAP_UI_DIMENSIONS: UVec2 = UVec2::new(50, 50);
pub const VIEWPORT_SIZE: [u32;2] = [160, 90];