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
    let mut b_right = true;
    let mut b_left = true;
    let mut b_top = true;
    let mut b_bottom = true;
    for y in 1..i_counter-1{
        for x in 1..i_counter-1{
            //print!("{:?}",array[x][y]);
            //if array[x][y] > array[x][ix] {
                for ix in x+1..i_counter{
                    if array[x][y] < array[ix][y] || array[x][y] == array[ix][y] {
                        b_right = false;
                        //println!("{:?} < {:?} -->R {:?}",array[x][y],array[ix][y],b_right)
                    }else {
                        //println!("{:?} > {:?} -->R {:?}",array[x][y],array[ix][y],b_right)
                    }
                }
                for ix in 1..x+1{
                    if array[x][y] < array[x-ix][y] || array[x][y] == array[x-ix][y] {
                        b_left = false;
                        //println!("{:?} < {:?} -->L {:?}",array[x][y],array[x-ix][y],b_left)
                    }else {
                        //println!("{:?} > {:?} -->L {:?}",array[x][y],array[x-ix][y],b_left)
                    }
                }
                //println!("Y {y}");
                for iy in 1..y+1{
                    if array[x][y] < array[x][y-iy] || array[x][y] == array[x][y-iy] {
                        b_top = false;
                        //println!("{:?} < {:?} -->T {:?}",array[x][y],array[x][y-iy],b_top)
                    }else {
                        //println!("{:?} > {:?} -->T {:?}",array[x][y],array[x][y-iy],b_top)
                    }
                }
                //println!("");
                for iy in y+1..i_counter{
                    if array[x][y] < array[x][iy] || array[x][y] == array[x][iy] {
                        b_bottom = false;
                        //println!("{:?} < {:?} -->L {:?}",array[x][y],array[x-ix][y],b_left)
                    }else {
                        //println!("{:?} > {:?} -->L {:?}",array[x][y],array[x-ix][y],b_left)
                    }
                }
                //for ix in x+1..i_counter{
                //    if array[x][y] < array[ix][y] || array[x][y] == array[ix][y] {
                //        b_right = false;
                //    }
                //}
                //println!("{b_right} - {b_left} - {b_top} - {b_bottom}");
            if b_right || b_left || b_top || b_bottom{
                i_score += 1;
            }
            b_right = true;
            b_left = true;
            b_bottom = true;
            b_top = true;
            //println!("");
        }
    }
    i_score += (4*i_counter)-4;
    //for y in 0..i_counter{
    //    for x in 0..i_counter{
    //        print!("{:?}",array[x][y]);
    //    }
    //    println!("");
    //}
    println!("{i_score}");
}
