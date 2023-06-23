use std::{
    env,
    fs::{self, ReadDir},
    io::{Error, Result},
    path::PathBuf,
};

pub fn ls() {
    // let test = env::current_dir().unwrap();
    // let files_in_current_dir = fs::read_dir(&test).unwrap();
    //
    // for file in files_in_current_dir {
    //     let full_path = file.unwrap().file_name();
    //     println!("{}", full_path.into_string().unwrap());
    // }
    //

    let files_in_current_dir = get_current_dir_files();
    for file in files_in_current_dir {
        println!("{}", file);
    }
}

pub fn get_current_dir_files() -> Vec<String> {
    let mut current_dir: PathBuf = PathBuf::new();
    let mut files_in_curent_dir: Option<ReadDir> = None;

    match env::current_dir() {
        Ok(dir) => current_dir = dir,
        Err(e) => println!("Error: {}", e),
    }

    match fs::read_dir(&current_dir) {
        Ok(files_in_dir) => files_in_curent_dir = Some(files_in_dir),
        Err(e) => println!("Error: {}", e),
    }

    let mut files: Vec<String> = Vec::new();
    if let Some(files_in_dir) = files_in_curent_dir {
        for file in files_in_dir {
            files.push(file.unwrap().file_name().into_string().unwrap());
        }
    }

    files
}
