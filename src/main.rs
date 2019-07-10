use std::fs;
use std::path::Path;
use std::ffi::OsStr;

fn main() {

    // Scan files and folders in directory
    let (files, folders) = match scan_path("C:/Users/dhjay/Music/Music") {

        Ok((fi, fo)) => {
        
            println!("\nFound {} files:\n", fi.len());
            for f in &fi {
                println!("{:?}", f.file_name());
            }

            println!("\nFound {} folders:\n", fo.len());
            for f in &fo {
                println!("{:?}", f.file_name());
            }

            (fi, fo)
        },

        Err(e) => panic!("ERROR: {}", e)

    };

    // Recursively scan folders for flacs
    recursive_find(folders);
}

fn scan_path(directory: &str) -> Result<(Vec<fs::DirEntry>, Vec<fs::DirEntry>), std::io::Error> {

    let paths = fs::read_dir(directory)?;

    let mut files = Vec::new();
    let mut folders = Vec::new();

    for p in paths {
        
        let path = p?;
        let metadata = path.metadata()?;

        if metadata.is_file() {
            files.push(path);
        } else {
            folders.push(path);
        }
    }

    Ok((files, folders))
}

fn recursive_find(folders: Vec<fs::DirEntry>) {

    println!();
    if folders.len() == 0 {
        return
    }
    for folder in folders {
        let (deep_files, deep_folders) = scan_path(folder.path().to_str().unwrap()).unwrap();
        for file in deep_files {
            println!("{:?}", file.file_name());
        }
        recursive_find(deep_folders);
    }

}