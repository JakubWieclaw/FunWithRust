
use rand::Rng;
use crate::user::User;

pub fn generate_secret_message(number_of_users_required_to_decipher: u32) -> u32 {
    format!("{} users are required to decipher this message", number_of_users_required_to_decipher);
    // return random number
    let random_number = rand::thread_rng().gen_range(1000..9999);
    println!("The generated secret is {}", random_number);
    return random_number;
}

pub fn eratostenes_generate_vector_of_primes(start_of_range: u32, end_of_range: u32) -> Vec<u32> {
    let mut primes: Vec<bool> = vec![true; (end_of_range + 1) as usize];

    let mut p: usize = 2;
    while p * p < (end_of_range as usize) {
        if primes[p] {
            for i in (p * p..=(end_of_range as usize)).step_by(p) {
                primes[i] = false
            }
        }
        p += 1;
    }
    let mut res: Vec<u32> = Vec::new();
    for i in (start_of_range as usize)..=(end_of_range as usize) {
        if primes[i] {
            res.push(i as u32);
        }
    }

    return res;
}

pub fn get_random_prime_from_vector(primes: Vec<u32>) -> u32 {
    let random_index = rand::thread_rng().gen_range(0..primes.len());
    println!("Random prime: {}", primes[random_index]);
    return primes[random_index];
}

pub fn divide_secret_for_t_shares(secret: u32, t: u32, n: u32, random_prime: u32) -> Vec<User> {
    let mut coefficients: Vec<u32> = Vec::new();
    coefficients.push(secret);
    for _ in 1..t {
        coefficients.push(rand::thread_rng().gen_range(1..1000));
    }

    let mut shares: Vec<User> = Vec::new();
    for i in 1..=n {
        let mut share: i64 = secret as i64;
        for j in 1..(t-1) {
            share += (coefficients[j as usize] as i64 * i.pow(j) as i64) % random_prime as i64;
        }
        share %= random_prime as i64;
        let user = User {
            id: i,
            share: share as u32,
        };
        println!("User {} has share {}", user.id, user.share);
        shares.push(user);
    }
    return shares;
}

pub fn decipher_secret_from_shares(shares: Vec<User>, t: u32, p: u32) -> u32 {
    let mut secret: i64 = 0;
    for i in 0..t as usize {
        let mut lagrange: i64 = 1;
        for j in 0..t as usize {
            if i != j {
                lagrange *= -(shares[j].id as i64) * mod_inverse((shares[i].id as i64 - shares[j].id as i64) as i64, p as i64);
            }
        }
        secret += shares[i].share as i64 * lagrange;
    }
    secret %= p as i64;
    if secret < 0 {
        secret += p as i64;
    }
    println!("Deciphered secret: {}", secret as u32);
    return secret as u32;
}


fn mod_inverse(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

