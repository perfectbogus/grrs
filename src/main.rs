use std::fs::read_to_string;
use std::path::PathBuf;
use std::{thread, time};
use std::env::args;
use std::io::Write;
use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf
}
fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    warn!("ops, nothing implemented");

    let pb = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(time::Duration::from_millis(50));
        pb.inc(1);
    }
    pb.finish_with_message("Done");

    let args = Cli::parse();

    let _ = read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;




    Ok(())
}




