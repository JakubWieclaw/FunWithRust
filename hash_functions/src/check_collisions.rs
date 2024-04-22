pub fn check_collisions(number_of_bits: u8, iterations:u32) -> u32 {
    let mut result = 0;

    for i in 0..iterations {
        // Create 2 random bits vectors, hash them and compare their first number_of_bits, if they are the same, increment result
        let mut file_content1 = vec![0u8; number_of_bits as usize];
        let mut file_content2 = vec![0u8; number_of_bits as usize];

        for j in 0..number_of_bits {
            file_content1[j as usize] = rand::random::<u8>();
            file_content2[j as usize] = rand::random::<u8>();
        }

        let digest1 = md5::compute(&file_content1);
        let digest2 = md5::compute(&file_content2);

        let mut collision = true;
        for j in 0..number_of_bits {
            if digest1[j as usize] != digest2[j as usize] {
                collision = false;
                break;
            }
        }

        result += collision as u32;
    }

    println!("Collisions: {}", result);
    return result;
}