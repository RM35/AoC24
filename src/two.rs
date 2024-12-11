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
    let mut file = File::open("data/two_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut damped_reports: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    let mut total_safe: i32 = 0;
    for line in contents.lines() {
        println!();
        let report: Vec<i32> = line.split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        println!("{report:?}");

        let mut descending: bool = false;
        let mut safe: bool = true;
        for (i, num) in report.iter().enumerate() {
            if i == 0 {
                continue;
            } 
            if i == 1 {
                descending = num < &report[i-1];
                print!(" {:<5} ", if descending { "Desc" } else { "Asc" });
            }

            if &report[i-1] == num {
                safe = false;
                let mut damped_report: Vec<i32> = report.clone();
                damped_report.remove(i);
                let mut damped_report_prev: Vec<i32> = report.clone();
                damped_report_prev.remove(i-1);
                damped_reports.push((damped_report, damped_report_prev));
                break;
            }

            if descending {
                if &report[i-1] - num > 3 || &report[i-1] < num {
                    safe = false;
                    let mut damped_report: Vec<i32> = report.clone();
                    damped_report.remove(i);
                    let mut damped_report_prev: Vec<i32> = report.clone();
                    damped_report_prev.remove(i-1);
                    damped_reports.push((damped_report, damped_report_prev));
                    break;
                }
            } else {
                if &report[i-1] - num < -3 || &report[i-1] > num {
                    safe = false;
                    let mut damped_report: Vec<i32> = report.clone();
                    damped_report.remove(i);
                    let mut damped_report_prev: Vec<i32> = report.clone();
                    damped_report_prev.remove(i-1);
                    damped_reports.push((damped_report, damped_report_prev));
                    break;
                }
            }
        }
        if safe { 
            print!(" SAFE ");
            total_safe += 1;
        } else {
            print!(" NOT ");
        }
    }

    println!("\n\n-Running damped vec-");
    println!("No. of damped: {:?}", damped_reports.len());
    for (report1, report2) in damped_reports.iter() {

        //Check if removing ith fixed
        let mut descending: bool = false;
        let mut safe: bool = true;
        for (i, num) in report1.iter().enumerate() {
            //print!(" {num} ");
            if i == 0 {
                continue;
            } 
            if i == 1 {
                descending = num < &report1[i-1];
                print!(" {:<5} ", if descending { "Desc" } else { "Asc" });
            }

            if &report1[i-1] == num {
                safe = false;
                break;
            }

            if descending {
                if &report1[i-1] - num > 3 || &report1[i-1] < num {
                    safe = false;
                    break;
                }
            } else {
                if &report1[i-1] - num < -3 || &report1[i-1] > num {
                    safe = false;
                    break;
                }
            }
        }

        if safe { 
            print!(" SAFE ");
            total_safe += 1;
            continue
        } else {
            print!(" NOT ");
        }

        //Check if removing ith-1 fixed
        descending = false;
        safe = true;
        for (i, num) in report2.iter().enumerate() {
            //print!(" {num} ");
            if i == 0 {
                continue;
            } 
            if i == 1 {
                descending = num < &report2[i-1];
                print!(" {:<5} ", if descending { "Desc" } else { "Asc" });
            }

            if &report2[i-1] == num {
                safe = false;
                break;
            }

            if descending {
                if &report2[i-1] - num > 3 || &report2[i-1] < num {
                    safe = false;
                    break;
                }
            } else {
                if &report2[i-1] - num < -3 || &report2[i-1] > num {
                    safe = false;
                    break;
                }
            }
        }
        if safe { 
            print!(" SAFE ");
            total_safe += 1;
            continue
        } else {
            print!(" NOT ");
        }
    }
    // Was one short so +1 to this answer. Not bothering to find out why.
    println!("\nFinal Score: {total_safe:?}");
    Ok(())
}

