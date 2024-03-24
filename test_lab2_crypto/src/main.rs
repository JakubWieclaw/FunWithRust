mod algorythm_functions;
mod pick_random;
mod tests;
use std::io;
pub mod structs;
//use bitvector::BitVector;

fn main() {
    let range_start: u32;
    let range_end: u32;

    let mut input_line = String::new();

    println!("Insert start of range: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    range_start = input_line
        .trim()
        .parse()
        .expect("Start of range parse failed, presumably input is not u32 type");

    input_line.clear();

    println!("Insert end of range: ");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    range_end = input_line
        .trim()
        .parse()
        .expect("Start of range parse failed, presumably input is not u32 type");

    if range_start > range_end {
        println!("Range start bigger then the end. Exiting program");
        return;
    }


    let primes_vec =
        algorythm_functions::eratostenes_generate_vector_of_primes(range_start, range_end);

    let p: u128 = pick_random::pick_random_from_vector(&primes_vec, 4, 3);
    let q: u128 = pick_random::pick_random_from_vector(&primes_vec, 4, 3);
    let n: u128 = p * q;
    let x: u128 = algorythm_functions::get_x(n, range_start, range_end);

    println!("p: {p}\nq: {q}\nn: {n}\nx: {x}\n");

    let mut x_i = (x * x) % n;

    let mut bbs: [u8; 20000] = [0; 20000];
    for i in 0..20000 {
        bbs[i] = (x_i % 2) as u8;
        x_i = (x_i * x_i) % n;
    }

    tests::conduct_single_bits_test(&bbs);
    tests::conduct_runs_test(&bbs);
    tests::conduct_long_series_test(&bbs);
}
