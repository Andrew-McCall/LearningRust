use piston_window::*;
use std::time::{SystemTime, Duration};

fn main() {
    let dim:usize = 16;
    let pdim: f64 = 16.0;

    let ssize: f64 = dim as f64 * pdim;

    // Spawn West
    let mut direction = 3;
    // NESW -> 0..4 

    let colours = [[0.0, 0.0, 0.0, 1.0],[1.0, 1.0, 0.0, 1.0],[0.5, 0.7, 0.0, 1.0],[1.0, 0.0, 0.0, 1.0]];
    // 1 - Head (Snake)
    // 2 - Body
    // 3 - Apple
    let mut state = vec![vec![0; dim]; dim];

    // Snake in center
    state[dim/2-1][dim/2] = 1; 
    state[dim/2][dim/2] = 2; 

    let mut window: PistonWindow = WindowSettings::new("Snake", [ssize, ssize]).exit_on_esc(true).build().unwrap();

    let mut deltaT: SystemTime = SystemTime::now();
    let mut frameT = Duration::new(0,500000000);

    let mut x = 1;

    while let Some(event) = window.next() {
        if deltaT.elapsed().unwrap() > frameT{
            deltaT = SystemTime::now();
            state[x][0] = 1;
            x+=1;
        }
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            // Bg
            rectangle([0.0, 0.4, 0.0, 1.0],
                [0.0, 0.0, ssize, ssize],
                context.transform,
                graphics);
            
            // Render
            for x in 0..dim{
                for y in 0..dim{
                    if state[x][y] != 0{
                        rectangle(colours[state[x][y]],
                            [pdim * x as f64, pdim * y as f64, pdim, pdim],
                            context.transform,
                            graphics);
                    }
                }
            }
        

        });
    }          


}
