pub fn check_collisions(number_of_bits: u8, iterations:u32) -> u32 {
    let mut result = 0;
    let vec_size = 1000;

    for _ in 0..iterations {
        // Create 2 random bits vectors, hash them and compare their first number_of_bits, if they are the same, increment result
        let mut file_content1 = vec![0u8; vec_size as usize];
        let mut file_content2 = vec![0u8; vec_size as usize];

        for j in 0..vec_size {
            file_content1[j as usize] = rand::random::<u8>() % 2;
            file_content2[j as usize] = file_content1[j as usize];
        }

        // Flip one random bit in file_content2
        let bit_to_flip = rand::random::<usize>() % vec_size;
        file_content2[bit_to_flip] ^= 1;

        let digest1 = md5::compute(&file_content1);
        let digest2 = md5::compute(&file_content2);

        let mut collision = true;
        for j in 0..(number_of_bits / 8) as usize {
            if digest1[j] != digest2[j] {
                collision = false;
                break;
            }
        }
        if collision {
            result += 1;
        }
    }
    println!("Collisions: {}", result);
    return result;
}