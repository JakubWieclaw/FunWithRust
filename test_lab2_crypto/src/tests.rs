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

pub fn runs_test(bbs_vec: &[u8; 20000]) -> SeriesLength {
    let mut res: SeriesLength = {SeriesLength {one: 0, two: 0, three: 0, four: 0, five: 0, six_and_more: 0}};
    let mut temp: u32 = 1;
    for i in 1..bbs_vec.len() {
        if bbs_vec[i] == bbs_vec[i-1] {
            temp += 1;
        }
        else {
            match temp {
                1 => res.one += 1,
                2 => res.two += 1,
                3 => res.three += 1,
                4 => res.four += 1,
                5 => res.five += 1,
                _ => res.six_and_more += 1,
            }
            temp = 1;
        }
    }

    return res;
}

pub fn conduct_runs_test(bbs_vec: &[u8; 20000]) {
    println!("RUNS TEST");
    let mut success: bool = false;
    let runs_test_res: SeriesLength = runs_test(bbs_vec);
    if runs_test_res.one >= 2315 && runs_test_res.one <= 2685 {
        if runs_test_res.two >= 1114 && runs_test_res.two <= 1386 {
            if runs_test_res.three >= 527 && runs_test_res.three <= 723 {
                if runs_test_res.four >= 240 && runs_test_res.four <= 384 {
                    if runs_test_res.five >= 103 && runs_test_res.five <= 209 {
                        if runs_test_res.six_and_more >= 103 && runs_test_res.six_and_more <= 209 {
                            success = true;
                        }
                    }
                }
            }
        }
    }
    println!("Length 1: {}\nLength 2: {}\nLength 3: {}\nLength 4: {}\nLength 5: {}\nLength 6 or more: {}", runs_test_res.one, runs_test_res.two, runs_test_res.three, runs_test_res.four, runs_test_res.five, runs_test_res.six_and_more);
    if success {
        println!("SUCCESS");
    }
    else {
        println!("FAILED");
    }

    println!();
}