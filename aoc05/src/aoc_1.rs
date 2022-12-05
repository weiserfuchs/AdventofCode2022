use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc05.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let mut v_st: Vec<Vec<String>> = Vec::new();

    let arr = [1,5,9,13,17,21,25,29,33];
    for i in 0..arr.len() {
        v_st.insert(i,Vec::new());
        //print!("{i}");
    }
    //println!("");
    env::set_var("RUST_BACKTRACE", "1");
    for line in buffered.lines() {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            if s_line.contains("[") {
                for i in arr{
                    if s_line.chars().nth(i).expect("char").is_alphabetic() {
                        let tmp = s_line.chars().nth(i).expect("tmp").to_string();
                        let index = arr.iter().position(|&x| x == i).expect("msg");
                        //println!("{tmp} -- {index} -- {i}");
                        v_st.get_mut(index).expect("").insert(0,tmp);
                    }else{
                        //println!("Ahhhh {i}");
                    }
                }
            }else if s_line.contains("move") {
                //for i in 0..arr.len(){
                //    for k in v_st.get(i).expect("msg") {
                //        print!("{k}")
                //    }
                //    println!("")
                //}
                let split = s_line.split(' ');
                let vec: Vec<&str> = split.collect();
                let i_from:usize = vec.get(3).expect("").parse().expect("msg");
                let i_to:usize = vec.get(5).expect("").parse().expect("msg");
                let i_amount:usize = vec.get(1).expect("").parse().expect("msg");
                //println!("{i_from} -> {i_to} -> {i_amount}");
                let v_from = v_st.get_mut(i_from-1).expect("");
                let mut tmp = Vec::new();
                for _i in 0..i_amount{
                    tmp.push(v_from.pop().expect("{_i}"))
                }
                let v_to = v_st.get_mut(i_to-1).expect("panic");
                for t in tmp{
                    //println!("move {t}");
                    v_to.insert(v_to.len(), t);
                }
                //for i in 0..arr.len(){
                //    for k in v_st.get(i).expect("msg") {
                //        print!("{k}")
                //    }
                //    println!("")
                //}

            }else {
                
            }
        }
    }
    for i in 0..arr.len(){
        print!("{}",v_st.get(i).expect("msg").last().expect("last"));
    }
    println!("");

}
