use std::vec;

use image::{self, GrayImage, ImageBuffer, Luma};
use rand::Rng;

pub fn read_100x100_image(path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    // // Print current directory
    // println!("Current directory: {:?}", std::env::current_dir().unwrap());
    // // Print path of the image that we are trying to read
    // println!("Path of the image: {:?}", path);
    // Combibe these preious paths to get the full path of the image (don't forget to add / between them)
    let image_path = format!(
        "{}/{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        path
    );
    // println!("Full path of the image: {:?}", image_path);
    // // Replace first space in a path with underscore
    let image_path = image_path.replace(" ", "_");
    println!(
        "Full path of the image after replacing spaces: {:?}",
        image_path
    );
    let img = image::open(image_path).unwrap().to_luma8();
    return img;
}

pub fn create_two_shares(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<ImageBuffer<Luma<u8>, Vec<u8>>> {
    let (width, height) = img.dimensions();
    let share1 = GrayImage::new(2 * width, height);
    let share2 = GrayImage::new(2 * width, height);
    let shares = vec![share1, share2];
    return vec![shares[0].clone(), shares[1].clone()];
}

pub fn write_two_shares(
    share1: ImageBuffer<Luma<u8>, Vec<u8>>,
    share2: ImageBuffer<Luma<u8>, Vec<u8>>,
    share1_path: &str,
    share2_path: &str,
) {
    share1.save(share1_path).unwrap();
    share2.save(share2_path).unwrap();
}

pub fn cypher_shares(mut shares: Vec<ImageBuffer<Luma<u8>, Vec<u8>>>, img: ImageBuffer<Luma<u8>, Vec<u8>>) {
    let (width, height) = shares[0].dimensions();
    let color_white = Luma([255]);
    let color_black = Luma([0]);
    let mut rng = rand::thread_rng();
    for x in (0..width as usize).step_by(2) {
        for y in 0..height {
            let pixel = img.get_pixel((x as u32) / 2, y);
            let pixel_value = pixel.0[0];
            if pixel_value < 128 {
                if rng.gen() {
                    shares[0].put_pixel(x as u32, y, color_black);
                    shares[0].put_pixel((x + 1) as u32, y, color_white);
                    shares[1].put_pixel(x as u32, y, color_white);
                    shares[1].put_pixel((x + 1) as u32, y, color_black);
                } else {
                    shares[0].put_pixel(x as u32, y, color_white);
                    shares[0].put_pixel((x + 1) as u32, y, color_black);
                    shares[1].put_pixel(x as u32, y, color_black);
                    shares[1].put_pixel((x + 1) as u32, y, color_white);
                }
            } else {
                if rng.gen() {
                    shares[0].put_pixel(x as u32, y, color_black);
                    shares[0].put_pixel((x + 1) as u32, y, color_white);
                    shares[1].put_pixel(x as u32, y, color_black);
                    shares[1].put_pixel((x + 1) as u32, y, color_white);
                } else {
                    shares[0].put_pixel(x as u32, y, color_white);
                    shares[0].put_pixel((x + 1) as u32, y, color_black);
                    shares[1].put_pixel(x as u32, y, color_white);
                    shares[1].put_pixel((x + 1) as u32, y, color_black);
                }
            }
        }
    }
    write_two_shares(
        shares[0].clone(),
        shares[1].clone(),
        "src/resources/share1.jpeg",
        "src/resources/share2.jpeg",
    )
}

pub fn combine_shares(share1: &ImageBuffer<Luma<u8>, Vec<u8>>, share2: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = share1.dimensions();
    let mut img = GrayImage::new(width / 2, height);
    for x in (0..width as usize).step_by(2) {
        for y in 0..height {
            let pixel1 = share1.get_pixel(x as u32, y);
            let pixel2 = share2.get_pixel(x as u32, y);
            let pixel_value1 = pixel1.0[0];
            let pixel_value2 = pixel2.0[0];
            if pixel_value1 == 0 && pixel_value2 == 255 {
                img.put_pixel((x as u32) / 2, y, Luma([0]));
            } else if pixel_value1 == 255 && pixel_value2 == 0 {
                img.put_pixel((x as u32) / 2, y, Luma([255]));
            } else {
                img.put_pixel((x as u32) / 2, y, Luma([0]));
            }
        }
    }
    return img;
}

pub fn write_image(img: ImageBuffer<Luma<u8>, Vec<u8>>, path: &str) {
    img.save(path).unwrap();
}