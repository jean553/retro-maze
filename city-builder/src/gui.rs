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

/// Displays selectable tile into the selectable tiles bar.
///
/// # Args:
///
/// `window` - the window where the textures will be displayed
/// `context` - the window context to use the appropriated transformations
/// `font` - the font to use to display the counter
/// `tile` - the tile texture to display
/// `amount` - the amount to display next to the tile
/// `index` - the selectable tile index
pub fn display_selectable_tile(
    window: &mut G2d,
    context: &Context,
    font: &mut Glyphs,
    tile: &G2dTexture,
    amount: &str,
    index: u8,
) {

    const FIRST_HORIZONTAL_POSITION: f64 = -60.0;
    const HORIZONTAL_DISTANCE: f64 = 120.0;
    let horizontal_position =
        FIRST_HORIZONTAL_POSITION +
        index as f64 *
        HORIZONTAL_DISTANCE;

    const TILE_VERTICAL_POSITION: f64 = 490.0;

    image(
        tile,
        context.transform.trans(
            horizontal_position,
            TILE_VERTICAL_POSITION,
        ),
        window,
    );

    const GRAY_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const SELECTABLE_TILE_COUNTER_FONT_SIZE: u32 = 16;

    const COUNTER_HORIZONTAL_OFFSET: f64 = 160.0;
    const COUNTER_VERTICAL_OFFSET: f64 = 100.0;
    const COUNTER_VERTICAL_POSITION: f64 = TILE_VERTICAL_POSITION + COUNTER_VERTICAL_OFFSET;

    text::Text::new_color(
        GRAY_COLOR,
        SELECTABLE_TILE_COUNTER_FONT_SIZE,
    ).draw(
        amount,
        font,
        &context.draw_state,
        context.transform.trans(
            horizontal_position + COUNTER_HORIZONTAL_OFFSET,
            COUNTER_VERTICAL_POSITION,
        ),
        window,
    ).unwrap();
}

/// Refactored function to display the palms.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
/// `palm_sprite` - the already loaded sprite to use for the palms
/// `origin_horizontal_position` - the origin horizontal position
/// `origin_vertical_position` - the origin vertical position
pub fn display_sides_palms(
    window: &mut G2d,
    transform: &[[f64; 3]; 2],
    palm_sprite: &G2dTexture,
    origin_horizontal_position: f64,
    origin_vertical_position: f64,
) {
    const TILE_HORIZONTAL_DISTANCE: f64 = 47.5;
    const TILE_VERTICAL_DISTANCE: f64 = 34.0;

    const TOP_PALMS_HORIZONTAL_OFFSET: f64 = -30.0;
    const TOP_PALMS_VERTICAL_OFFSET: f64 = -50.0;

    const BOTTOM_PALMS_HORIZONTAL_OFFSET: f64 = -600.0;
    const BOTTOM_PALMS_VERTICAL_OFFSET: f64 = 370.0;

    const SIDES_PALMS_AMOUNT: usize = 28;
    const ONE_SIDE_PALMS_AMOUNT: usize = SIDES_PALMS_AMOUNT / 2;

    const TILE_WIDTH: f64 = 250.0;
    const TILE_HEIGHT: f64 = 150.0;

    for index in 0..SIDES_PALMS_AMOUNT {

        let (
            index,
            horizontal_offset,
            vertical_offset
        ) = if index >= ONE_SIDE_PALMS_AMOUNT {

            (
                index - ONE_SIDE_PALMS_AMOUNT,
                BOTTOM_PALMS_HORIZONTAL_OFFSET,
                BOTTOM_PALMS_VERTICAL_OFFSET,
            )
        } else {
            (
                index,
                TOP_PALMS_HORIZONTAL_OFFSET,
                TOP_PALMS_VERTICAL_OFFSET,
            )
        };

        let horizontal_position = origin_horizontal_position +
            horizontal_offset +
            index as f64 * TILE_HORIZONTAL_DISTANCE * 2.0;

        const WINDOW_WIDTH: f64 = 800.0;
        if horizontal_position < -TILE_WIDTH ||
            horizontal_position > WINDOW_WIDTH {
            continue;
        }

        let vertical_position = origin_vertical_position +
            vertical_offset +
            index as f64 * TILE_VERTICAL_DISTANCE * 2.0;

        const WINDOW_HEIGHT: f64 = 800.0;
        if vertical_position < -TILE_HEIGHT ||
            vertical_position > WINDOW_HEIGHT {
            continue;
        }

        image(
            palm_sprite,
            transform.trans(
                horizontal_position,
                vertical_position,
            ),
            window,
        );
    }
}

/// Displays the sun image. Only called once but refactored for readability.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
/// `sun_sprite` - the already loaded sprite to use for the sun
/// `origin_horizontal_position` - the origin horizontal position
/// `origin_vertical_position` - the origin vertical position
pub fn display_sun(
    window: &mut G2d,
    transform: &[[f64; 3]; 2],
    sun_sprite: &G2dTexture,
    sun_horizontal_position: f64,
    sun_vertical_position: f64,
) {

     image(
         sun_sprite,
         transform.trans(
             sun_horizontal_position,
             sun_vertical_position,
         ),
         window,
     );
}

/// Display the tiles. Called only once but refactored into a function for readability.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
/// `all_sprites` - the list of all availables sprites
/// `tiles` - the displayed tiles list
/// `origin_horizontal_position` - the origin horizontal position
/// `origin_vertical_position` - the origin vertical position
pub fn display_tiles(
    window: &mut G2d,
    transform: &[[f64; 3]; 2],
    all_sprites: &[G2dTexture; 11],
    tiles: &[usize; 330],
    origin_horizontal_position: f64,
    origin_vertical_position: f64,
) {

    let mut column: usize = 0;
    let mut line: usize = 0;

    for (index, tile) in tiles.iter().enumerate() {

        const TILES_PER_LINE: usize = 11;

        if index != 0 &&
            index % TILES_PER_LINE == 0 {
            column = 0;
            line += 1;
        }

        const WINDOW_WIDTH: f64 = 800.0;
        const WINDOW_HEIGHT: f64 = 600.0;

        const TILE_HORIZONTAL_OFFSET: f64 = -75.0;
        const TILE_VERTICAL_OFFSET: f64 = -25.0;

        const TILE_HORIZONTAL_DISTANCE: f64 = 47.5;
        let tile_horizontal_position = TILE_HORIZONTAL_OFFSET -
            TILE_HORIZONTAL_DISTANCE * (column as f64) +
            TILE_HORIZONTAL_DISTANCE * (line as f64) +
            origin_horizontal_position;

        const TILE_WIDTH: f64 = 250.0;
        if tile_horizontal_position < -TILE_WIDTH ||
            tile_horizontal_position > WINDOW_WIDTH {
            column += 1;
            continue;
        }

        const TILE_VERTICAL_DISTANCE: f64 = 34.0;
        let tile_vertical_position = TILE_VERTICAL_OFFSET +
            TILE_VERTICAL_DISTANCE * (column as f64) +
            TILE_VERTICAL_DISTANCE * (line as f64) +
            origin_vertical_position;

        const TILE_HEIGHT: f64 = 150.0;
        if tile_vertical_position < -TILE_HEIGHT ||
            tile_vertical_position > WINDOW_HEIGHT {
            column += 1;
            continue;
        }

        image(
            &all_sprites[*tile],
            transform.trans(
                tile_horizontal_position,
                tile_vertical_position
            ),
            window,
        );

        column += 1;
    }
}
