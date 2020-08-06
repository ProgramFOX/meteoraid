#![feature(try_trait)]

mod areas;
mod builder;
mod distribution;
mod factors;
mod field;
mod interpreter;
mod lua;
mod meteor;
mod session;
mod stars;
mod timestamp;

use clap::{App, Arg};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let matches = App::new("Meteoraid")
        .version("0.1.0")
        .about("Processes visual meteor observations")
        .arg(
            Arg::with_name("INPUT")
                .help("Specified the input code file to process")
                .required(true),
        )
        .arg(
            Arg::with_name("output-count")
                .short("c")
                .long("output-count")
                .value_name("PATH")
                .help("Path to store the CSV with the counts. (stdout if omitted)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output-distr")
                .short("d")
                .long("output-distr")
                .value_name("PATH")
                .help("Path to store the CSV with the magnitude distribution. (stdout if omitted)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Overwrite output files if they already exist."),
        )
        .get_matches();

    let force_overwrite = matches.is_present("force");
    let output_count = matches.value_of("output-count");
    let output_distr = matches.value_of("output-distr");

    let input_file = matches.value_of("INPUT").unwrap();

    let mut intrprtr = match interpreter::Interpreter::new() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not initialize Lua context. Error: {}", e);
            return;
        }
    };

    match fs::File::open(input_file) {
        Ok(file) => {
            let reader = BufReader::new(&file);
            for (num, line) in reader.lines().enumerate() {
                let line_text = match line {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("Error when reading line: {}", e);
                        return;
                    }
                };
                match intrprtr.execute_one_line(&line_text) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error while executing code, line {}:\n{}", num + 1, e);
                        return;
                    }
                };
            }
        }
        Err(e) => eprintln!("Error when reading file: {}", e),
    };

    let session = match intrprtr.get_session() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error while executing code, EOF:\n{}", e);
            return;
        }
    };

    let (count_csv, distr_csv) = match session.get_csvs() {
        Ok(csvs) => csvs,
        Err(e) => {
            eprintln!("Error while generating CSVs: {:?}", e);
            return;
        }
    };

    if let Some(output_count_path) = output_count {
        if force_overwrite || !Path::new(&output_count_path).exists() {
            match fs::write(&output_count_path, &count_csv) {
                Ok(_) => println!("Count CSV written to {}.", &output_count_path),
                Err(e) => {
                    eprintln!("Error while writing count CSV: {}", e);
                    return;
                }
            }
        } else {
            println!(
                "{} already exists and -f flag not set, outputting count CSV to stdout:",
                output_count_path
            );
            println!("--------------------------------------------");
            println!("{}", count_csv);
            println!("--------------------------------------------");
        }
    } else {
        println!("Count CSV:");
        println!("--------------------------------------------");
        println!("{}", count_csv);
        println!("--------------------------------------------");
    }

    if let Some(output_distr_path) = output_distr {
        if force_overwrite || !Path::new(&output_distr_path).exists() {
            match fs::write(&output_distr_path, &distr_csv) {
                Ok(_) => println!("Distribution CSV written to {}.", &output_distr_path),
                Err(e) => {
                    eprintln!("Error while writing distribution CSV: {}", e);
                }
            }
        } else {
            println!(
                "{} already exists and -f flag not set, outputting distribution CSV to stdout:",
                output_distr_path
            );
            println!("--------------------------------------------");
            println!("{}", distr_csv);
            println!("--------------------------------------------");
        }
    } else {
        println!("Distribution CSV:");
        println!("--------------------------------------------");
        println!("{}", distr_csv);
        println!("--------------------------------------------");
    }
}
