extern crate piston_window;

use piston_window::*;
use std::time::{SystemTime, Duration};
use rand::{Rng};

fn main() {
    let dim:usize = 24;
    let pdim: f64 = 16.0;

    let ssize: f64 = dim as f64 * pdim;

    // Spawn West
    let mut direction = 3;
    // NESW -> 0..4 
    // X = [0, 1, 0, -1]
    // Y = [-1, 0, 1, 0]

    // 9999 - Apple
    // 1 - Head (Snake)
    let mut state = vec![vec![0; dim]; dim];

    // Snake in center
    // Snake [x, y, tail n]
    let mut snake = [dim/2-1, dim/2, 2, direction.clone()];
    state[snake[0]][snake[1]] = 1; 
    state[snake[0]+1][snake[1]] = 2; 

    let mut window: PistonWindow = WindowSettings::new("Snake", [ssize, ssize]).exit_on_esc(true).build().unwrap();

    let mut delta_t: SystemTime = SystemTime::now();
    let frame_t = Duration::new(0,200000000);

    let mut game_state: usize = 1;

    state = apple(state, dim);

    while let Some(event) = window.next() {
        
        if let Some(button) = event.press_args() {
            if  Button::Keyboard(Key::Up) == button && snake[3] != 2{
                direction = 0
            } else if  Button::Keyboard(Key::Right) == button && snake[3] != 3{
                direction = 1
            }  else if  Button::Keyboard(Key::Down) == button && snake[3] != 0{
                direction = 2
            }  else if  Button::Keyboard(Key::Left) == button && snake[3] != 1{
                direction = 3
            } 
        }
        // Logic Tick
        if delta_t.elapsed().unwrap() > frame_t{
            delta_t = SystemTime::now();
            if game_state != 2{
            
                if (snake[0] == 0 && direction == 3) || (snake[0] == dim && direction == 1) || (snake[1] == dim && direction == 2)|| (snake[1] == 0 && direction == 0) {
                    game_state = 2;
                }else{
                    snake[3] = direction.clone();

                    snake[0] = snake[0] + ( [1, 2, 1, 0][direction] ) - 1 ;
                    snake[1] = snake[1] + ( [0, 1, 2, 1][direction] ) - 1 ;
    
                    if state[snake[0]][snake[1]] != 0{
                        if state[snake[0]][snake[1]] == 9999{
                            snake[2] += 1;
                            state = apple(state, dim);
                        }else{
                            game_state = 2;
                        }
                        
                    }
    
                    if game_state == 1{
                        for x in 0..dim{
                            for y in 0..dim{
                                if state[x][y] != 0{
                                    if state[x][y] == snake[2] as i32{
                                        state[x][y] = 0;
                                    }else if state[x][y] != 9999{
                                        state[x][y] += 1;
                                    }
                                }
                            }
                        }
                        
                        state[snake[0]][snake[1]] = 1;
                    }
                }

                
            }
        }

        // Render
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            // Bg
            rectangle([[0.0, 0.4, 0.0, 1.0],[0.0, 0.4, 0.0, 1.0],[0.9, 0.0, 0.0, 1.0]][game_state],
                [0.0, 0.0, ssize, ssize],
                context.transform,
                graphics);
            
            // Render
            for x in 0..dim{
                for y in 0..dim{
                    if state[x][y] == 9999{
                        rectangle([1.0, 0.0, 0.0, 1.0],
                            [pdim * x as f64, pdim * y as f64, pdim, pdim],
                            context.transform,
                            graphics);
                    }else if state[x][y] == 1 {
                        rectangle([1.0, 1.0, 0.0, 1.0],
                            [pdim * x as f64, pdim * y as f64, pdim, pdim],
                            context.transform,
                            graphics);
                    }else if state[x][y] != 0 {
                        rectangle([0.5, 0.7, 0.0, 1.0],
                            [pdim * x as f64, pdim * y as f64, pdim, pdim],
                            context.transform,
                            graphics);
                    }
                }
            }
        

        });
    }          

    

}

fn apple(mut state:std::vec::Vec<std::vec::Vec<i32>>, dim:usize) -> std::vec::Vec<std::vec::Vec<i32>>{
    let mut rng = rand::thread_rng();
    
    let x:usize = rng.gen_range(0..dim);
    let y:usize = rng.gen_range(0..dim);

    if state[x][y] != 0{
        return apple(state, dim);
    }else{
        state[x][y] = 9999;
    }
    return state;
}