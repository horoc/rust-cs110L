use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines = read_file_lines(filename).expect(format!("open file {} error", filename).as_str());
    print!("lines: {}, ", lines.len());
    let mut word_count = 0;
    let mut char_count = 0;
    for line in lines {
        char_count += line.len() + 1;
        let sp = line.split(" ");
        for s in sp {
            word_count += 1;
        }
    }

    print!("word count: {}, char count: {}\n", word_count, char_count);
}

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut v = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        v.push(line_str);
    }
    Ok(v)
}
