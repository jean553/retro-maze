extern crate piston_window;

mod gui;

use std::time;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Texture,
    G2dTexture,
    TextureSettings,
    Transformed,
    Flip,
    Button,
    Key,
    PressEvent,
    Glyphs,
    clear,
    image,
};

use gui::{
    display_selector,
    display_selectable_tile,
};

/// Refactored code to load a texture from a given image file name. Looks for files into the images resources folder.
///
/// # Args:
///
/// `window` - the window where the textures will be displayed
/// `image` - the file of the image to load
fn load_texture_from_file(
    window: &mut PistonWindow,
    file_name: &str,
) -> G2dTexture {

    const IMAGES_FOLDER: &str = "res/images/";
    let file_path = format!(
        "{}/{}",
        IMAGES_FOLDER,
        file_name,
    );

    Texture::from_path(
        &mut window.create_texture_context(),
        file_path,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap()
}

fn main() {

    const WINDOW_WIDTH: f64 = 800.0;
    const WINDOW_HEIGHT: f64 = 600.0;

    let mut window: PistonWindow = WindowSettings::new(
        "City Builder",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ],
    )
        .fullscreen(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let sun = load_texture_from_file(&mut window, "sun.png");
    let palm = load_texture_from_file(&mut window, "palm.png");

    let all_tiles = [
        load_texture_from_file(&mut window, "default.png"),
        load_texture_from_file(&mut window, "arrival_0.png"),
        load_texture_from_file(&mut window, "arrival_1.png"),
        load_texture_from_file(&mut window, "road.png"),
        load_texture_from_file(&mut window, "palm.png"),
        load_texture_from_file(&mut window, "departure_0.png"),
        load_texture_from_file(&mut window, "departure_1.png"),
        load_texture_from_file(&mut window, "departure_2.png"),
        load_texture_from_file(&mut window, "departure_3.png"),
        load_texture_from_file(&mut window, "departure_4.png"),
        load_texture_from_file(&mut window, "selected_ground.png"),
    ];

    const SELECTOR_DIGITS_FONT_FILE_PATH: &str = "res/fonts/fast_money.ttf";
    let mut selector_digits_font = Glyphs::new(
        SELECTOR_DIGITS_FONT_FILE_PATH,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();

    const TILE_HORIZONTAL_OFFSET: f64 = -75.0;
    const TILE_VERTICAL_OFFSET: f64 = -25.0;

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    const TILES_AMOUNT: usize = 330;
    const DEFAULT_SPRITE_INDEX: usize = 0;
    let mut tiles: [usize; TILES_AMOUNT] = [
        DEFAULT_SPRITE_INDEX;
        TILES_AMOUNT
    ];

    const ARRIVAL_TILE_INDEX: usize = 5;
    const FIRST_ARRIVAL_SPRITE_INDEX: usize = 1;
    const SECOND_ARRIVAL_SPRITE_INDEX: usize = 2;
    tiles[ARRIVAL_TILE_INDEX] = FIRST_ARRIVAL_SPRITE_INDEX;

    const DEPARTURE_TILE_INDEX: usize = 38;
    const FIRST_DEPARTURE_SPRITE_INDEX: usize = 5;
    const LAST_DEPARTURE_SPRITE_INDEX: usize = 9;
    tiles[DEPARTURE_TILE_INDEX] = FIRST_DEPARTURE_SPRITE_INDEX;

    const FIRST_SELECTED_SPRITE_INDEX: usize = 0;
    const SECOND_SELECTED_SPRITE_INDEX: usize = 10;

    let mut event_previous_time = time::Instant::now();
    let mut animations_previous_time = time::Instant::now();

    const DEFAULT_SELECTED_TILE_INDEX: usize = 27;
    let mut selected_tile_index: usize = DEFAULT_SELECTED_TILE_INDEX;

    while let Some(event) = window.next() {

        const ANIMATION_INTERVAL: u128 = 100;
        if time::Instant::now().duration_since(animations_previous_time).as_millis() >
            ANIMATION_INTERVAL {

            if tiles[ARRIVAL_TILE_INDEX] == FIRST_ARRIVAL_SPRITE_INDEX {
                tiles[ARRIVAL_TILE_INDEX] = SECOND_ARRIVAL_SPRITE_INDEX;
            } else {
                tiles[ARRIVAL_TILE_INDEX] = FIRST_ARRIVAL_SPRITE_INDEX;
            }

            tiles[DEPARTURE_TILE_INDEX] = if tiles[DEPARTURE_TILE_INDEX] == LAST_DEPARTURE_SPRITE_INDEX {
                FIRST_DEPARTURE_SPRITE_INDEX
            } else {
                tiles[DEPARTURE_TILE_INDEX] + 1
            };

            tiles[selected_tile_index] = if tiles[selected_tile_index] == SECOND_SELECTED_SPRITE_INDEX {
                FIRST_SELECTED_SPRITE_INDEX
            } else {
                SECOND_SELECTED_SPRITE_INDEX
            };

            animations_previous_time = time::Instant::now();
        }

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 10.0;
        const CAMERA_MOVEMENT_INTERVAL: u128 = 25;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Return)) = pressed_key {

            /* TODO: simply "visually" adds one road for now,
               but it should also update the selected tile
               in order to ensure future iterations */
            tiles[selected_tile_index] = 3;
            selected_tile_index -= 11;
        }

        window.draw_2d(
            &event,
            |context, window, device| {

                const BACKGROUND_COLOR: [f32; 4] = [0.06, 0.0, 0.1, 0.0];
                clear(BACKGROUND_COLOR, window);

                const SUN_RELATIVE_HORIZONTAL_POSITION: f64 = -730.0;
                const SUN_RELATIVE_VERTICAL_POSITION: f64 = -260.0;

                let sun_horizontal_position = SUN_RELATIVE_HORIZONTAL_POSITION +
                    origin_horizontal_position;
                let sun_vertical_position = SUN_RELATIVE_VERTICAL_POSITION +
                    origin_vertical_position;

                const SUN_WIDTH: f64 = 1000.0;
                const SUN_HEIGHT: f64 = 500.0;

                if sun_horizontal_position > -SUN_WIDTH &&
                    sun_horizontal_position < WINDOW_WIDTH &&
                    sun_vertical_position > -SUN_HEIGHT &&
                    sun_vertical_position < WINDOW_HEIGHT {

                    image(
                        &sun,
                        context.transform.trans(
                            SUN_RELATIVE_HORIZONTAL_POSITION +
                            origin_horizontal_position,
                            SUN_RELATIVE_VERTICAL_POSITION +
                            origin_vertical_position,
                        ),
                        window,
                    );
                }

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

                    if horizontal_position < -TILE_WIDTH ||
                        horizontal_position > WINDOW_WIDTH {
                        continue;
                    }

                    let vertical_position = origin_vertical_position +
                        vertical_offset +
                        index as f64 * TILE_VERTICAL_DISTANCE * 2.0;

                    if vertical_position < -TILE_HEIGHT ||
                        vertical_position > WINDOW_HEIGHT {
                        continue;
                    }

                    image(
                        &palm,
                        context.transform.trans(
                            horizontal_position,
                            vertical_position,
                        ),
                        window,
                    );
                }

                let mut column: usize = 0;
                let mut line: usize = 0;

                for (index, tile) in tiles.iter().enumerate() {

                    const TILES_PER_LINE: usize = 11;

                    if index != 0 &&
                        index % TILES_PER_LINE == 0 {
                        column = 0;
                        line += 1;
                    }

                    let tile_horizontal_position = TILE_HORIZONTAL_OFFSET -
                        TILE_HORIZONTAL_DISTANCE * (column as f64) +
                        TILE_HORIZONTAL_DISTANCE * (line as f64) +
                        origin_horizontal_position;

                    if tile_horizontal_position < -TILE_WIDTH ||
                        tile_horizontal_position > WINDOW_WIDTH {
                        column += 1;
                        continue;
                    }

                    let tile_vertical_position = TILE_VERTICAL_OFFSET +
                        TILE_VERTICAL_DISTANCE * (column as f64) +
                        TILE_VERTICAL_DISTANCE * (line as f64) +
                        origin_vertical_position;

                    if tile_vertical_position < -TILE_HEIGHT ||
                        tile_vertical_position > WINDOW_HEIGHT {
                        column += 1;
                        continue;
                    }

                    image(
                        &all_tiles[*tile],
                        context.transform.trans(
                            tile_horizontal_position,
                            tile_vertical_position
                        ),
                        window,
                    );

                    column += 1;
                }

                display_selector(
                    window,
                    &context.transform,
                );

                const FIRST_SELECTABLE_TILE_AMOUNT: &str = "0";
                const FIRST_SELECTABLE_TILE_INDEX: usize = 3;
                const FIRST_SELECTABLE_TILE_HORIZONTAL_POSITION: f64 = -60.0;
                const FIRST_SELECTABLE_TILE_VERTICAL_POSITION: f64 = 490.0;
                display_selectable_tile(
                    window,
                    &context,
                    &mut selector_digits_font,
                    &all_tiles[FIRST_SELECTABLE_TILE_INDEX],
                    FIRST_SELECTABLE_TILE_AMOUNT,
                    FIRST_SELECTABLE_TILE_HORIZONTAL_POSITION,
                    FIRST_SELECTABLE_TILE_VERTICAL_POSITION,
                );

                const SECOND_SELECTABLE_TILE_AMOUNT: &str = "3";
                const SECOND_SELECTABLE_TILE_INDEX: usize = 3;
                const SECOND_SELECTABLE_TILE_HORIZONTAL_POSITION: f64 = 60.0;
                const SECOND_SELECTABLE_TILE_VERTICAL_POSITION: f64 = 490.0;
                display_selectable_tile(
                    window,
                    &context,
                    &mut selector_digits_font,
                    &all_tiles[SECOND_SELECTABLE_TILE_INDEX],
                    SECOND_SELECTABLE_TILE_AMOUNT,
                    SECOND_SELECTABLE_TILE_HORIZONTAL_POSITION,
                    SECOND_SELECTABLE_TILE_VERTICAL_POSITION,
                );

                selector_digits_font.factory
                    .encoder
                    .flush(device);
            }
        );
    }
}
