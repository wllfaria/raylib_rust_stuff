use rand::Rng;
use raylib::{ffi::Vector2, prelude::*};

pub fn draw() {
    let mut rng = rand::thread_rng();
    let size: (f32, f32) = (480.0, 270.0);
    let (mut rl, thread) = raylib::init()
        .size(size.0 as i32, size.1 as i32)
        .title("Sierpinski Triangle")
        .build();

    let triangle = vec![
        (size.0 / 2.0, 45.0),
        (90.0, size.1 - 45.0),
        (size.0 - 90.0, size.1 - 45.0),
    ];

    let mut point = Vector2 {
        x: size.0 / 2.0,
        y: size.1 / 2.0,
    };

    let mut cleared = false;

    while !rl.window_should_close() {
        let rnd = rng.gen_range(0..3);
        let chosen = triangle[rnd];

        point.x = (point.x + chosen.0) / 2f32;
        point.y = (point.y + chosen.1) / 2f32;

        let mut d = rl.begin_drawing(&thread);
        if !cleared {
            d.clear_background(Color::BLACK);
            cleared = true;
        }
        d.draw_pixel_v(point, Color::WHITE);
    }
}
