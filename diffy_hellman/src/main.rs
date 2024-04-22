mod helpers;
use std::io;
pub mod user;

fn main() {

    let range_start: u32;
    let range_end: u32;

    println!("Insert start of range for prime numbers: ");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    range_start = line.trim().parse().expect("Not a number");
    line.clear();


    println!("Insert end of range for prime numbers: ");
    io::stdin().read_line(&mut line).expect("Failed to read line");
    range_end = line.trim().parse().expect("Not a number");
    line.clear();


    if range_start > range_end {
        println!("Range start bigger then the end. Exiting program");
        return;
    }


    let primes = helpers::eratostenes_generate_vector_of_primes(range_start, range_end);
    let n = helpers::pick_random_from_vector(&primes);
    let g = helpers::count_primitive_root(&primes, n);
    println!("Generated prime number: {}", n);
    println!("Generated primitive root modulo n: {}", g);
    let mut user_a = user::User::new(n, g, range_start, range_end);
    let mut user_b = user::User::new(n, g, range_start, range_end);

    user_a.generate_shared_secret(user_b.get_public_key());
    user_b.generate_shared_secret(user_a.get_public_key());

    println!("Shared secret for user 1: {}", user_a.get_shared_secret());
    println!("Shared secret for user 2: {}", user_b.get_shared_secret());

    user_a.count_key();
    user_b.count_key();

    println!("Key for user 1: {}", user_a.get_k());
    println!("Key for user 2: {}", user_b.get_k());

//     println!("Insert message to encrypt: ");
//     io::stdin().read_line(&mut line).expect("Failed to read line");
//     let encrypted_message = user_a.encrypt_message_block_cipher(&line.trim().to_string());
//     println!("Encrypted message: ");
//     for c in &encrypted_message {
//         print!("{} ", c);
//     }
//     println!();

//     let decrypted_message = user_b.decrypt_message_block_cipher(&encrypted_message);
//     println!("Decrypted message: {}", decrypted_message);
}
