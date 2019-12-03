//! Handles GUI actions.

use piston_window::{
    G2d,
    rectangle,
};

/// Displays the static selector separator and background at the bottom of the window. Only called once but refactored for readability.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
pub fn display_selector(
    window: &mut G2d,
    transform: &[[f64; 3]; 2]
) {

    const WINDOW_WIDTH: f64 = 800.0;

    const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const SEPARATOR_HORIZONTAL_POSITION: f64 = 0.0;
    const SEPARATOR_VERTICAL_POSITION: f64 = 500.0;
    const SEPARATOR_HEIGHT: f64 = 1.0;
    rectangle(
        WHITE_COLOR,
        [
            SEPARATOR_HORIZONTAL_POSITION,
            SEPARATOR_VERTICAL_POSITION,
            WINDOW_WIDTH,
            SEPARATOR_HEIGHT,
        ],
        *transform,
        window,
    );
    
    const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const SELECTOR_HORIZONTAL_POSITION: f64 = 0.0;
    const SELECTOR_VERTICAL_POSITION: f64 = 501.0;
    const SELECTOR_HEIGHT: f64 = 100.0;
    rectangle(
        BLACK_COLOR,
        [
            SELECTOR_HORIZONTAL_POSITION,
            SELECTOR_VERTICAL_POSITION,
            WINDOW_WIDTH,
            SELECTOR_HEIGHT,
        ],
        *transform,
        window,
    );
}
