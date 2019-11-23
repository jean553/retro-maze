extern crate piston_window;

mod tile;

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

    let default_ground = Texture::from_path(
        &mut window.create_texture_context(),
        "res/images/ground_default.png",
        Flip::None,
        &TextureSettings::new(),
    )
        .unwrap();

    const TILE_HORIZONTAL_OFFSET: f64 = -75.0;
    const TILE_VERTICAL_OFFSET: f64 = -25.0;

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    const TILES_AMOUNT: usize = 625;
    let tiles: [Tile; TILES_AMOUNT] = [
        Tile::new();
        TILES_AMOUNT
    ];

    while let Some(event) = window.next() {

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 5.0;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
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

                let mut column: usize = 0;
                let mut line: usize = 0;

                for (index, _) in tiles.iter().enumerate() {

                    const TILES_PER_LINE: usize = 25;

                    if index != 0 &&
                        index % TILES_PER_LINE == 0 {
                        column = 0;
                        line += 1;
                    }

                    const TILE_HORIZONTAL_DISTANCE: f64 = 47.5;
                    const TILE_VERTICAL_DISTANCE: f64 = 34.0;

                    image(
                        &default_ground,
                        context.transform.trans(
                            TILE_HORIZONTAL_OFFSET +
                            TILE_HORIZONTAL_DISTANCE * (column as f64) +
                            TILE_HORIZONTAL_DISTANCE * (line as f64) +
                            origin_horizontal_position,
                            TILE_VERTICAL_OFFSET -
                            TILE_VERTICAL_DISTANCE * (column as f64) +
                            TILE_VERTICAL_DISTANCE * (line as f64) +
                            origin_vertical_position,
                        ),
                        window,
                    );

                    column += 1;
                }
            }
        );
    }
}
