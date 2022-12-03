use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc03.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let s_priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut i_counter = 0;
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();
    let mut i_priototal = 0;
    let mut i_prio;
    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            for character in s_line.chars(){
                if i_counter < (s_line.len()/2) {
                    //first
                    array1.push(character);
                    //println!("{character}")
                }else{
                    //second
                    if array1.contains(&character) && array2.contains(&character) == false {
                        i_prio = s_priority.find(character).expect("") + 1;
                        //println!("{i_priototal} + {i_prio} + {character}");
                        i_priototal += i_prio;
                        array2.push(character);
                    }
                }
                i_counter += 1;
            }
            i_counter = 0;
            array1 = Vec::new();
            array2 = Vec::new();
        }
    }
    println!("{i_priototal}");
    println!("");
}
