use std::fs;

fn main() {

    // Scan files and folders in directory
    let (files, folders) = match scan_path() {

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
}

fn scan_path() -> Result<(Vec<fs::DirEntry>, Vec<fs::DirEntry>), std::io::Error> {

    let paths = fs::read_dir("C:/Users/dhjay/Music/Music")?;

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