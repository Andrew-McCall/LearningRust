extern crate piston_window;

use piston_window::*;
use rand::{Rng};

fn main() {
    
    let dim: usize = 30;
    let pdim: f64 = 8.0;
    
    let chance: f64 = 0.500; 


    let sdim: u32 = dim as u32 * pdim as u32;

    let mut state = vec![vec![false; dim]; dim];

    {
        let mut rng = rand::thread_rng();
        let mut z:f64;
    
        let mut x:usize = 0;
        while x < dim{
            let mut y:usize = 0;
            while y < dim {
                z = rng.gen();
                if z < chance{
                    state[x][y] = true;
                }
                y+=1;
            }
            x+=1;
        }
    }

    let mut window: PistonWindow = WindowSettings::new("Game Of Life", [sdim, sdim]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([0.0, 0.0, 0.0, 0.6],
                [0.0, 0.0, pdim*dim as f64, pdim*dim as f64],
                context.transform,
                graphics);
            let mut next = state.clone();

            for x in 0..dim{
                for y in 0..dim{
                    let z:i8 = count(state.clone(), x.clone(), y.clone(), dim.clone());

                    if state[x][y]{ 

                        if z < 2 || z > 3 {
                            next[x][y] = false;
                        } 

                        let color: f32 = 0.1 * z as f32;

                        rectangle([color, 0.0, color, 1.0],
                            [pdim*x as f64, pdim*y as f64, pdim, pdim],
                            context.transform,
                            graphics);

                    } else{

                        if z == 3 {
                            next[x][y] = true;
                        }

                    }

                }
            }

            state = next.clone();

        });
        
        

    }
}

fn count(state:std::vec::Vec<std::vec::Vec<bool>>, x:usize , y:usize, dim:usize) -> i8{

    let mut alive: i8 = 0;
    for xo in 0..3 {
        for yo in 0..3 {
            if  xo == 1 && yo == 1{
                
            }else{
                if state[((x + xo + (dim-1))%dim)][(y + yo + (dim-1))%dim]{
                    alive += 1;
                }
            }
        }
    }

    return alive;

}