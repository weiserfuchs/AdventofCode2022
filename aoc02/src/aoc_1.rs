use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc02.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut i_score = 0;
    let mut i_cur = 0;
    
    for line in buffered.lines() {
        let s_line = line.unwrap();

        if s_line.len() > 0{
            if s_line.ends_with("Y"){
                //Paper
                i_cur += 2;
                if s_line.starts_with("A") {
                    i_cur += 6;
                }else if s_line.starts_with("B") {
                    i_cur += 3;
                }
            }else if s_line.ends_with("X") {
                //Rock
                i_cur += 1;
                if s_line.starts_with("A") {
                    i_cur += 3;
                }else if s_line.starts_with("C") {
                    i_cur += 6;
                }
            }else if s_line.ends_with("Z") {
                //Scissor
                i_cur += 3;
                if s_line.starts_with("C") {
                    i_cur += 3;
                }else if s_line.starts_with("B") {
                    i_cur += 6;
                }
            }
        }
        i_score += i_cur;
        i_cur = 0;
    }
    println!("{i_score}");
    println!("");
}
