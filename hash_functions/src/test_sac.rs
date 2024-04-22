use sha3::{Digest as Sha3Digest, Sha3_256};

pub fn test_sac() -> f32 {
    // Create vector of size 1000 with random bits
    let vec_size = 1000;
    let mut file_content = vec![0u8; vec_size as usize];
    for i in 0..vec_size {
        file_content[i as usize] = rand::random::<u8>() % 2;
    }
    // Hash the vector
    let digest = Sha3_256::digest(&file_content);
    let mut result_avg:f32 = 0.0;
    let iterations = 1000;
    for i in 0..iterations {
        let mut changed_message = file_content.clone();
        if file_content[i as usize] == 0 {
            changed_message[i as usize] = 1;
        } else {
            changed_message[i as usize] = 0;
        }
        let digest_changed = Sha3_256::digest(&changed_message);
        let xor_result = digest.iter().zip(digest_changed.iter()).map(|(a, b)| a ^ b);
        result_avg += xor_result.map(|x| x.count_ones() as u32).sum::<u32>() as f32;
    }
    result_avg = (result_avg as f32) / ((iterations as f32) * 256.0);
    println!("SAC: {}", result_avg);
    return result_avg;
}
