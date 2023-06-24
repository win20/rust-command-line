use std::{env, fs, io::Result, process};

pub fn ls(args: Vec<String>) {
    let files_in_current_dir: Vec<String>;

    if args.len() > 0 && args[0] == "-a" {
        files_in_current_dir = match get_current_dir_files(false) {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };
    } else {
        files_in_current_dir = match get_current_dir_files(true) {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        };
    }

    for file in files_in_current_dir {
        println!("{}", file);
    }
}

pub fn get_current_dir_files(list_all_files: bool) -> Result<Vec<String>> {
    let current_dir = env::current_dir()?;
    let files_in_curent_dir = fs::read_dir(&current_dir)?;

    let mut files: Vec<String> = Vec::new();
    for file in files_in_curent_dir {
        if list_all_files {
            if !file
                .as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .starts_with(".")
            {
                files.push(file.unwrap().file_name().into_string().unwrap());
            }
        } else {
            files.push(file.unwrap().file_name().into_string().unwrap());
        }
    }

    Ok(files)
}
