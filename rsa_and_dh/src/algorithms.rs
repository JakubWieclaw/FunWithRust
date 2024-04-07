use rand::Rng;

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

// pub fn pick_random_from_vector_mod(v: &Vec<u32>, mod_n: u32, mod_eq: u32) -> u128 {
//     let mut candidates: Vec<u32> = vec![];

//     for n in v {
//         if n % mod_n == mod_eq {
//             candidates.push(*n);
//         }
//     }

//     let random_id = rand::thread_rng().gen_range(0..candidates.len());
//     return candidates[random_id] as u128;
// }

pub fn pick_random_from_vector(v: &Vec<u32>) -> u128 {
    let random_id = rand::thread_rng().gen_range(0..v.len());
    return v[random_id] as u128;
}

pub fn get_e(v: &Vec<u32>, phi: u128) -> u128 {
    loop {
        let id = rand::thread_rng().gen_range(0..v.len());
        let e = v[id];
        if are_these_comprime(e as u128, phi) {
            return e as u128;
        }
    }
}

fn are_these_comprime(x: u128, n: u128) -> bool {
    let mut x_val = x;
    let mut n_val = n;
    while n_val > 1 && x_val > 1 {
        if n_val > x_val {
            n_val = n_val - x_val;
        } else if x_val > n_val {
            x_val = x_val - n_val;
        } else if x == 1 || n == 1 {
            return true;
        } else {
            break;
        }
    }

    return x_val == 1 || n_val == 1;
}

pub fn get_d(v: &Vec<u32>, e: u128, phi: u128) -> u128 {
    let mut d: u128 = 0;
    let mut k: u128 = 1;
    loop {
        d = (k * phi + 1) / e;
        if (e * d) % phi == 1 {
            return d;
        }
        k += 1;
    }
    // loop {
    //     let d = pick_random_from_vector(v);
    //     if (e * d) % phi == 1 {
    //         return d;
    //     }
    // }
}

pub fn rsa(range_start: u32, range_end: u32) {

    println!("Preparing variables");
    println!("Generating vector of primes");
    let primes_vec = eratostenes_generate_vector_of_primes(range_start, range_end);
    println!("Generating p and q");
    let p: u128 = pick_random_from_vector(&primes_vec);
    let q: u128 = pick_random_from_vector(&primes_vec);
    let n: u128 = p * q;
    let phi = (p-1)*(q-1);
    println!("Generating e and d");
    let e = get_e(&primes_vec, phi); // Public key
    let d = get_d(&primes_vec, e, phi); // Private key
    println!("Variables prepared");
    println!("p: {}\nq: {}\nn: {}\nphi: {}\ne: {}\nd: {}", p, q, n, phi, e, d);

    let message: String = String::from("This is the message that will be encrypted and decrypted");

    let cyphered_message = rsa_cypher_message(&message, e, &n);
    let decyphered_message = rsa_decypher_message(&cyphered_message, d, &n);

    println!("Cyphered message: ");
    for c in cyphered_message {
        print!("{}, ", c);
    }
    println!();
    println!("Decyphered message: {:?}", decyphered_message);
}

fn rsa_cypher_message(message: &String, e: u128, n: &u128) -> Vec<u128> {
    let mut cyphered_message: Vec<u128> = Vec::new();
    println!("Encrypting message");
    for c in message.chars() {
        let c_as_u128 = c as u128;
        let cyphered_char = smart_power_modulo(c_as_u128, e, *n);
        cyphered_message.push(cyphered_char);
    }
    println!("Message encrypted");
    return cyphered_message;
}

fn rsa_decypher_message(cyphered_message: &Vec<u128>, d: u128, n: &u128) -> String {
    let mut decyphered_message: String = String::new();
    println!("Decripting message");
    for c in cyphered_message {
        let decyphered_char = smart_power_modulo(*c, d, *n);
        //println!("Decyphered char: {}", decyphered_char);
        decyphered_message.push(std::char::from_u32(decyphered_char as u32).unwrap());
    }
    println!("Message decrypted");
    return decyphered_message;
}

fn smart_power_modulo(base: u128, exp: u128, modulo: u128) -> u128 {
    let mut result: u128 = 1;
    let mut base = base % modulo;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp = exp >> 1;
        base = (base * base) % modulo;
    }
    return result;
}