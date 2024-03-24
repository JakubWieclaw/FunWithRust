mod algorythm_functions; 
mod pick_random;
use std::io;
pub mod structs;
use bitvector::BitVector;

fn main() {
    let range_start:u32;
    let range_end:u32;

    let mut input_line = String::new();

    println!("Insert start of range: ");
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    range_start = input_line.trim().parse().expect("Start of range parse failed, presumably input is not u32 type");

    input_line.clear();

    println!("Insert end of range: ");
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    range_end= input_line.trim().parse().expect("Start of range parse failed, presumably input is not u32 type");

    if range_start > range_end{
        println!("Range start bigger then the end. Exiting program");
        return;
    }

    //let primes_vec = database_gen::eratostenes_generate_vector_of_primes(range_start, range_end);
    // database_gen::save_vector_as_csv(primes_vec);

    let primes_vec = algorythm_functions::eratostenes_generate_vector_of_primes(range_start, range_end);

    let p = pick_random::pick_random_from_vector(&primes_vec, 4, 3);
    let q = pick_random::pick_random_from_vector(&primes_vec, 4, 3);
    let n = p*q;
    let x = algorythm_functions::get_x(n, range_start, range_end);

    let mut x_i = (x*x)%n;

    let bv_size = 20000;
    let mut bitvec = BitVector::new(bv_size);
    for _i in 0..bv_size {
        let val = x_i%2;
        bitvec.insert(val as usize);
        x_i = (x_i*x_i)%n;
    }

    println!("{bitvec}");
}
