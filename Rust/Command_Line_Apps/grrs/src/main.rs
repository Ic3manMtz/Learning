struct Cli{
    pattern: String, 
    path: std::path::Pathbuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");

    println!("Pattern: {:?}, path: {:?}", pattern, path)
}
