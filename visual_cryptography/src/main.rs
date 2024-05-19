
mod helpers;

fn main() {
    // read the image from resources
    let path: String = "src/resources/image2.png".to_string();
    let img_data = helpers::read_100x100_image(&path);

    // create two shares
    let shares = helpers::create_two_shares(&img_data);

    // cypher the shares
    helpers::cypher_shares(shares.clone(), img_data);

    println!("The image has been cyphered successfully!");

    // decrypt the shares
    let combined_shares = helpers::combine_shares(&shares[0], &shares[1]);

    // write the decrypted image
    let path: String = "src/resources/decrypted_image.jpeg".to_string();
    helpers::write_image(combined_shares, &path);
}
