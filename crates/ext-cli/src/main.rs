use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ext-cli",
    about = "Compile a local Rust crate into a WebAssembly module (.wasm) for embedding in a Rust host application at runtime."
)]
struct Args {
    /// The path to the extension directory
    #[arg(short, long("source"))]
    source_dir: PathBuf,
    /// The output directory to place the packaged extension.
    #[arg(short, long("output"))]
    output_dir: PathBuf,
    /// The path to a directory where build dependencies are downloaded
    #[arg(long("scratch"))]
    scratch_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("source_dir: {:?}", args.source_dir);
    Ok(())
}
