// General control constants
pub const SHOW_FPS: bool = true;
pub const SHOW_MAPGEN_VISUALIZER: bool = true;
pub const SHOW_BOUNDARIES: bool = false;

// GUI/HUD location constants
pub const CAMERA_OFFSET: i32 = 30;
pub const DISPLAY_WIDTH: i32 = 160;
pub const DISPLAY_HEIGHT: i32 = 100;
pub const RIGHT_HUD_OFFSET: i32 = DISPLAY_WIDTH-CAMERA_OFFSET-1;
pub const RIGHT_HUD_INFO: i32 = RIGHT_HUD_OFFSET+1;
pub const BOTTOM_HUD_OFFSET: i32 = DISPLAY_HEIGHT-20;
pub const HUD_MAP_WIDTH: i32 = DISPLAY_WIDTH-(CAMERA_OFFSET*2)-1;
pub const HUD_MAP_HEIGHT: i32 = BOTTOM_HUD_OFFSET - 1;

// Health and Mana HUG constants
pub const HEALTH_X: i32 = (DISPLAY_WIDTH / 2) - 28; // odd adjustment based on size of asset.
pub const HEALTH_Y: i32 = BOTTOM_HUD_OFFSET - 2;    // object draw from top left corner
pub const MANA_X: i32 = (DISPLAY_WIDTH / 2) + 17;
pub const MANA_Y: i32 = BOTTOM_HUD_OFFSET - 2;
pub const ATT_X: i32 = CAMERA_OFFSET + 5;
pub const LVL_X: i32 = (DISPLAY_WIDTH / 2) - 12;
pub const INI_X: i32 = 121;
pub const STATUS_X: i32 = 109;

// Event Logger constants
pub const MAX_LOG_DISPLAY: usize = 30;

// Menu location constants
pub const MENU_LEFT_BOUNDS: i32 = CAMERA_OFFSET + 15;
pub const MENU_RIGHT_BOUNDS: i32 = RIGHT_HUD_OFFSET - 15;