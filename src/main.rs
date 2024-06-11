extern crate piston_window;

use piston_window::*;

fn main() {
    use markov::Chain;
    let mut chain = Chain::of_order(4);

    // train the chain on some vectors
    chain.feed(vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1])
         .feed(vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]);


    // constants
    let tile_count = 25;
    let tile_sides = 4;

    // generate sequences
    let mut gen = chain.iter();
    let mut n = 0;
    while n < tile_count{
        
        let seq = unsafe{(gen.next()).unwrap_unchecked()};
        
        if seq.len() >= tile_sides {
            
            let mut s = 0;
            while s < tile_sides {
                print!("{}", seq[s]);
                s += 1;
            }
            println!("");
            n += 1;
        }
    }

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
