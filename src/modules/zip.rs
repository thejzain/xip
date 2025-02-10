use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub fn decompress(zip_path: PathBuf) -> zip::result::ZipResult<()> {
    let extract_dir = Path::new(".");

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(extract_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let filename = file.name();
        let outpath = extract_dir.join(filename);

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent_dir) = outpath.parent() {
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    println!("Extracted files to {} success", extract_dir.display());
    Ok(())
}
