use clap::error::Result;
use flate2::read::GzDecoder;
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
