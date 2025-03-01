use std::fs::File;
use std::path::PathBuf;
use std::{fs, io, io::Read, io::Write};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

pub fn extract(zip_path: PathBuf, to_directory: Option<PathBuf>) -> zip::result::ZipResult<()> {
    let extract_dir = to_directory.unwrap_or(PathBuf::from("."));

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(&extract_dir)?;

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
            println!("{} file extracted", outpath.display());
        }
    }
    println!("Extracted files to {} success", extract_dir.display());
    Ok(())
}

pub fn archive(zip_path: PathBuf, files_to_compress: Vec<PathBuf>) -> zip::result::ZipResult<()> {
    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let options: FileOptions<'_, ()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut directories: Vec<PathBuf> = Vec::new();

    for file_path in &files_to_compress {
        if file_path.is_file() {
            let file = File::open(file_path)?;
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            zip.start_file(file_name, options)?;
            let mut buffer = Vec::new();
            io::copy(&mut file.take(u64::MAX), &mut buffer)?;

            zip.write_all(&buffer)?;
        } else if file_path.is_dir() {
            directories.push(file_path.clone());
        }
        while !directories.is_empty() {
            match fs::read_dir(directories.first().unwrap()) {
                Ok(entries) => {
                    for entry in entries {
                        let entry = entry.unwrap();
                        if entry.path().is_dir() {
                            directories.push(entry.path())
                        } else if entry.path().is_file() {
                            let entry = entry.path();
                            let file = File::open(&entry)?;
                            let file_location = entry
                                .strip_prefix(file_path.parent().unwrap())
                                .unwrap()
                                .to_str()
                                .unwrap();
                            dbg!(&file_location);

                            zip.start_file(file_location, options)?;
                            let mut buffer = Vec::new();
                            io::copy(&mut file.take(u64::MAX), &mut buffer)?;

                            zip.write_all(&buffer)?;
                        }
                        println!("{:?}", entry.path());
                    }
                }
                Err(e) => {
                    eprintln!("Error getting directory : {:?}", e)
                }
            }
            directories.remove(0);
        }
    }

    zip.finish()?;

    println!("success");

    Ok(())
}
