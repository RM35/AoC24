use std::fs::File;
use std::io::prelude::*;

pub fn problem_part_one() -> std::io::Result<()> { 
    let mut file = File::open("data/two_input")?;

    let mut contents = String::new();

    let _ = match file.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(e)   => {
            println!("Error {} reading in file", e);
            0
        }
    };

    
    let mut total_safe: i32 = 0;
    for line in contents.lines() {
        println!();
        let report: Vec<i32> = line.split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut descending: bool = false;
        let mut safe: bool = true;
        for (i, num) in report.iter().enumerate() {
            print!(" {num} ");
            if i == 0 {
                continue;
            } 
            if i == 1 {
                descending = num < &report[i-1];
                print!(" |{num:?} < {:?} ", &report[i-1]);
                print!(" Is desc: {descending:<5}| ");
            }

            if &report[i-1] == num {
                safe = false;
                break;
            }

            if descending {
                if &report[i-1] - num > 3 || &report[i-1] < num {
                    safe = false;
                    break;
                }
            } else {
                if &report[i-1] - num < -3 || &report[i-1] > num {
                    safe = false;
                    break;
                }
            }
        }

        if safe { 
            print!("- SAFE");
            total_safe += 1;
        } else {
            print!("- NOT SAFE");
        }
    }

    println!("\nFinal Score: {total_safe:?}");
    Ok(())
}


pub fn problem_part_two() -> std::io::Result<()> { 
    Ok(())
}



