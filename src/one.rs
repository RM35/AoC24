use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
    for (&a, &b) in first_vec.iter().zip(second_vec.iter()) {
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

    //Count vector 2 occurences
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for &item in second_vec.iter() {
        counts.entry(item)
              .and_modify(|x| *x += 1)
              .or_insert(1);
    }
    //println!("{:?}", counts);

    let mut total: i32 = 0;

    for value in first_vec.iter() {
        if let Some(counted) = counts.get(&value) {
            total += counted * value
        }
    }

    println!("{total}");

    Ok(())
}



