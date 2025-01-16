pub fn main() {
    // Analysis of our input shows it does the following
    // Takes a and adds 365*7, we can this new value A
    // Effectively reads the bits least to most significant
    // and outputs as B
    // It restarts back to A once out of bits

    // Therefore, we want something like A = b'1010101010...' > 365*7

    // That's 2, 2+8, 2+8+16, 2+8+16+64...

    let threshold: u32 = 365 * 7;
    let mut a: u32 = 0;
    let mut place: u32 = 1;

    while a < threshold {
        a += (2u32).pow(place);
        place += 2;
    }

    let p1: u32 = a - threshold;

    println!("{p1}");
}
