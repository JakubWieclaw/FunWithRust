use crate::structs::zeroes_and_ones::Zeroes_and_Ones;
use crate::structs::series_len::SeriesLength;


pub fn single_bits_test(bbs_vec: &[u8; 20000]) -> Zeroes_and_Ones{
    let mut res: Zeroes_and_Ones = {Zeroes_and_Ones {zeroes: 0, ones: 0}};
    for el in bbs_vec {
        if *el == 0 {
            res.zeroes += 1;
        }
        else {
            res.ones += 1;
        }
    }

    return res;
}

pub fn conduct_single_bits_test(bbs_vec: &[u8; 20000]) {
    let single_bits_test: Zeroes_and_Ones = single_bits_test(bbs_vec);
    println!("SINGLE BITS TEST");
    println!("Zeroes: {}\nOnes: {}", single_bits_test.zeroes, single_bits_test.ones);
    if single_bits_test.zeroes > 9725 && single_bits_test.zeroes < 10275 {
        println!("SUCCESS");
    }
    else {
        println!("FAILED");
    }

    println!();
}

pub fn long_series_test(bbs_vec: &[u8; 20000]) -> u32 {
    let mut res: u32 = 1;
    let mut temp: u32 = 1;
    for i in 1..bbs_vec.len() {
        if bbs_vec[i] == bbs_vec[i-1] {
            temp += 1;
        }
        else {
            temp = 1;
        }

        if temp > res {
            res = temp;
        }
    }

    return res;
}

pub fn conduct_long_series_test(bbs_vec: &[u8; 20000]) {
    println!("LONG SERIES TEST");
    let long_series_res: u32 = long_series_test(bbs_vec);
    println!("Longest subsequence: {long_series_res}");
    if long_series_res < 26 {
        println!("SUCCESS");
    }
    else {
        println!("FAILED");
    }

    println!();
}

pub fn runs_test(bbs_vec: &[u8; 20000]) -> (SeriesLength, SeriesLength) {
    let mut res_zeroes: SeriesLength = {SeriesLength {one: 0, two: 0, three: 0, four: 0, five: 0, six_and_more: 0}};
    let mut res_ones: SeriesLength = {SeriesLength {one: 0, two: 0, three: 0, four: 0, five: 0, six_and_more: 0}};
    let mut temp: u32 = 1;
    for i in 1..bbs_vec.len() {
        if bbs_vec[i] == bbs_vec[i-1] {
            temp += 1;
        }
        else {
            if bbs_vec[i-1] == 0 {
                match temp {
                    1 => res_zeroes.one += 1,
                    2 => res_zeroes.two += 1,
                    3 => res_zeroes.three += 1,
                    4 => res_zeroes.four += 1,
                    5 => res_zeroes.five += 1,
                    _ => res_zeroes.six_and_more += 1,
                }
            }
            else {
                match temp {
                    1 => res_ones.one += 1,
                    2 => res_ones.two += 1,
                    3 => res_ones.three += 1,
                    4 => res_ones.four += 1,
                    5 => res_ones.five += 1,
                    _ => res_ones.six_and_more += 1,
                }
            }
            
            temp = 1;
        }
    }

    return (res_zeroes, res_ones);
}

pub fn conduct_runs_test(bbs_vec: &[u8; 20000]) {
    println!("RUNS TEST");
    let runs_test_res: (SeriesLength, SeriesLength) = runs_test(bbs_vec);
    let succes_zeroes = check_single_series_length(&runs_test_res.0);
    let success_ones = check_single_series_length(&runs_test_res.1);
    println!("ZEROES\nLength 1: {}\nLength 2: {}\nLength 3: {}\nLength 4: {}\nLength 5: {}\nLength 6 or more: {}", runs_test_res.0.one, runs_test_res.0.two, runs_test_res.0.three, runs_test_res.0.four, runs_test_res.0.five, runs_test_res.0.six_and_more);
    println!("\nONES\nLength 1: {}\nLength 2: {}\nLength 3: {}\nLength 4: {}\nLength 5: {}\nLength 6 or more: {}", runs_test_res.1.one, runs_test_res.1.two, runs_test_res.1.three, runs_test_res.1.four, runs_test_res.1.five, runs_test_res.1.six_and_more);
    if succes_zeroes && success_ones{
        println!("SUCCESS");
    }
    else {
        println!("FAILED");
    }

    println!();
}

fn check_single_series_length (s: &SeriesLength) -> bool {
    if s.one >= 2315 && s.one <= 2685 {
        if s.two >= 1114 && s.two <= 1386 {
            if s.three >= 527 && s.three <= 723 {
                if s.four >= 240 && s.four <= 384 {
                    if s.five >= 103 && s.five <= 209 {
                        if s.six_and_more >= 103 && s.six_and_more <= 209 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}