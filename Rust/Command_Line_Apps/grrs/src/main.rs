use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String, 
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }
}
