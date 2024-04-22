use rand::Rng;

pub fn smart_power_modulo(base: u128, exp: u128, modulo: u128) -> u128 {
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

pub fn pick_random_from_vector(v: &Vec<u32>) -> u128 {
    let random_id = rand::thread_rng().gen_range(0..v.len());
    return v[random_id] as u128;
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

pub fn get_number_from_range(start: u32, end: u32) -> u128 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(start..end) as u128;
}

pub fn count_primitive_root(primes: &Vec<u32>, n: u128) -> u128 {
    let phi: u128 = n - 1;
    //let primes = eratostenes_generate_vector_of_primes(2, n as u32);
    let mut result: u128 = 0;
    for i in 0..primes.len() {
        let mut flag: bool = true;
        for j in 0..primes.len() {
            if (phi / primes[i] as u128) % primes[j] as u128 == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            result = primes[i] as u128;
            break;
        }
    }
    return result;
}