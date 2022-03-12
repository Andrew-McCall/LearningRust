extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Texture, GlyphCache};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button, ButtonArgs, ButtonEvent, MouseButton, ButtonState, MouseCursorEvent};
use piston::window::WindowSettings;
use std::path::Path;

use graphics::*;
use graphics::rectangle::{square};
use graphics::{DrawState, Image};

use rand::{Rng};

struct HudButton{
    texture: Texture,
    image: Image,
    position: [f64; 2],
    id: i8,
}

struct Arrow{
    texture: usize,
    speed: f64,
    position: [f64; 2],
    rotation: f64,
}

struct Enemy{
    texture: usize,
    speed: f64,
    position: [f64; 2],
    health: i8,
}

struct Decal{
    texture: usize,
    position: [f64; 2],
    rotation: f64,
}

struct App {
    gl: GlGraphics,
    mouse_pos: [f64; 2],
    mouse_down: bool,
    
    enemies: Vec<Enemy>,
    arrows: Vec<Arrow>,
    decals: Vec<Decal>,
    difficulty: f64,
    last_spawn: f64,
    textures: [Texture; 4],
    images: [Image; 4],
    huds: [HudButton; 2],
    text: String,//Option<text::Text>,

    gamestate: i8,
    health: f64,

    rotation: f64,
    cooldown: f64,
    gold: i32,
    score: i32,
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

        let hud:[HudButton; 2] = [
            HudButton{
                image: Image::new().rect(square(0.0, 0.0, 40.0)),
                texture:Texture::from_path(Path::new("./assets/Shop.png"), &TextureSettings::new()).expect("Could not load Shop."),
                position:[395.0, 5.0],
                id: 0,
            },
            HudButton{
                image: Image::new().rect(square(0.0, 0.0, 50.0)),
                texture:Texture::from_path(Path::new("./assets/Pause.png"), &TextureSettings::new()).expect("Could not load Shop."),
                position:[430.0, 2.0],
                id: 1,
            }
        ];

        return App {
            gl:GlGraphics::new(opengl),
            mouse_pos: [0.0, 0.0],
            mouse_down: false,
            rotation:1.5708,
            enemies: Vec::new(),
            arrows: Vec::new(),
            decals: Vec::new(),
            huds: hud, 
            text: "Click To Start".to_string(),
            difficulty: difficulty,
            last_spawn:0.0,
            images:images,
            textures:textures,
            cooldown:0.0,
            gamestate: 0,
            health: 100.0,
            gold:0,
            score:0,
        };

    }

    fn render(&mut self, args: &RenderArgs) {

        let draw_state: DrawState = Default::default();
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();
        let text_gold = text::Text::new_color([1.0, 1.0, 0.4, 1.0], 18);
        let text_score = text::Text::new_color([1.0, 1.0, 0.4, 1.0], 16);

        let colo_r;
        if self.gamestate == -1 {
            colo_r = [0.6, 0.0, 0.0, 1.0];
        }else{
            colo_r = [1.0, 1.0, 1.0, 1.0];
        }
        
        let text_announce = text::Text::new_color(colo_r, 48);

        self.gl.draw(args.viewport(), |context, gl| {
            clear([0.0,0.4,0.0,1.0], gl);
            
            // Entities //
            for decal in &self.decals{
                let transform = context.transform.trans(decal.position[0],decal.position[1]).rot_rad(decal.rotation).trans(-self.images[decal.texture].rectangle.unwrap()[2]/2.0,-self.images[decal.texture].rectangle.unwrap()[3]/2.);
                self.images[decal.texture].draw(&self.textures[decal.texture], &draw_state, transform, gl)
            }
            
            // (Crossbow)
            self.images[0].draw(&self.textures[0], &draw_state, context.transform.trans(480.0, 205.0).rot_rad(self.rotation).trans(-50.0, -50.0), gl);
            
            for arrow in &self.arrows{
                let transform = context.transform.trans(arrow.position[0],arrow.position[1]).rot_rad(arrow.rotation).trans(-self.images[arrow.texture].rectangle.unwrap()[2]/2.0,-self.images[arrow.texture].rectangle.unwrap()[3]/2.);
                self.images[arrow.texture].draw(&self.textures[arrow.texture], &draw_state, transform, gl)
            }

            for enemy in &self.enemies{
                let transform = context.transform.trans(enemy.position[0],enemy.position[1]).trans(-self.images[enemy.texture].rectangle.unwrap()[2]/2.0,-self.images[enemy.texture].rectangle.unwrap()[3]/2.);
                self.images[enemy.texture].draw(&self.textures[enemy.texture], &draw_state, transform, gl)
            }

            // GUI //
            let hud = rectangle::rectangle_by_corners(0.0, 0.0, 480.0, 50.0);
            rectangle([0.25, 0.27, 0.25, 1.0], hud, context.transform, gl);

            let health_back = rectangle::rectangle_by_corners(0.0, 0.0, 300.0, 30.0);
            rectangle([0.6, 0.0, 0.0, 1.0], health_back, context.transform.trans(7.50, 10.0), gl);
            let health = rectangle::rectangle_by_corners(0.0, 0.0, 3.0*self.health, 30.0);
            rectangle([0.9, 0.0, 0.0, 1.0], health, context.transform.trans(7.50, 10.0), gl);

            for hbutton in &self.huds{
                hbutton.image.draw(&hbutton.texture, &draw_state, context.transform.trans(hbutton.position[0], hbutton.position[1]), gl)
            }
            
            text_score.draw(&("Score: ".to_string()+&self.score.to_string()),
            &mut glyph_cache,
            &Default::default(),
            context.transform.trans(317.5,20.0),
            gl).unwrap();

            text_gold.draw(&("Gold: ".to_string()+&self.gold.to_string()),
            &mut glyph_cache,
            &Default::default(),
            context.transform.trans(317.5,40.0),
            gl).unwrap();

            

            text_announce.draw(&self.text,
            &mut glyph_cache,
            &Default::default(),
            context.transform.trans(240.0-(self.text.len()*10) as f64,125.0),
            gl).unwrap();



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
                        
                        self.gold += (rng.gen::<f32>()*5.0).trunc() as i32;
                        self.score += 10;

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
            }
        }

        if self.health <= 0.0{
            self.gamestate = -1;
            self.health = 0.0;
            self.text = "Game Over".to_string();
        };

        // Spawner
        self.last_spawn += args.dt * rng.gen::<f64>() * self.difficulty;
        if  self.last_spawn > 100.0{

            while self.last_spawn > 100.0{
                self.enemies.push(Enemy{
                    health:10,
                    position: [-50.0, rng.gen::<f64>()*270.0+60.0], 
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
        if  self.mouse_down{
            if self.mouse_pos[1]>50.0{
                if self.cooldown == 0.0 && self.gamestate==1{
                    self.arrows.push(Arrow{
                        position: [480.0, 205.0], 
                        speed: 150.0,
                        texture: 1,
                        rotation:(self.rotation), // Width - x | y - Height/2
                    });
                    self.cooldown = 1.0;
                }  
            }else{
            }
        }
        
        // Crossbow Rotation
        if self.mouse_pos[0] > 480.0{
            self.mouse_pos[0] = 480.0;
        }

        let real_rotation = (480.0-self.mouse_pos[0]).atan2(self.mouse_pos[1]-205.0);
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
            if !self.mouse_down && self.gamestate != -1{
                for hbutton in &self.huds{
                    if self.mouse_pos[0]>hbutton.position[0] && self.mouse_pos[1]>hbutton.position[1] && self.mouse_pos[0]<hbutton.position[0]+hbutton.image.rectangle.unwrap()[2] && self.mouse_pos[1]<hbutton.position[1]+hbutton.image.rectangle.unwrap()[3]{
                        
                        // Pause (Shop = 0)
                        if hbutton.id == 1{
                            
                            if self.gamestate == 0{
                                self.text = "".to_string();
                                self.gamestate = 1;
                            }else{
                                self.text = "Pause!".to_string();
                                self.gamestate = 0;
                            }
                            
                        }
                    }
                }
            }else if self.gamestate == 2{
                self.gamestate = 1;
                self.text = "".to_string();
            }
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

    app.gamestate = 2;

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