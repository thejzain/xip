use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    Zip,
    TarGz,
}

pub fn read_file_extension(filepath: &PathBuf) -> io::Result<FileType> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut header = [0u8; 4];

    reader.read_exact(&mut header)?;

    Ok(extension_list(header))
}

fn extension_list(header: [u8; 4]) -> FileType {
    match header {
        [80, 75, 3, 4] => FileType::Zip,
        [31, 139, 8, 0] => FileType::TarGz,
        _ => panic!("Unknown FileType"),
    }
}
