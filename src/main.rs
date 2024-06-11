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
}
