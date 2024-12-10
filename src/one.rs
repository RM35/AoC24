use std::fs::File;
use std::io::prelude::*;

pub fn problem_part_one() -> std::io::Result<()> { 
    let mut file = File::open("data/one_input")?;

    let mut contents = String::new();

    let _ = match file.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(e)   => {
            println!("Error {} reading in file", e);
            0
        }
    };

    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();

    for line in contents.lines() {

        let mut split = line.split("   ");

        if let Some(first) = split.next() {
            if let Ok(first_int) = first.parse::<i32>() {
                first_vec.push(first_int);
                //println!("Adding {} to first vec", first_int);
            }
        }

        if let Some(second) = split.next() {
            if let Ok(second_int) = second.parse::<i32>() {
                second_vec.push(second_int);
                //println!("Adding {} to second vec", second_int);
            }
        }

    }

    first_vec.sort();
    second_vec.sort();

    let mut total: i32 = 0;
    for (a, b) in first_vec.iter().zip(second_vec.iter()) {
        total = total + (b - a).abs();
    } 

    println!("{}", total);

    Ok(())
}


pub fn problem_part_two() -> std::io::Result<()> { 
    let mut file = File::open("data/one_input")?;

    let mut contents = String::new();

    let _ = match file.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(e)   => {
            println!("Error {} reading in file", e);
            0
        }
    };

    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();

    for line in contents.lines() {

        let mut split = line.split("   ");

        if let Some(first) = split.next() {
            if let Ok(first_int) = first.parse::<i32>() {
                first_vec.push(first_int);
                //println!("Adding {} to first vec", first_int);
            }
        }

        if let Some(second) = split.next() {
            if let Ok(second_int) = second.parse::<i32>() {
                second_vec.push(second_int);
                //println!("Adding {} to second vec", second_int);
            }
        }

    }

    first_vec.sort();
    second_vec.sort();

    let mut total: i32 = 0;
    for (a, b) in first_vec.iter().zip(second_vec.iter()) {
        total = total + (b - a).abs();
    } 

    println!("{}", total);

    Ok(())
}



