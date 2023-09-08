
#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    println!("Arguement 1 - {}",args.pattern);
    println!("Arguement 2 - {}",args.path.display());
}





