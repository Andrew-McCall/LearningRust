extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button, ButtonArgs, ButtonEvent, MouseButton, ButtonState, MouseCursorEvent};
use piston::window::WindowSettings;
use std::path::Path;

use graphics::*;
use graphics::rectangle::{square};
use graphics::{DrawState, Image};

use rand::{Rng};

// Could make rot part of pos
pub struct Arrow{
    texture: usize,
    speed: f64,
    position: [f64; 2],
    rotation: f64,
}

pub struct Decal{
    texture: usize,
    postition: [f64; 2],
    rotation: f64,
}

pub struct Enemy{
    texture: usize,
    speed: f64,
    position: [f64; 2],
    health: i32,
    immune: f64,
}

pub struct App {
    gl: GlGraphics,
    mouse: [f64; 2],
    enemies: Vec<Enemy>,
    arrows: Vec<Arrow>,
    difficulty: f64,
    last_spawn: f64,
    textures: [Texture; 4],
    images: [Image; 4],
}

impl App {
    pub fn new(opengl:glutin_window::OpenGL, difficulty: f64) -> Self {
        
        let textures:[Texture; 4] = [
            Texture::from_path(Path::new("./assets/test.png"), &TextureSettings::new()).expect("Could not loadw test."),
            Texture::from_path(Path::new("./assets/arrow.png"), &TextureSettings::new()).expect("Could not load arrow."),
            Texture::from_path(Path::new("./assets/zombie.png"), &TextureSettings::new()).expect("Could not load zombie."),
            Texture::from_path(Path::new("./assets/BloodSplat.png.png"), &TextureSettings::new()).expect("Could not load Blood."),
            ];

        let images:[Image; 4] = [
            Image::new().rect(square(0.0, 0.0, 200.0)),
            Image::new().rect(square(0.0, 0.0, 20.0)),
            Image::new().rect(square(0.0, 0.0, 50.0)),
            Image::new().rect(square(0.0, 0.0, 200.0))
        ];

        return App {
            gl:GlGraphics::new(opengl),
            mouse: [0.0, 0.0],
            enemies: Vec::new(),
            arrows: Vec::new(),
            difficulty: difficulty,
            last_spawn:0.0,
            images:images,
            textures:textures,
        };

    }

    fn render(&mut self, args: &RenderArgs) {

        let draw_state: DrawState = Default::default();


        self.gl.draw(args.viewport(), |context, gl| {
            clear([0.0,0.4,0.0,1.0], gl);

            let transform = context
                .transform
                .trans(480.0, 180.0)
                .rot_rad((480.0-self.mouse[0]).atan2(self.mouse[1]-180.0)) // Width - x | y - Height/2
                .trans(-100.0, -100.0);
            
            self.images[0].draw(&self.textures[0], &draw_state, transform, gl);

            for enemy in &self.enemies{
                let transform = context.transform.trans(enemy.position[0],enemy.position[1]).trans(-self.images[enemy.texture].rectangle.unwrap()[2]/2.0,-self.images[enemy.texture].rectangle.unwrap()[3]/2.);
                self.images[enemy.texture].draw(&self.textures[enemy.texture], &draw_state, transform, gl)
            }

            for arrow in &self.arrows{
                let transform = context.transform.trans(arrow.position[0],arrow.position[1]).rot_rad(arrow.rotation).trans(-self.images[arrow.texture].rectangle.unwrap()[2]/2.0,-self.images[arrow.texture].rectangle.unwrap()[3]/2.);
                self.images[arrow.texture].draw(&self.textures[arrow.texture], &draw_state, transform, gl)
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        
        for x in 0..self.enemies.len(){
            self.enemies[x].position[0] += self.enemies[x].speed * args.dt;
        }

        for x in 0..self.arrows.len(){
            self.arrows[x].position[0] -= self.arrows[x].speed * args.dt * self.arrows[x].rotation.sin(); 
            self.arrows[x].position[1] += self.arrows[x].speed * args.dt * self.arrows[x].rotation.cos(); 
        }

        let mut rng = rand::thread_rng();
        self.last_spawn += args.dt * rng.gen::<f64>() * self.difficulty;
        if  self.last_spawn > 100.0{
            self.last_spawn = 0.0;
            self.enemies.push(Enemy{
                health:10,
                position: [-50.0, rng.gen::<f64>()*300.0+30.0], 
                speed: rng.gen::<f64>()*6.0+12.0,
                texture: 2,
                immune:0.0,
            });
        }

    }

    fn input(&mut self, args: &ButtonArgs){
        if  Button::Mouse(MouseButton::Left) == args.button{
            if (args.state) == ButtonState::Press{
                println!("{:#?} at ({}, {})", args.state, self.mouse[0], self.mouse[1]);
                self.arrows.push(Arrow{
                    position: [480.0, 180.0], 
                    speed: 100.0,
                    texture: 1,
                    rotation:((480.0-self.mouse[0]).atan2(self.mouse[1]-180.0)), // Width - x | y - Height/2
                });
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

    let mut app = App::new(opengl, 50.0);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {

        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(args) = event.update_args() {
            app.update(&args);
        }

        if let Some(args) = event.button_args() {
            app.input(&args);
        }

        event.mouse_cursor(|pos| {
            app.mouse = pos;
            
        });

    }
}