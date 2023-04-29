use std::ffi::OsStr;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    file: String,

    #[arg(long,short)]
    limit: Option<u64>,
}

fn main() -> Result<()>{
    println!("Compute oracle");
    let mut oracle_path = std::env::current_exe()?;
    let mut file_name = oracle_path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    while oracle_path.pop() {
        if file_name == "target" {
            break;
        }
        file_name = oracle_path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    }
    println!("Project dir: {:?}", oracle_path);
    let args = Args::parse();
    leia_compute_oracle::compute_oracle(oracle_path, &args.file, &args.limit)?;
    Ok(())
}
