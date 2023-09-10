
#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use indicatif::ProgressStyle;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
println!("Arguement 1 - {}",args.pattern);
println!("Arguement 2 - {}",args.path.display());

    let path = "src/main.rs";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    
        let pb = indicatif::ProgressBar::new(100);
        for i in 0..100 {
            //do_hard_work();
            pb.println(format!("[+] finished #{}", i));
            pb.inc(1);
        }
        pb.finish_with_message("done");
    
        env_logger::init();
        info!("starting up");
        warn!("oops, nothing implemented!");

    Ok(())
}




