pub mod models;
pub mod system;

// Target 60fps
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
