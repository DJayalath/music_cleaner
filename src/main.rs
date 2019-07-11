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

    // TODO: Allow specifying custom file extensions

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("No path/args/file extensions specified!");
        print_usage();
        process::exit(1);
    }

    let origin = Path::new(&args[1]);
    if !origin.exists() {
        println!("Given path doesn't exist!");
        print_usage();
        process::exit(1);
    }

    // Set flle extensions to keep
    let mut file_extensions = Vec::new();

    let option = &args[2];
    match &option[..] {
        "-e" | "--extract" => extract(origin, &file_extensions),
        "-r" | "--rename" => rename(origin),
        "-er" | "--both" => {extract(origin, &file_extensions); rename(origin);},
        _ => {
            println!("Invalid option!");
            print_usage();
            process::exit(1);
        }
    }

    let extensions = &args[3];
    let extensions: Vec<&str> = extensions.split(",").collect();
    for extension in extensions {
        file_extensions.push(OsStr::new(extension));
    }

    println!("Complete!");

    pause();
}

fn extract(origin: &Path, file_extensions: &Vec<&std::ffi::OsStr>) {
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

    // Recursively scan folders for flacs
    let mut deep_files: Vec<fs::DirEntry> = Vec::new();
    if let Err(e) = recursive_find(&folders, &mut deep_files) {
        println!("ERROR: {}", e);
        process::exit(1);
    }
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

}

fn rename(origin: &Path) {
    println!("Updating directories...");
    // Rescan for renaming
    let (files, _folders) = match scan_path(origin) {

        Ok((fi, fo)) => (fi, fo),
        Err(e) => panic!("ERROR: {}", e)
    };

    println!("Renaming files...");
    // Rename!
    for file in files {
        if let Err(e) = rename_file_with_metadata(&file, origin) {
            println!("Skipped {} because {}", &file.file_name().into_string().unwrap(), e);
        }
    }
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

fn recursive_find(folders: &Vec<fs::DirEntry>, found_files: &mut Vec<fs::DirEntry>) -> Result<(), Box<dyn Error>> {

    if folders.len() == 0 {
        return Ok(())
    }
    for folder in folders {
        
        let (deep_files, deep_folders) = scan_path(folder.path().as_path())?;
        for file in deep_files {
            found_files.push(file);
        }
        recursive_find(&deep_folders, found_files)?;
    }

    Ok(())
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

fn rename_file_with_metadata(file: &fs::DirEntry, origin: &Path) -> Result<(), String> {

    match Tag::read_from_path(&file.path()) {

        Ok(tag) => {

            let artist = match tag.get_vorbis("artist") {
                Some(a) => a[0].clone(),
                None => return Err(String::from(format!("failed to get artist name"))),
            };
            let title = match tag.get_vorbis("title") {
                Some(t) => t[0].clone(),
                None => return Err(String::from(format!("failed to get song title"))),
            };
            let path = file.path();
            let ext = match path.extension() {
                Some(e) => e,
                None => return Err(String::from(format!("failed to get file extension"))),
            };

            // Remove any Windows special characters
            let artist = artist.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");
            let title = title.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");

            // Format final name and directory
            let destination = origin.join(format!("{} - {}.{}", title, artist, ext.to_str().unwrap()));

            if let Err(e) = fs::rename(file.path(), destination) {
                return Err(format!("{}", e));
            }
        }

        Err(e) => return Err(format!("{}", e)),
    }

    Ok(())
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn print_usage() {
    println!("usage: music_cleaner.exe directory [options] [file extensions]");
    println!("  options:");
    println!("      -e, --extract   Moves files from subfolder to root and deletes subfolders");
    println!("      -r, --rename    Renames files in root folder using format Title - Artist");
    println!("      -er, --both     Moves and renames files");
    println!("  file extensions (comma separated list):");
    println!("      example: flac,mp3,webm");
}