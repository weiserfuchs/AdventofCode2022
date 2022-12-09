use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let input = File::open("aoc08.txt").expect("Unable to open file");
    let buffered = BufReader::new(input);
    let vec: Vec<Result<String, std::io::Error>>  = buffered.lines().collect();
    let mut x;
    let mut y = 0;
    let i_counter = vec.len();
    let mut array = vec![vec![0; i_counter]; i_counter];
    print!("");
    for line in vec {
        let s_line = line.unwrap();
        if s_line.len() > 0{
            x = 0;
            for c in s_line.chars(){
                array[x][y] = c.to_digit(32).expect("");
                x += 1;
            }
        }
        y += 1;
    }
    let mut i_score = 0;
    let mut i_right = 0;
    let mut i_left = 0;
    let mut i_top = 0;
    let mut i_bottom = 0;
    for y in 1..i_counter-1{
        for x in 1..i_counter-1{
            for ix in x+1..i_counter{
                if array[x][y] > array[ix][y] {
                    i_right +=1
                }else {
                    i_right +=1;
                    break;
                }
            }
            for ix in 1..x+1{
                if array[x][y] > array[x-ix][y]{
                    i_left +=1;
                }else {
                    i_left +=1;
                    break;
                }
            }
            for iy in 1..y+1{
                if array[x][y] > array[x][y-iy] {
                    i_top+=1;
                }else {
                    i_top+=1;
                    break;
                }
            }
            for iy in y+1..i_counter{
                if array[x][y] > array[x][iy] {
                    i_bottom+=1;
                }else {
                    i_bottom+=1;
                    break;
                }
            }
            if i_score < (i_right * i_left * i_top * i_bottom) {
                i_score = i_right * i_left * i_top * i_bottom
            }
            i_right = 0;
            i_left = 0;
            i_top = 0;
            i_bottom = 0
        }
    }
    //for y in 0..i_counter{
    //    for x in 0..i_counter{
    //        print!("{:?}",array[x][y]);
    //    }
    //    println!("");
    //}
    //println!("");
    println!("{i_score}");
}
