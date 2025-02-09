use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

pub fn read_file_extension(filepath: &PathBuf) -> io::Result<[u8; 4]> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut header = [0u8; 4];

    reader.read_exact(&mut header)?;
    extension_list(header);

    Ok(header)
}

fn extension_list(header: [u8; 4]) {
    match header {
        [80, 75, 3, 4] => {
            println!("zip")
        }
        [31, 139, 8, 0] => {
            println!("tar.gz")
        }
        _ => {
            println!("Unkown file format")
        }
    }
}
