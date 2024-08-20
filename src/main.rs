mod miniquad;

pub use crate::miniquad::draw;

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
    let gen = chain.iter_for(tile_count);
    let mut n = 1;
    for tile in gen {
        print!("Tile {n}: ");
        print!("{:?}", &tile[0..tile_sides]);
        println!("\n");
        n += 1;
    }

    draw::draw_window();
}
