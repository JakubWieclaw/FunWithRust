mod database_gen;
use std::io;
pub mod structs;

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

    let primes_vec = database_gen::generate_vector_of_primes(range_start, range_end);
    database_gen::save_vector_as_csv(primes_vec);
}
