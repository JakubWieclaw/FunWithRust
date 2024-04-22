use sha1::digest::generic_array::iter;
use sha3::{Digest as Sha3Digest, Sha3_256};
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn test_sac(path: &str) -> f32 {
    // Create a for loop to create a file with exactly 1 bit difference from the original file (the 1st bit, 2nd bit, 3rd bit, etc.)
    // hash both files and compare the hash results through XOR
    let original_content = fs::read(path).expect("Unable to read file");
    let original_digest = Sha3_256::digest(&original_content);
    // // Create a file to store the results
    // let mut res_file = File::create("sac_results.txt").expect("Unable to create file");
    let mut result_avg:f32 = 0.0;
    let iterations = 1000;
    for i in 0..iterations {
        let mut file_content = std::fs::read(path).expect("Unable to read file");
        file_content[i] = !file_content[i];
        let digest = Sha3_256::digest(&file_content);
        let xor_result = original_digest.iter().zip(digest.iter()).map(|(a, b)| a ^ b).collect::<Vec<u8>>();
        // Sum the vector and print result into file
        result_avg += xor_result.iter().sum::<u8>() as f32;
    }
    result_avg = (result_avg as f32) / (iterations as f32);
    println!("SAC: {}", result_avg);
    return result_avg;
}