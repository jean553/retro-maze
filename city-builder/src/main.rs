extern crate piston_window;

mod tile;

use std::time;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Texture,
    TextureSettings,
    Transformed,
    Flip,
    Button,
    Key,
    PressEvent,
    clear,
    image,
};

use tile::Tile;

fn main() {

    const WINDOW_WIDTH: f64 = 1600.0;
    const WINDOW_HEIGHT: f64 = 900.0;

    let mut window: PistonWindow = WindowSettings::new(
        "City Builder",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ],
    )
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let sun = Texture::from_path(
        &mut window.create_texture_context(),
        "res/images/sun.png",
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let all_tiles = [
        Texture::from_path(
            &mut window.create_texture_context(),
            "res/images/ground_default.png",
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
        Texture::from_path(
            &mut window.create_texture_context(),
            "res/images/ground_road_1.png",
            Flip::None,
            &TextureSettings::new(),
        ).unwrap()
    ];

    const TILE_HORIZONTAL_OFFSET: f64 = -75.0;
    const TILE_VERTICAL_OFFSET: f64 = -25.0;

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    const TILES_AMOUNT: usize = 625;
    let tiles: [Tile; TILES_AMOUNT] = [
        Tile::new();
        TILES_AMOUNT
    ];

    let mut previous_time = time::Instant::now();

    while let Some(event) = window.next() {

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 10.0;
        const CAMERA_MOVEMENT_INTERVAL: u128 = 50;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            if time::Instant::now().duration_since(previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
                previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            if time::Instant::now().duration_since(previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
                previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            if time::Instant::now().duration_since(previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
                previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            if time::Instant::now().duration_since(previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
                previous_time = time::Instant::now();
            }
        }

        window.draw_2d(
            &event,
            |context, window, _| {

                const BLACK_COLOR: [f32; 4] = [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ];

                clear(
                    BLACK_COLOR,
                    window,
                );

                const SUN_RELATIVE_HORIZONTAL_POSITION: f64 = 50.0;
                const SUN_RELATIVE_VERTICAL_POSITION: f64 = -800.0;

                let sun_horizontal_position = SUN_RELATIVE_HORIZONTAL_POSITION +
                    origin_horizontal_position;
                let sun_vertical_position = SUN_RELATIVE_VERTICAL_POSITION +
                    origin_vertical_position;

                if sun_horizontal_position > -1000.0 &&
                    sun_horizontal_position < WINDOW_WIDTH &&
                    sun_vertical_position > -500.0 &&
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

                let mut column: usize = 0;
                let mut line: usize = 0;

                for (index, tile) in tiles.iter().enumerate() {

                    const TILES_PER_LINE: usize = 25;

                    if index != 0 &&
                        index % TILES_PER_LINE == 0 {
                        column = 0;
                        line += 1;
                    }

                    const TILE_WIDTH: f64 = 250.0;
                    const TILE_HEIGHT: f64 = 150.0;
                    const TILE_HORIZONTAL_DISTANCE: f64 = 47.5;
                    const TILE_VERTICAL_DISTANCE: f64 = 34.0;

                    let tile_horizontal_position = TILE_HORIZONTAL_OFFSET +
                        TILE_HORIZONTAL_DISTANCE * (column as f64) +
                        TILE_HORIZONTAL_DISTANCE * (line as f64) +
                        origin_horizontal_position;

                    if tile_horizontal_position < -TILE_WIDTH ||
                        tile_horizontal_position > WINDOW_WIDTH {
                        column += 1;
                        continue;
                    }

                    let tile_vertical_position = TILE_VERTICAL_OFFSET -
                        TILE_VERTICAL_DISTANCE * (column as f64) +
                        TILE_VERTICAL_DISTANCE * (line as f64) +
                        origin_vertical_position;

                    if tile_vertical_position < -TILE_HEIGHT ||
                        tile_vertical_position > WINDOW_HEIGHT {
                        column += 1;
                        continue;
                    }

                    image(
                        &all_tiles[tile.get_sprite()],
                        context.transform.trans(
                            tile_horizontal_position,
                            tile_vertical_position
                        ),
                        window,
                    );

                    column += 1;
                }
            }
        );
    }
}
