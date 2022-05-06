pub mod models;
pub mod system;

pub const GRID_SIZE: f32 = 16.0;

// Target 60fps
pub const TIME_STEP: f32 = 1.0 / 60.0;

// Window props
pub const WINDOW_WIDTH: f32 = 15.0 * 16.0;
pub const WINDOW_HEIGHT: f32 = 15.0 * 16.0;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
