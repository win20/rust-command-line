use std::{env, fs, io::Result, process};

pub fn ls() {
    let files_in_current_dir = match get_current_dir_files() {
        Ok(files) => files,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    for file in files_in_current_dir {
        println!("{}", file);
    }
}

pub fn get_current_dir_files() -> Result<Vec<String>> {
    let current_dir = env::current_dir()?;
    let files_in_curent_dir = fs::read_dir(&current_dir)?;

    let mut files: Vec<String> = Vec::new();
    for file in files_in_curent_dir {
        files.push(file.unwrap().file_name().into_string().unwrap());
    }

    Ok(files)
}
