use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

use flate2::write::ZlibEncoder;
use flate2::Compression;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use sha2::{Digest, Sha256};

use walkdir::WalkDir; // TODO: Remove this dep later

#[derive(Serialize, Deserialize)]
struct FileData {
    path: String,
    content: String,
}

fn main() -> Result<()> {
    let mut files = Vec::new();

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let content = read_file_content(path)?;
            files.push(FileData {
                path: path.to_string_lossy().into_owned(),
                content,
            });
        }
    }

    // Step 4: Serialize the vector
    let serialized = serde_json::to_string(&files)?;

    // Step 5: Compress the serialized data using zlib
    let compressed = compress_data(&serialized)?;

    // Write the compressed data to a file
    let mut file = File::create("compressed_files.zlib")?;
    file.write_all(&compressed)?;

    Ok(())
}

fn read_file_content<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn compress_data(data: &str) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes())?;
    Ok(encoder.finish()?)
}
