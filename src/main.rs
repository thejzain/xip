use std::path::PathBuf;

use clap::Parser;

mod modules;
use modules::read_extension;
use modules::read_extension::FileType;

#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
    paths: Option<Vec<PathBuf>>,

    #[arg(short = 'd', long = "dir", value_name = "DIR")]
    directory: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    // let extension = read_extension::read_file_extension(&args.path).unwrap();
    // match extension {
    //     FileType::Zip => modules::zip::extract(args.path, args.directory)
    //         .unwrap_or_else(|error| println!("{:?}", error)),
    //     FileType::TarGz => modules::tar::gz::extract(args.path, args.directory)
    //         .unwrap_or_else(|error| panic!("{error}")),
    // }
    // modules::tar::gz::archive(args.path, args.paths).unwrap_or_else(|error| panic!("{error}"));
    if let Some(directory) = &args.directory {
        let extension = read_extension::read_file_extension(&args.path).unwrap();
        match extension {
            FileType::Zip => modules::zip::extract(args.path, args.directory)
                .unwrap_or_else(|error| println!("{:?}", error)),
            FileType::TarGz => modules::tar::gz::extract(args.path, args.directory)
                .unwrap_or_else(|error| panic!("{error}")),
        }
    } else if let Some(paths) = args.paths {
        modules::tar::gz::archive(args.path, paths).unwrap_or_else(|error| panic!("{error}"));
    } else {
        let extension = read_extension::read_file_extension(&args.path).unwrap();
        match extension {
            FileType::Zip => modules::zip::extract(args.path, args.directory)
                .unwrap_or_else(|error| println!("{:?}", error)),
            FileType::TarGz => modules::tar::gz::extract(args.path, args.directory)
                .unwrap_or_else(|error| panic!("{error}")),
        }
    }
}
