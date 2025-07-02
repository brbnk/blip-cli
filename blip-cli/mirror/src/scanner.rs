use std::{fs, io::Result};

pub fn list_identifiers() -> Result<()> {
    let dirs = fs::read_dir("./data")?;

    for d in dirs {
        let dir = d?;
        let file_type = dir.file_type()?;
        if file_type.is_dir() {
            let file_name = dir.file_name();
            let tenant = file_name.to_string_lossy().to_string();
            println!("\nContrato: {}", &tenant);
            let tenant_dirs = fs::read_dir(&format!("./data/{}", &tenant))?;

            for td in tenant_dirs {
                let tdir = td?;
                let t_file_type = tdir.file_type()?;
                if t_file_type.is_dir() {
                    println!("|-- {}", tdir.file_name().to_string_lossy());
                }
            }
        }
    }

    Ok(())
}
