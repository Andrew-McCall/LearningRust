use piston_window::*;
use std::time::{SystemTime, Duration};
use rand::{Rng};

fn main() {


    let mut window: PistonWindow = WindowSettings::new("Fortress", [480, 360]).exit_on_esc(true).build().unwrap();

    let mut cursor = [0.0, 0.0];

    while let Some(event) = window.next() {

        event.mouse_cursor(|pos| {
            cursor = pos;
            println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });
        
        if let Some(cursor) = event.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };

        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);

                rectangle([0.0,0.4,0.0,1.0],
                    [0.0, 0.0, 480.0, 360.0],
                    context.transform,
                    graphics);

            });
        };

        
        // if let Some(_args) = event.idle_args() {
        // }

        // if let Some(_args) = event.update_args() {
        // }

    }
}
