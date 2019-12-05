//! Handles GUI actions.

use piston_window::{
    G2d,
    G2dTexture,
    Transformed,
    Context,
    Glyphs,
    rectangle,
    image,
    text,
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

/// Refactored code to displays selectable tile into the selectable tiles bar.
///
/// # Args:
///
/// `window` - the window where the textures will be displayed
/// `context` - the window context to use the appropriated transformations
/// `font` - the font to use to display the counter
/// `tile` - the tile texture to display
/// `amount` - the amount to display next to the tile
pub fn display_selectable_tile(
    window: &mut G2d,
    context: &Context,
    font: &mut Glyphs,
    tile: &G2dTexture,
    amount: &str,
    horizontal_position: f64,
    vertical_position: f64,
) {

    image(
        tile,
        context.transform.trans(
            horizontal_position,
            vertical_position,
        ),
        window,
    );

    const GRAY_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const SELECTABLE_TILE_COUNTER_FONT_SIZE: u32 = 16;

    const COUNTER_HORIZONTAL_OFFSET: f64 = 160.0;
    const COUNTER_VERTICAL_OFFSET: f64 = 100.0;
    text::Text::new_color(
        GRAY_COLOR,
        SELECTABLE_TILE_COUNTER_FONT_SIZE,
    ).draw(
        amount,
        font,
        &context.draw_state,
        context.transform.trans(
            horizontal_position + COUNTER_HORIZONTAL_OFFSET,
            vertical_position + COUNTER_VERTICAL_OFFSET,
        ),
        window,
    ).unwrap();
}
