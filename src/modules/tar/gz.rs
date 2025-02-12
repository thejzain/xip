use clap::error::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{fs::File, path::PathBuf};
use tar::Archive;

pub fn extract(tar_path: PathBuf, directory: Option<PathBuf>) -> Result<(), std::io::Error> {
    let extract_dir = directory.unwrap_or(PathBuf::from("."));
    let file = File::open(tar_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    archive.unpack(&extract_dir)?;
    println!("Extracted to {} success", extract_dir.display());

    Ok(())
}

pub fn archive(tar_create: PathBuf, files: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let tar_gz = File::create(tar_create)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    for file in files {
        if file.is_dir() {
            tar.append_dir_all(file.file_name().unwrap(), &file)?;
        } else {
            tar.append_file(file.file_name().unwrap(), &mut File::open(&file)?)?;
        }
    }
    tar.finish()?;
    println!("success!");
    Ok(())
}
