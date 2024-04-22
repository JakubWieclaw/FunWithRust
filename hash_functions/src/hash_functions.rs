//use crate::structs::hash_function_result::HashFunctionResult;
use md5;
use sha2::Sha256;
use sha1::{Digest as Sha1Digest, Sha1};
use sha3::{Digest as Sha3Digest, Sha3_256};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::time::Duration;

pub struct HashFunctionResult {
    pub hash_type: String,
    pub time_1mb: Duration,
    pub time_3mb: Duration,
    pub time_10mb: Duration,
}

fn md5_hash(file_name: &str, output_file: &str) {
    let file_content = std::fs::read(file_name).expect("Unable to read file");
    let digest = md5::compute(file_content);
    let mut file = File::create(output_file).expect("Unable to create file");
    write!(file, "{:x}", digest).expect("Unable to write data");
}

pub fn test_md5_hash(
    one_mb_file: &str,
    three_mb_file: &str,
    ten_mb_file: &str,
) -> Vec<HashFunctionResult> {
    let start = Instant::now();
    md5_hash(one_mb_file, "md5_1mb.txt");
    let time_1mb = start.elapsed();
    let start = Instant::now();
    md5_hash(three_mb_file, "md5_3mb.txt");
    let time_3mb = start.elapsed();
    let start = Instant::now();
    md5_hash(ten_mb_file, "md5_10mb.txt");
    let time_10mb = start.elapsed();
    let mut hash_results = Vec::new();
    hash_results.push(HashFunctionResult {
        hash_type: "md5".to_string(),
        time_1mb,
        time_3mb,
        time_10mb,
    });

    println!("MD5");
    for result in hash_results.iter() {
        println!("1mb: {:?}, 3mb: {:?}, 10mb: {:?}", result.time_1mb, result.time_3mb, result.time_10mb);
    }
    return hash_results;
}

pub fn sha1_hash(file_name: &str, output_file: &str) {
    let file_content = std::fs::read(file_name).expect("Unable to read file");
    let mut hasher = Sha1::new();
    hasher.update(&file_content);
    let result = hasher.finalize();
    let mut file = File::create(output_file).expect("Unable_to_create_file");
    write!(file, "{:x}", result).expect("Unable_to_write_data");
}

pub fn test_sha1_hash(
    one_mb_file: &str,
    three_mb_file: &str,
    ten_mb_file: &str,
) -> Vec<HashFunctionResult> {
    let start = Instant::now();
    sha1_hash(one_mb_file, "sha1_1mb.txt");
    let time_1mb = start.elapsed();
    let start = Instant::now();
    sha1_hash(three_mb_file, "sha1_3mb.txt");
    let time_3mb = start.elapsed();
    let start = Instant::now();
    sha1_hash(ten_mb_file, "sha1_10mb.txt");
    let time_10mb = start.elapsed();
    let mut hash_results = Vec::new();
    hash_results.push(HashFunctionResult {
        hash_type: "sha1".to_string(),
        time_1mb,
        time_3mb,
        time_10mb,
    });

    println!("SHA1");
    for result in hash_results.iter() {
        println!("1mb: {:?}, 3mb: {:?}, 10mb: {:?}", result.time_1mb, result.time_3mb, result.time_10mb);
    }
    return hash_results;
}

fn sha256_hash(file_name: &str, output_file: &str) {
    let file_content = std::fs::read(file_name).expect("Unable to read file");
    let digest = Sha256::digest(&file_content);
    let mut file = File::create(output_file).expect("Unable to create file");
    write!(file, "{:x}", digest).expect("Unable to write data");
}

pub fn test_sha256_hash(
    one_mb_file: &str,
    three_mb_file: &str,
    ten_mb_file: &str,
) -> Vec<HashFunctionResult> {
    let start = Instant::now();
    sha256_hash(one_mb_file, "sha256_1mb.txt");
    let time_1mb = start.elapsed();
    let start = Instant::now();
    sha256_hash(three_mb_file, "sha256_3mb.txt");
    let time_3mb = start.elapsed();
    let start = Instant::now();
    sha256_hash(ten_mb_file, "sha256_10mb.txt");
    let time_10mb = start.elapsed();
    let mut hash_results = Vec::new();
    hash_results.push(HashFunctionResult {
        hash_type: "sha256".to_string(),
        time_1mb,
        time_3mb,
        time_10mb,
    });

    println!("SHA2-256");
    for result in hash_results.iter() {
        println!("1mb: {:?}, 3mb: {:?}, 10mb: {:?}", result.time_1mb, result.time_3mb, result.time_10mb);
    }
    return hash_results;
}

pub fn sha3_256_hash(file_name: &str, output_file: &str) {
    let file_content = std::fs::read(file_name).expect("Unable to read file");
    let mut hasher = Sha3_256::new();
    hasher.update(&file_content);
    let result = hasher.finalize();
    let mut file = File::create(output_file).expect("Unable_to_create_file");
    write!(file, "{:x}", result).expect("Unable_to_write_data");
}

pub fn test_sha3_256_hash(
    one_mb_file: &str,
    three_mb_file: &str,
    ten_mb_file: &str,
) -> Vec<HashFunctionResult> {
    let start = Instant::now();
    sha3_256_hash(one_mb_file, "sha3_256_1mb.txt");
    let time_1mb = start.elapsed();
    let start = Instant::now();
    sha3_256_hash(three_mb_file, "sha3_256_3mb.txt");
    let time_3mb = start.elapsed();
    let start = Instant::now();
    sha3_256_hash(ten_mb_file, "sha3_256_10mb.txt");
    let time_10mb = start.elapsed();
    let mut hash_results = Vec::new();
    hash_results.push(HashFunctionResult {
        hash_type: "sha3_256".to_string(),
        time_1mb,
        time_3mb,
        time_10mb,
    });

    println!("SHA3-256");
    for result in hash_results.iter() {
        println!("1mb: {:?}, 3mb: {:?}, 10mb: {:?}", result.time_1mb, result.time_3mb, result.time_10mb);
    }
    return hash_results;
}
