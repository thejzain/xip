use clap::Parser;

mod modules;
use modules::read_extension::{self, FileType};

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let extension = read_extension::read_file_extension(&args.path).unwrap();
    match extension {
        FileType::Zip => {
            modules::zip::decompress(args.path).unwrap_or_else(|error| println!("{:?}", error))
        }
        FileType::TarGz => (),
    }
}
