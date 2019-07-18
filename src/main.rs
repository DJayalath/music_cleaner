use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::fmt;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for CustomError {}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "music_cleaner",
    about = "A utility to help organize music files."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    directory: PathBuf,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Clone, StructOpt)]
enum Cmd {

    // Moves files from subfolder to root and deletes subfolders
    #[structopt(
        name = "extract",
        about = "Move files from subfolder to root and delete subfolders"
    )]
    Extract {
        #[structopt(raw(use_delimiter = "true"), parse(from_os_str))]
        extensions: Vec<OsString>,
    },

    // Renames files in format Title - Artist
    #[structopt(
        name = "rename",
        about = "Rename files in format Title - Artist"
    )]
    Rename,

    // Move and rename files
    #[structopt(
        name = "both",
        about = "Extract AND rename files"
    )]
    Both {
        #[structopt(raw(use_delimiter = "true"), parse(from_os_str))]
        extensions: Vec<OsString>,
    },
}

fn main() {

    let opt = Opt::from_args();

    if let Err(err) = execute(&opt) {
        eprint!("Err: {}", err);
        std::process::exit(1);
    }

    println!("Complete");
    pause().unwrap();
}

fn execute(opt: &Opt) -> Result<(), Box<dyn Error>> {
    match &opt.cmd {
        Cmd::Extract { extensions } => extract(&opt.directory, extensions),
        Cmd::Rename => rename(&opt.directory),
        Cmd::Both { extensions } => {
            extract(&opt.directory, extensions)?;
            rename(&opt.directory)?;
            Ok(())
        }
    }
}

fn extract(origin: &Path, file_extensions: &[OsString]) -> Result<(), Box <dyn Error>> {
    
    // Check extensions exist
    if file_extensions.is_empty() {
        return Result::Err(Box::new(CustomError("No file extensions given!".into())));
    }

    // Scan files and folders in directory
    println!("\nScanning...");
    let (files, folders) = scan_path(origin)?;
    println!("Found:");
    println!("  => {} files", files.len());
    println!("  => {} folders", folders.len());

    // Recursively scan folders for music
    let mut deep_files: Vec<fs::DirEntry> = Vec::new();
    recursive_find(&folders, &mut deep_files)?;
    println!("  => {} files nested in folders", deep_files.len());

    // Extract found files
    println!("\nExtracting...");
    extract_music(&deep_files, &file_extensions, origin)?;

    // Remove folders
    println!("\nRemoving folders...");
    for folder in folders {
        // Ignore hidden directories
        if !folder.file_name().to_string_lossy().starts_with('.') {
            fs::remove_dir_all(folder.path())?;
        }
    }

    Ok(())
}

fn rename(origin: &Path) -> Result<(), Box<dyn Error>> {

    // Scan for renaming
    println!("\nScanning...");
    let (files, _folders) = scan_path(origin)?;

    // Rename files
    println!("\nRenaming...");
    for file in files {
        if let Err(e) = rename_file_with_metadata(&file, origin) {
            println!(
                "Skipped {} because {}",
                &file.file_name().to_string_lossy(),
                e
            );
        }
    }

    Ok(())
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
        } else if metadata.is_dir() {
            folders.push(path);
        } else {
            unreachable!();
        }
    }

    Ok((files, folders))
}

fn recursive_find(
    folders: &[fs::DirEntry],
    found_files: &mut Vec<fs::DirEntry>,
) -> Result<(), Box<dyn Error>> {
    if folders.is_empty() {
        return Ok(());
    }
    for folder in folders {
        let (deep_files, deep_folders) = scan_path(folder.path().as_path())?;
        found_files.extend(deep_files);
        recursive_find(&deep_folders, found_files)?;
    }

    Ok(())
}

fn extract_music(
    files: &[fs::DirEntry],
    file_extensions: &[OsString],
    origin: &Path,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        let path = file.path();
        if let Some(ext) = path.extension() {
            if file_extensions.iter().any(|x| x == ext) {
                let destination = origin.join(file.file_name());
                fs::copy(file.path(), destination)?;
            }
        };
    }

    Ok(())
}

fn rename_file_with_metadata(file: &fs::DirEntry, origin: &Path) -> Result<(), String> {
    match metaflac::Tag::read_from_path(&file.path()) {
        Ok(tag) => {
            let artist = match tag.get_vorbis("artist") {
                Some(a) => a[0].clone(),
                None => return Err(String::from("failed to get artist name")),
            };
            let title = match tag.get_vorbis("title") {
                Some(t) => t[0].clone(),
                None => return Err(String::from("failed to get song title")),
            };
            let path = file.path();
            let ext = match path.extension() {
                Some(e) => e,
                None => return Err(String::from("failed to get file extension")),
            };

            // Remove any Windows special characters
            let artist = artist.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");
            let title = title.replace(&['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..], "");

            // Format final name and directory
            let destination =
                origin.join(format!("{} - {}.{}", title, artist, ext.to_str().unwrap()));

            if let Err(e) = fs::rename(file.path(), destination) {
                return Err(format!("{}", e));
            }
        }

        Err(e) => return Err(format!("{}", e)),
    }

    Ok(())
}

fn pause() -> Result<(), std::io::Error> {
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    stdout.write_all(b"Press Enter to continue...")?;
    stdout.flush()?;
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}

// fn print_usage() {
//     println!("usage: music_cleaner.exe directory [options] [file extensions]");
//     println!("  options:");
//     println!("      -e, --extract   Moves files from subfolder to root and deletes subfolders");
//     println!("      -r, --rename    Renames files in root folder using format Title - Artist");
//     println!("      -er, --both     Moves and renames files");
//     println!("  file extensions (comma separated list):");
//     println!("      e.g. flac,mp3   Selects files to extract from folders and not delete");
// }