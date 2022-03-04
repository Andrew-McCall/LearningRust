extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button, ButtonArgs, ButtonEvent, MouseButton, ButtonState};
use piston::window::WindowSettings;
use graphics::{DrawState, Image};
use graphics::*;

use std::path::Path;
use graphics::rectangle::square;


pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let square:types::Rectangle = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let image   = Image::new().rect(rectangle::square(0.0, 0.0, 200.0));

        let texture = Texture::from_path(Path::new("./assets/test.png"), &TextureSettings::new())
        .expect("Could not loadw test.");

        let draw_state: DrawState = Default::default();


        self.gl.draw(args.viewport(), |context, gl| {
            clear([0.0,0.4,0.0,1.0], gl);

            let transform = context
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-100.0, -100.0);

            rectangle([0.0,1.0,0.0,1.0], square, transform, gl);
            
            image.draw(&texture, &draw_state, transform, gl);


        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 10.0 * args.dt;
    }

    fn button(&mut self, args: &ButtonArgs){
        if  Button::Mouse(MouseButton::Left) == args.button{
            if (args.state) == ButtonState::Press{
                println!("{:#?}", args.state);
            }
        } 
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Fortress", [480, 360])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 5.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {

        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(args) = event.update_args() {
            app.update(&args);
        }

        if let Some(args) = event.button_args() {
            app.button(&args);
        }

    }
}