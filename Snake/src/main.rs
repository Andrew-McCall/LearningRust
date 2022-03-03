use piston_window::*;

fn main() {
    let dim:usize = 16;
    let pdim: f64 = 16.0;


    let ssize: f64 = dim as f64 * pdim;

    let colours = [[0.0, 0.0, 0.0, 1.0],[1.0, 1.0, 0.0, 1.0],[0.5, 0.7, 0.0, 1.0],[1.0, 0.0, 0.0, 1.0]];
    // 1 - Head (Snake)
    // 2 - Body
    // 3 - Apple
    let mut state = vec![vec![0; dim]; dim];

    state[dim/2-1][dim/2] = 1; // Snake in center
    state[dim/2][dim/2] = 2; // Snake in center

    let mut window: PistonWindow = WindowSettings::new("Snake", [ssize, ssize]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
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
