mod hash_functions;
mod create_file;
mod test_sac;
mod check_collisions;
//pub mod structs;


fn main() {
    // Crerate files 1mb, 3mb and 10mb
    let one_mb_file = create_file::create_file(1);
    let three_mb_file = create_file::create_file(3);
    let ten_mb_file = create_file::create_file(10);

    // Test hash functions and measure times
    let md5_results = hash_functions::test_md5_hash(&one_mb_file, &three_mb_file, &ten_mb_file);
    let sha1_results = hash_functions::test_sha1_hash(&one_mb_file, &three_mb_file, &ten_mb_file);
    let sha256_results = hash_functions::test_sha256_hash(&one_mb_file, &three_mb_file, &ten_mb_file);
    let sha3_256_results = hash_functions::test_sha3_256_hash(&one_mb_file, &three_mb_file, &ten_mb_file);

    // Test SAC
    let sac_results = test_sac::test_sac();

    // Test collisions
    let number_of_bits = 12;
    let iterations = 1000;
    let collisions = check_collisions::check_collisions(number_of_bits, iterations);

}
