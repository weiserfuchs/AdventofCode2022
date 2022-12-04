use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc04.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut i_pairs = 0;
    let mut first:i32 = 0;
    let mut second:i32 = 0;
    let mut first2:i32 = 0;
    let mut second2:i32 = 0;
    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            let split = s_line.split(',');
            let mut b_first = true;
            for s in split{
                if b_first {
                    let k = s.split('-');
                    let vec: Vec<&str> = k.collect();
                    first = vec.first().expect("").parse().expect("");
                    second = vec.last().expect("").parse().expect("");
                    b_first = false;
                }else{
                    let k = s.split('-');
                    let vec: Vec<&str> = k.collect();
                    first2 = vec.first().expect("").parse().expect("");
                    second2 = vec.last().expect("").parse().expect("");
                    if first >= first2 && first <= second2 {
                        i_pairs += 1;
                        break;
                    }else if first2 >= first && first2 <= second{
                        i_pairs += 1;
                        break;
                    }else if second >= first2 && second <= second2{
                        i_pairs += 1;
                        break;
                    }else if second2 >= first && second2 <= second{
                        i_pairs += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{i_pairs}");
    println!("");
}