use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)   // Create the file if it doesn't exist
        .append(true)   // Append to the file if it does exist
        .open(path)
        .expect("Failed to open or create the file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to the file");
}
