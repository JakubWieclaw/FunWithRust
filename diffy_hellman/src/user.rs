use crate::helpers;

pub struct User {
    n: u128, // big prime number
    g: u128, // primitive root modulo n
    private_key: u128, // private key
    shared_secret: u128, // shared secret
    k: u128, // key for session
}

impl User {
    pub fn new(n: u128, g: u128, start: u32, end: u32) -> User {
        let private_key = helpers::get_number_from_range(start, end);
        return User {
            n: n,
            g: g,
            private_key: private_key,
            shared_secret: 0,
            k: 0,
        };
    }

    pub fn generate_shared_secret(&mut self, other_public_key: u128) {
        self.shared_secret = helpers::smart_power_modulo(other_public_key, self.private_key, self.n);
    }

    pub fn get_shared_secret(&self) -> u128 {
        return self.shared_secret;
    }

    pub fn count_key(&mut self) {
        self.k = helpers::smart_power_modulo(self.shared_secret, self.private_key, self.n);
    }

    pub fn get_public_key(&self) -> u128 {
        return helpers::smart_power_modulo(self.g, self.private_key, self.n);
    }

    // pub fn encrypt_message_block_cipher(&self, message: &str) -> Vec<u128> {
    //     let mut encrypted_message: Vec<u128> = Vec::new();
    //     let chars: Vec<char> = message.chars().collect();
    //     for i in (0..chars.len()).step_by(2) {
    //         let block = if i + 1 < chars.len() {
    //             (chars[i] as u128) << 8 | chars[i + 1] as u128
    //         } else {
    //             chars[i] as u128
    //         };
    //         let encrypted_block = helpers::smart_power_modulo(block, self.k, self.n);
    //         encrypted_message.push(encrypted_block);
    //     }
    //     return encrypted_message;
    // }

    // pub fn decrypt_message_block_cipher(&self, message: &Vec<u128>) -> String {
    //     let mut decrypted_message = String::new();
    //     for &c in message {
    //         let decrypted_block = helpers::smart_power_modulo(c, self.k, self.n);
    //         let chars = [(decrypted_block >> 8) as u8 as char, (decrypted_block & 0xFF) as u8 as char];
    //         decrypted_message.push(chars[0]);
    //         if chars[1] != '\0' {
    //             decrypted_message.push(chars[1]);
    //         }
    //     }
    //     return decrypted_message;
    // }

    pub fn get_k(&self) -> u128 {
        return self.k;
    }
}