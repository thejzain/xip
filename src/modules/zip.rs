use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub fn decompress(zip_path: PathBuf) -> zip::result::ZipResult<()> {
    let extract_dir = Path::new("extract_dir");

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(&extract_dir)?;

    println!("archive len = {:?}", archive.len());

    Ok(())
}
