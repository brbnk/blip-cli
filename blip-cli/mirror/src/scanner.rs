use std::{fs, io::Result};

pub fn list_identifiers() -> Result<()> {
    let dirs = fs::read_dir("./data")?;

    for d in dirs {
        let dir = d?;
        let file_type = dir.file_type()?;
        if file_type.is_dir() {
            println!("{}", dir.file_name().to_string_lossy());
        }
    };
    
    Ok(())
}