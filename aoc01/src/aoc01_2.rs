use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc01.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut count_elves = 0;
    let mut i_highest_elve = 0;
    let mut i_elve_cal = 0;
    let mut i_cal = 0;
    let mut b_last = false;
    let mut a_top3 = [0,0,0];
    
    for line in buffered.lines() {
        b_last = false;
        let s_line = line.unwrap();
        if s_line.len() > 0{
            let i_line:i32 = s_line.trim().parse().expect("");
            i_cal += i_line;
        }else {
            count_elves += 1;
            if i_cal > i_elve_cal {
                i_highest_elve = count_elves;
                i_elve_cal = i_cal;
            }
            if i_cal > a_top3[0] {
                a_top3[2] = a_top3[1];
                a_top3[1] = a_top3[0];
                a_top3[0] = i_cal;
            }else if i_cal > a_top3[1] {
                a_top3[2] = a_top3[1];
                a_top3[1] = i_cal;
            }else if i_cal > a_top3[2] {
                a_top3[2] = i_cal;
            }
            i_cal = 0;
            b_last = true;
        }
    }

    if b_last ==false {
        count_elves += 1;
        if i_cal > i_elve_cal {
            i_highest_elve = count_elves;
            i_elve_cal = i_cal;
        }
        if i_cal > a_top3[0] {
            a_top3[2] = a_top3[1];
            a_top3[1] = a_top3[0];
            a_top3[0] = i_cal;
        }else if i_cal > a_top3[1] {
            a_top3[2] = a_top3[1];
            a_top3[1] = i_cal;
        }else if i_cal > a_top3[2] {
            a_top3[2] = i_cal;
        }
    }
    println!("{i_highest_elve}");
    println!("{i_elve_cal}");
    let i_sum = a_top3[0] + a_top3[1] + a_top3[2];
    println!("{i_sum}");
}