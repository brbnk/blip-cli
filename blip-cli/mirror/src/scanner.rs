use std::{fs, io::Result};

use domain::constants::{DATA_FOLDER};
use ui::{printer, types::Color};

pub fn list_identifiers() -> Result<()> {
    let dirs = fs::read_dir(format!("./{}", DATA_FOLDER))?;

    for d in dirs {
        let dir = d?;
        let file_type = dir.file_type()?;
        
        if file_type.is_dir() {
            let file_name = dir.file_name();
            let tenant = file_name.to_string_lossy().to_string();

            printer::print("\n|- ", Color::White);
            printer::println(&tenant, Color::Blue);

            let tenant_dirs = fs::read_dir(&format!("./{}/{}", DATA_FOLDER, &tenant))?;

            for td in tenant_dirs {
                let tdir = td?;
                let t_file_type = tdir.file_type()?;
                if t_file_type.is_dir() {
                    let dir_name = &format!("|--- {}", tdir.file_name().to_string_lossy());
                    printer::println(dir_name, Color::White);
                }
            }
        }
    }

    Ok(())
}
