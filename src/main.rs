use clap::Parser;

mod modules;
use modules::read_extension;

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let extension = read_extension::read_file_extension(&args.path).unwrap();
}
