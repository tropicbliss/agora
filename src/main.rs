use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = agora::utils::get_config(args.config)?;
    agora::find(config);
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Path to seed config
    config: PathBuf,
}
