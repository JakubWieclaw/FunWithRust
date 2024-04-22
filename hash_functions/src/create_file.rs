use rand::Rng;
use std::fs::File;
use std::io::Write;

pub fn create_file(size_in_mb: u32) -> String {
    let file_name = format!("file_{}mb.txt", size_in_mb);
    let mut file = File::create(&file_name).expect("Unable to create file");
    let mut buffer = vec![0u8; size_in_mb as usize * 1024 * 1024];
    rand::thread_rng().fill(&mut buffer[..]);
    file.write_all(&buffer).expect("Unable to write file");
    println!("File created: {}", file_name);
    return file_name;
}