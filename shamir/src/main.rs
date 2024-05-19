mod helpers;
mod user;

fn main() {
    

    // Generate secret
    //let secret = helpers::generate_secret_message(3);
    let secret = 954;

    // Generate vector of primes
    let primes = helpers::eratostenes_generate_vector_of_primes(1000, 9999);

    // Get random prime from vector
    let random_prime = helpers::get_random_prime_from_vector(primes);

    // Divide secret for t shares
    let t = 3;
    let n = 4;
    let shares = helpers::divide_secret_for_t_shares(secret, t, n, random_prime);

    // Combine shares
    let secret = helpers::decipher_secret_from_shares(shares, t, random_prime);
}
