use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf
}
fn main() -> Result<()> {
    let args = Cli::parse();

    let result = read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;

    for line in result.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())

}
