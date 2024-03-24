//use csv::Reader;
use rand::Rng;

pub fn pick_random_from_vector(v: &Vec<u32>, mod_n: u32, mod_eq: u32) -> u32 {

    let mut candidates: Vec<u32> = vec![];

    for n in v {
        if n%mod_n == mod_eq {
            candidates.push(*n);
        }
    }

    let random_id = rand::thread_rng().gen_range(0..candidates.len());
    return candidates[random_id];
}

// pub fn pick_random_number_from_database(path: &str, size: &u32, mod_n: u32, mod_eq: u32) -> u32 {

//     let mut reader = Reader::from_path(path).expect("Failed to read from database");

//     let mut candidates: Vec<u32>;

//     for record in reader.records() {
//         let r = record.expect("Fail to assign record");
//         let n_str = r.get(1).ok_or("Failed to parse number");
//         let n: u32 = n_str.parse().map_err(|_| "Failed to parse number")?;
//         if n%mod_n == mod_eq {
//             candidates.push(n);
//         }
//     }

//     let random_prime = candidates.choose(&mut rand::thread_rng()).ok_or("No prime found with these conditions");

//     Ok(*random_prime)
// }