// Parse a fasta file, and then return a tsv that goes like "fastaheader\tlength" 
// use clap to parse command line arguments

use clap::Parser; // argparse for rust
use std::fs::File; // Open files
use std::io::{self, BufRead, BufReader}; // I think this is open a file line by line?

// define a struct to handle your command line arguments
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Parse a FASTA file, and output a summary of headers and lengths" )]

struct Args { // list your command line arguments here
    // path to the file
    #[arg(short, long)]
    file: String, 
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let filepath = &args.file;

    // println!("The path to your file is {filepath}"); // works
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut curr_len = 0;
    let mut i = 0; // keep track of line number 

    println!("Contig/Scaffold\tLength");

    for line in reader.lines() {
        let line = line?; // strings, phew
        let line = String::from(line);
        // println!("{line}"); // works!
        // use an iterator to get the first character of a string

        let first_char = line.chars().next().unwrap_or(' '); // Default is an empty character

        if first_char == '>' {
            if i != 0 {
                print!("{curr_len}");
                println!();
            }
            curr_len = 0;
            let header_name = &line[1..];
            print!("{header_name}\t"); // works!
        }
        else {
            curr_len += line.chars().count();
        }
        i += 1;

    }
    print!("{curr_len}");

    Ok(())
    
}
