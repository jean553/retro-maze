extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
};

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

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |_, window, _| {

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
            }
        );
    }
}
