use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc02.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut i_score = 0;
    let mut i_cur = 0;
    let mut s_strategy = "";
    let mut win = HashMap::new();
    win.insert(String::from("A"),String::from("Y"));
    win.insert(String::from("B"),String::from("Z"));
    win.insert(String::from("C"),String::from("X"));
    let mut loose = HashMap::new();
    loose.insert(String::from("A"),String::from("Z"));
    loose.insert(String::from("B"),String::from("X"));
    loose.insert(String::from("C"),String::from("Y"));
    
    for line in buffered.lines() {
        let s_line = line.unwrap();

        if s_line.len() > 0{
            if s_line.ends_with("X") {
                if s_line.starts_with("A") {
                    s_strategy = loose.get("A").unwrap();
                }else if s_line.starts_with("B") {
                    s_strategy = loose.get("B").unwrap();
                }else{
                    s_strategy = loose.get("C").unwrap();
                }
            }else if s_line.ends_with("Y") {
                i_cur += 3;
                if s_line.starts_with("A") {
                    i_cur += 1;
                }else if s_line.starts_with("B") {
                    i_cur += 2;
                }else{
                    i_cur += 3;
                }
            }else{
                //Win
                i_cur += 6;
                if s_line.starts_with("A") {
                    s_strategy = win.get("A").unwrap();
                }else if s_line.starts_with("B") {
                    s_strategy = win.get("B").unwrap();
                }else{
                    s_strategy = win.get("C").unwrap();
                }
            }
            if s_strategy == "X"{
                i_cur += 1;
            }else if s_strategy == "Y" {
                i_cur += 2;
            }else if s_strategy == "Z" {
                i_cur += 3;
            }
        }
        //println!("{s_line} -{s_strategy}- {i_cur}");
        i_score += i_cur;
        i_cur = 0;
        s_strategy = "";
    }
    println!("{i_score}");
    println!("");
}