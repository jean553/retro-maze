extern crate piston_window;

mod gui;

use std::time;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Texture,
    G2dTexture,
    TextureSettings,
    Flip,
    Button,
    Key,
    PressEvent,
    Glyphs,
    clear,
    image,
    Transformed,
};

use gui::{
    display_selector,
    display_sides_palms,
    display_sun,
    display_tiles,
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

    let car_forward_texture = load_texture_from_file(&mut window, "delorean_0.png");
    let car_left_texture = load_texture_from_file(&mut window, "delorean_1.png");
    let car_backward_texture = load_texture_from_file(&mut window, "delorean_2.png");
    let car_right_texture = load_texture_from_file(&mut window, "delorean_3.png");

    let all_tiles = [
        load_texture_from_file(&mut window, "palm.png"),
        load_texture_from_file(&mut window, "default.png"),
        load_texture_from_file(&mut window, "arrival_0.png"),
        load_texture_from_file(&mut window, "arrival_1.png"),
        load_texture_from_file(&mut window, "road_0.png"),
        load_texture_from_file(&mut window, "road_1.png"),
        load_texture_from_file(&mut window, "departure_0.png"),
        load_texture_from_file(&mut window, "departure_1.png"),
        load_texture_from_file(&mut window, "departure_2.png"),
        load_texture_from_file(&mut window, "departure_3.png"),
        load_texture_from_file(&mut window, "departure_4.png"),
        load_texture_from_file(&mut window, "turn_0.png"),
        load_texture_from_file(&mut window, "turn_1.png"),
        load_texture_from_file(&mut window, "turn_2.png"),
        load_texture_from_file(&mut window, "turn_3.png"),
        load_texture_from_file(&mut window, "intersection_0.png"),
        load_texture_from_file(&mut window, "intersection_1.png"),
        load_texture_from_file(&mut window, "intersection_2.png"),
        load_texture_from_file(&mut window, "intersection_3.png"),
    ];

    const SELECTOR_DIGITS_FONT_FILE_PATH: &str = "res/fonts/fast_money.ttf";
    let mut selector_digits_font = Glyphs::new(
        SELECTOR_DIGITS_FONT_FILE_PATH,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    const TILES_AMOUNT: usize = 330;
    const DEFAULT_SPRITE_INDEX: usize = 1;
    let mut tiles: [usize; TILES_AMOUNT] = [
        DEFAULT_SPRITE_INDEX;
        TILES_AMOUNT
    ];

    /* TODO: set some tiles for now, should be set per level */

    const ARRIVAL_TILE_INDEX: usize = 5;
    const FIRST_ARRIVAL_SPRITE_INDEX: usize = 2;
    const SECOND_ARRIVAL_SPRITE_INDEX: usize = 3;
    tiles[ARRIVAL_TILE_INDEX] = FIRST_ARRIVAL_SPRITE_INDEX;

    const DEPARTURE_TILE_INDEX: usize = 82;
    const FIRST_DEPARTURE_SPRITE_INDEX: usize = 6;
    const LAST_DEPARTURE_SPRITE_INDEX: usize = 10;
    tiles[DEPARTURE_TILE_INDEX] = FIRST_DEPARTURE_SPRITE_INDEX;

    tiles[71] = 5;
    tiles[60] = 5;
    tiles[49] = 14;
    tiles[50] = 4;
    tiles[51] = 16;

    tiles[62] = 13;
    tiles[63] = 4;
    tiles[64] = 12;
    tiles[53] = 5;
    tiles[42] = 5;
    tiles[31] = 5;
    tiles[20] = 11;
    tiles[19] = 4;
    tiles[18] = 18;
    tiles[17] = 4;

    tiles[40] = 5;
    tiles[29] = 5;

    tiles[16] = 17;
    tiles[27] = 12;
    tiles[26] = 4;
    tiles[25] = 17;

    tiles[14] = 11;
    tiles[13] = 4;
    tiles[12] = 14;
    tiles[23] = 5;
    tiles[34] = 5;
    tiles[45] = 5;
    tiles[56] = 5;
    tiles[67] = 13;
    tiles[68] = 4;
    tiles[69] = 12;
    tiles[58] = 5;
    tiles[47] = 5;
    tiles[36] = 5;

    tiles[61] = 0;
    tiles[57] = 0;
    tiles[15] = 0;
    tiles[70] = 0;
    tiles[28] = 0;
    tiles[39] = 0;
    tiles[52] = 0;

    let mut event_previous_time = time::Instant::now();
    let mut animations_previous_time = time::Instant::now();

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

                    display_sun(
                        window,
                        &context.transform,
                        &sun,
                        sun_horizontal_position,
                        sun_vertical_position,
                    );
                }

                const PALM_TILE_INDEX: usize = 0;
                display_sides_palms(
                    window,
                    &context.transform,
                    &all_tiles[PALM_TILE_INDEX],
                    origin_horizontal_position,
                    origin_vertical_position,
                );

                display_tiles(
                    window,
                    &context.transform,
                    &all_tiles,
                    &tiles,
                    origin_horizontal_position,
                    origin_vertical_position,
                );

                /* TODO: display a fixed position car for now,
                   the car should be able to move on the map */

                const CAR_HORIZONTAL_POSITION: f64 = 20.0;
                const CAR_VERTICAL_POSITION: f64 = 380.0;
                image(
                    &car_forward_texture,
                    context.transform.trans(
                        CAR_HORIZONTAL_POSITION + origin_horizontal_position,
                        CAR_VERTICAL_POSITION + origin_vertical_position,
                    ),
                    window,
                );

                display_selector(
                    window,
                    &context.transform,
                );

                selector_digits_font.factory
                    .encoder
                    .flush(device);
            }
        );
    }
}
