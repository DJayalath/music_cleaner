extern crate metaflac;

use metaflac::Tag;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::error::Error;
use std::env;
use std::io::{stdin, stdout, Read, Write};
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No args specified!");
        println!("usage: music_cleaner.exe directory [options]");
        println!("  options:");
        println!("      -r, --rename    Renames files using metadata in format Title - Artist");
        println!("      -s, --skip      Skips directory extraction and renames files in root folder");
        process::exit(1);
    }

    let origin = Path::new(&args[1]);
    if !origin.exists() {
        println!("Given path doesn't exist!");
        process::exit(1);
    }

    // TODO: Allow specifying custom file extensions

    // Set flle extensions to keep
    let file_extensions = vec![OsStr::new("flac"), OsStr::new("mp3"), OsStr::new("webm")];

    println!("Found:");

    // Scan files and folders in directory
    let (_files, folders) = match scan_path(origin) {

        Ok((fi, fo)) => {
        
            println!("  => {} files", fi.len());
            println!("  => {} folders", fo.len());

            (fi, fo)
        },

        Err(e) => panic!("ERROR: {}", e)

    };

    // Recursively scan folders for flacs (UNSAFE)
    let mut deep_files: Vec<fs::DirEntry> = Vec::new();
    recursive_find(&folders, &mut deep_files);
    println!("  => {} files nested in folders", deep_files.len());

    println!("\nExtracting...");
    if let Err(e) = extract_music(&deep_files, &file_extensions, origin) {
        panic!("ERROR: {}", e);
    }

    // Remove folders
    println!("Removing folders...");
    for folder in folders {
        if !folder.file_name().into_string().unwrap().starts_with(".") {
            fs::remove_dir_all(folder.path()).unwrap();
        }
    }

    println!("Updating directories...");
    // Rescan for renaming
    let (files, _folders) = match scan_path(origin) {

        Ok((fi, fo)) => (fi, fo),
        Err(e) => panic!("ERROR: {}", e)
    };

    println!("Renaming files...");
    // Rename!
    for file in files {
        rename_file_with_metadata(file, origin);
    }

    println!("Complete!");

    pause();
}

fn scan_path(directory: &Path) -> Result<(Vec<fs::DirEntry>, Vec<fs::DirEntry>), std::io::Error> {

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

fn recursive_find(folders: &Vec<fs::DirEntry>, found_files: &mut Vec<fs::DirEntry>) {

    if folders.len() == 0 {
        return
    }
    for folder in folders {
        
        let (deep_files, deep_folders) = scan_path(folder.path().as_path()).unwrap();
        for file in deep_files {
            found_files.push(file);
        }
        recursive_find(&deep_folders, found_files);
    }

}

fn extract_music(files: &Vec<fs::DirEntry>, file_extensions: &Vec<&OsStr>, origin: &Path) -> Result<(), Box<dyn Error>> {

    for file in files {
        let path = file.path();
        let path = Path::new(&path);
        match path.extension() {
            Some(val) => {
                if file_extensions.contains(&val) {
                    let destination = origin.join(file.file_name());
                    fs::copy(file.path(), destination)?;
                }
            },
            None => ()
        };
    }

    Ok(())
}

fn rename_file_with_metadata(file: fs::DirEntry, origin: &Path) {

    match Tag::read_from_path(&file.path()) {

        Ok(tag) => {

            let artist = match tag.get_vorbis("artist") {
                Some(a) => a[0].clone(),
                None => String::from("Unknown"),
            };
            let title = match tag.get_vorbis("title") {
                Some(t) => t[0].clone(),
                None => String::from("Unknown"),
            };
            let path = file.path();
            let ext = match path.extension() {
                Some(e) => e,
                None => OsStr::new("err"),
            };

            // Remove any Windows special characters
            let artist = artist.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");
            let title = title.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");

            // Format final name and directory
            let destination = origin.join(format!("{} - {}.{}", title, artist, ext.to_str().unwrap()));

            fs::rename(file.path(), destination).expect(&format!("Failed to rename {}", file.file_name().into_string().unwrap()));
        }

        Err(e) => println!("Skipped {} because of error {}", file.file_name().into_string().unwrap(), e)
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}