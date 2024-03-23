use crate::structs::pair::Pair;
use csv::Writer;
// use std::error::Error;

pub fn generate_vector_of_primes(start_of_range: u32, end_of_range: u32) -> Vec<Pair> {
    let mut primes: Vec<Pair> = Vec::new();
    let mut id:u32 = 0;
    for i in start_of_range..=end_of_range {
        if is_prime(i){
            primes.push(Pair{id: id, number: i});
            id += 1;
        }
    }

    return primes;
}

fn is_prime(number: u32) -> bool{
    if number <=2 {
        return false;
    }
    let mut i = 2;
    while i*i < number {
        if number%i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn save_vector_as_csv(vector: Vec<Pair>) {
    let mut wtr = Writer::from_path("resources/primes.csv").expect("Unable to open the file");
    let vector_iter = vector.iter();

    for el in vector_iter {
        wtr.serialize(el).expect("Failed to write to csv");
    }
    wtr.flush().expect("Flush failed");
}