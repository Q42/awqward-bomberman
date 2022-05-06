pub mod models;
pub mod system;

pub const GRID_SIZE: f32 = 16.0;

// Target 60fps
pub const TIME_STEP: f32 = 1.0 / 60.0;

// Window props
pub const WINDOW_WIDTH: f32 = 15.0 * 16.0;
pub const WINDOW_HEIGHT: f32 = 15.0 * 16.0;

pub const E: usize = 0; // Edge
pub const W: usize = 1; // Wall
pub const S: usize = 2; // Shaded grass
pub const G: usize = 3; // Grass
pub const BOMB: usize = 4;

pub const LAYER_BACKGROUND: f32 = 0.0;
pub const LAYER_ITEMS: f32 = 1.0;
pub const LAYER_PLAYER: f32 = 2.0;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
