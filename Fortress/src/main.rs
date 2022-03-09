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

pub struct Enemy{
    texture: usize,
    speed: f64,
    position: [f64; 2],
    health: i8,
}

pub struct Decal{
    texture: usize,
    position: [f64; 2],
    rotation: f64,
}

pub struct App {
    gl: GlGraphics,
    mouse_pos: [f64; 2],
    mouse_down: bool,
    rotation: f64,
    enemies: Vec<Enemy>,
    arrows: Vec<Arrow>,
    decals: Vec<Decal>,
    difficulty: f64,
    last_spawn: f64,
    textures: [Texture; 4],
    images: [Image; 4],
    cooldown: f64,
    gamestate: i8,
    health: f64,
}

impl App {
    pub fn new(opengl:glutin_window::OpenGL, difficulty: f64) -> Self {
        
        let textures:[Texture; 4] = [
            Texture::from_path(Path::new("./assets/Crossbow.png"), &TextureSettings::new()).expect("Could not load crossbow."),
            Texture::from_path(Path::new("./assets/arrow.png"), &TextureSettings::new()).expect("Could not load arrow."),
            Texture::from_path(Path::new("./assets/zombie.png"), &TextureSettings::new()).expect("Could not load zombie."),
            Texture::from_path(Path::new("./assets/BloodSplat.png"), &TextureSettings::new()).expect("Could not load Blood."),
            ];

        let images:[Image; 4] = [
            Image::new().rect(square(0.0, 0.0, 100.0)),
            Image::new().rect(square(0.0, 0.0, 20.0)),
            Image::new().rect(square(0.0, 0.0, 50.0)),
            Image::new().rect(square(0.0, 0.0, 40.0))
        ];

        return App {
            gl:GlGraphics::new(opengl),
            mouse_pos: [0.0, 0.0],
            mouse_down: false,
            rotation:1.0,
            enemies: Vec::new(),
            arrows: Vec::new(),
            decals: Vec::new(),
            difficulty: difficulty,
            last_spawn:0.0,
            images:images,
            textures:textures,
            cooldown:0.0,
            gamestate: 0,
            health: 100.0,
        };

    }

    fn render(&mut self, args: &RenderArgs) {

        let draw_state: DrawState = Default::default();

        self.gl.draw(args.viewport(), |context, gl| {
            clear([0.0,0.4,0.0,1.0], gl);
            
            for decal in &self.decals{
                let transform = context.transform.trans(decal.position[0],decal.position[1]).rot_rad(decal.rotation).trans(-self.images[decal.texture].rectangle.unwrap()[2]/2.0,-self.images[decal.texture].rectangle.unwrap()[3]/2.);
                self.images[decal.texture].draw(&self.textures[decal.texture], &draw_state, transform, gl)
            }
            
            // Crossbow
            self.images[0].draw(&self.textures[0], &draw_state, context.transform.trans(480.0, 180.0).rot_rad(self.rotation).trans(-50.0, -50.0), gl);
            
            for arrow in &self.arrows{
                let transform = context.transform.trans(arrow.position[0],arrow.position[1]).rot_rad(arrow.rotation).trans(-self.images[arrow.texture].rectangle.unwrap()[2]/2.0,-self.images[arrow.texture].rectangle.unwrap()[3]/2.);
                self.images[arrow.texture].draw(&self.textures[arrow.texture], &draw_state, transform, gl)
            }

            for enemy in &self.enemies{
                let transform = context.transform.trans(enemy.position[0],enemy.position[1]).trans(-self.images[enemy.texture].rectangle.unwrap()[2]/2.0,-self.images[enemy.texture].rectangle.unwrap()[3]/2.);
                self.images[enemy.texture].draw(&self.textures[enemy.texture], &draw_state, transform, gl)
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.gamestate != 1 {
            return;
        }

        let mut rng = rand::thread_rng();

        // Arrow Logic
        for x in 0..self.arrows.len(){
            self.arrows[x].position[0] -= self.arrows[x].speed * args.dt * self.arrows[x].rotation.sin(); 
            self.arrows[x].position[1] += self.arrows[x].speed * args.dt * self.arrows[x].rotation.cos(); 

            for y in 0..self.enemies.len(){
                if self.enemies[y].health!=0{
                    let dx = self.enemies[y].position[0] - self.arrows[x].position[0];
                    let dy = self.enemies[y].position[1] - self.arrows[x].position[1];

                    if dx*dx + dy*dy < 400.0{
                        self.decals.push(Decal{
                            texture: 3,
                            position: self.enemies[y].position,
                            rotation: rng.gen::<f64>()*6.4,
                        });
                        
                        self.enemies[y].health = 0;

                        self.difficulty += 10.0;

                        self.arrows[x].position[0] = -1000.0;
                    }
                } 
            }
        }

        // Dead Clean up
        self.arrows.retain(|x| &x.position[0] > &-5.0);
        self.enemies.retain(|x| &x.health > &0);

        // Enemy Logic
        for x in 0..self.enemies.len(){
            if self.enemies[x].position[0] < 480.0{
                self.enemies[x].position[0] += self.enemies[x].speed * args.dt;
            }else{
                self.health -= 1.0 * args.dt;
                println!("{}",self.health);
            }
        }

        if self.health <= 0.0{
            self.gamestate = -1;
        };
        // Spawner
        self.last_spawn += args.dt * rng.gen::<f64>() * self.difficulty;
        if  self.last_spawn > 100.0{

            while self.last_spawn > 100.0{
                self.enemies.push(Enemy{
                    health:10,
                    position: [-50.0, rng.gen::<f64>()*300.0+30.0], 
                    speed: rng.gen::<f64>()*25.0+25.0,
                    texture: 2,
                });
                self.last_spawn -= 150.0;
            }

            self.last_spawn = 0.0;
            
        }

        // Crossbow Cooldown
        self.cooldown -= args.dt*2.0;

        if self.cooldown < 0.0{
            self.cooldown = 0.0;
        }

        // Crossbow Firing
        if self.cooldown == 0.0 && self.gamestate==1 && self.mouse_down{
            self.arrows.push(Arrow{
                position: [480.0, 180.0], 
                speed: 150.0,
                texture: 1,
                rotation:(self.rotation), // Width - x | y - Height/2
            });
            self.cooldown = 1.0;
        }

        // Crossbow Rotation
        let real_rotation = (480.0-self.mouse_pos[0]).atan2(self.mouse_pos[1]-180.0);
        if (self.rotation - real_rotation).abs()> args.dt * 1.5{
            if real_rotation > self.rotation{
                self.rotation += args.dt * 1.5;
            }else{
                self.rotation -= args.dt * 1.5;
            }
        }
        
    }

    fn input(&mut self, args: &ButtonArgs){
        if  Button::Mouse(MouseButton::Left) == args.button{
            self.mouse_down = (args.state) == ButtonState::Press;
        } 
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Fortress", [480, 360])
        .graphics_api(opengl)
        .exit_on_esc(true)

        .fullscreen(false)

        .build()
        .unwrap();

    let mut app = App::new(opengl, 50.0);
    
    app.gamestate = 1;

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
            app.mouse_pos = pos;
        });

    }
}