use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc06.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut v_char: Vec<char> = Vec::new();
    let mut ch:char;
    let mut b_hit:bool = true;
    let mut i_marker = 0;
    let i_group = 4;
    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            for i in 0..s_line.len()-i_group{
                for k in 0..i_group{
                    ch = s_line.chars().nth(i+k).expect("msg");
                    if v_char.contains(&ch) {
                        b_hit = false;
                        //println!("{:?}",v_char);
                        break;
                    }else{
                        v_char.push(ch);
                    }
                }
                if b_hit{
                    i_marker = i+i_group;
                    break;
                }else{
                    v_char = Vec::new();
                    b_hit = true;
                }
            }
        }
    }
    println!("{i_marker}");
    println!("");
}
