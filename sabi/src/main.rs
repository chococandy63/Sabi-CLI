
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


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}


fn main() -> Result<()> {
    let args = Cli::parse();
println!("Arguement 1 - {}",args.pattern);
println!("Arguement 2 - {}",args.path.display());

    let path = "src/main.rs";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    
        let pb = indicatif::ProgressBar::new(100);
        for i in 0..100 {
            find_matches(&content, &args.pattern, &mut std::io::stdout());
            pb.println(format!("[+] finished #{}", i));
            pb.inc(1);
        }
        pb.finish_with_message("done");
    
        // env_logger::init();
        // info!("starting up");
        // warn!("oops, nothing implemented!");

    Ok(())
}




