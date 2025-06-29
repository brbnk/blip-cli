use std::{fs::{self, File}, io::{Result, Write}};

pub fn handle_creation(folder_name: &str, file_name: &str, file_content: &str) -> Result<()> {
    let path = format!("./data/{}", folder_name.trim());
    fs::create_dir_all(&path).expect("created dir");

    let filename = format!("{}/{}", path, file_name);
    let mut file = File::create(filename)?;

    file.write_all(file_content.as_bytes())
        .expect("file created");

    Ok(())
}