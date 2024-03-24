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

pub fn get_x(n: u32, range_start: u32, range_end: u32) -> u32 {
    for i in range_start..=range_end {
        if are_these_comprime(i, n) {
            return i;
        }
    }
    return 0;
}

fn are_these_comprime(x:u32, n: u32) -> bool {
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

    return false;
}
