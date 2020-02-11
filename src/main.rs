use std::env;
use std::fs;
use std::io;
use std::collections::VecDeque;
use std::io::BufRead;

fn tail_file(file_path: &String, number_of_lines: u32) -> Result<VecDeque<String>, io::Error> {
    let mut lines: VecDeque<String> = VecDeque::<String>::new();
    let file = fs::File::open(file_path)?;
    let buf_reader = io::BufReader::new(file);
    for line in buf_reader.lines() {
        lines.push_back(line?);
        if lines.len() as u32 > number_of_lines {
            lines.pop_front();
        }
    }

    return Ok(lines);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        println!("Incorrect format. Try tail <file_path> <number_of_lines>");
        return ();
    }
    let file_path: &String = &args[1];
    let number_of_lines: u32 = args[2].parse::<u32>().unwrap();
    let meta = std::fs::metadata(file_path);
    match meta {
        Ok(file_info) => {
            if file_info.is_file() {
                match tail_file(file_path, number_of_lines) {
                    Ok(mut lines) => {
                        let mut line: Option<String> = lines.pop_front();
                        while line != None {
                            println!("{}", line.unwrap());
                            line = lines.pop_front();
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            } else {
                println!("File does not exist.");
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
