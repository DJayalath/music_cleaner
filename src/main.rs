use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "music_cleaner",
    about = "A utility to help organize music files."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    origin: PathBuf,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Clone, StructOpt)]
enum Cmd {
    /// Moves files from subfolder to root and deletes subfolders.
    #[structopt(name = "extract")]
    Extract {
        #[structopt(raw(use_delimiter = "true"), parse(from_os_str))]
        extensions: Vec<OsString>,
    },

    /// Renames files in root folder using format Title - Artist.
    #[structopt(name = "rename")]
    Rename,

    // Both move and rename files.
    #[structopt(name = "both")]
    Both {
        #[structopt(raw(use_delimiter = "true"), parse(from_os_str))]
        extensions: Vec<OsString>,
    },
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = real_main(&opt) {
        eprintln!("Err: {}", err);
        std::process::exit(1);
    }
    println!("Complete");
    pause().unwrap();
}

fn real_main(opt: &Opt) -> Result<(), Box<dyn Error>> {
    match &opt.cmd {
        Cmd::Extract { extensions } => extract(&opt.origin, extensions),
        Cmd::Rename => rename(&opt.origin),
        Cmd::Both { extensions } => {
            extract(&opt.origin, extensions)?;
            rename(&opt.origin)?;
            Ok(())
        }
    }
}

fn extract(origin: &Path, file_extensions: &[OsString]) -> Result<(), Box<dyn Error>> {
    println!("Found:");
    // Scan files and folders in directory
    let (files, folders) = scan_path(origin)?;
    println!("  => {} files", files.len());
    println!("  => {} folders", folders.len());

    // Recursively scan folders for flacs
    let mut deep_files: Vec<fs::DirEntry> = Vec::new();
    recursive_find(&folders, &mut deep_files)?;
    println!("  => {} files nested in folders", deep_files.len());

    println!("\nExtracting...");
    extract_music(&deep_files, &file_extensions, origin)?;

    // Remove folders
    println!("Removing folders...");
    for folder in folders {
        if !folder.file_name().to_string_lossy().starts_with('.') {
            fs::remove_dir_all(folder.path())?;
        }
    }
    Ok(())
}

fn rename(origin: &Path) -> Result<(), Box<dyn Error>> {
    println!("Updating directories...");
    // Rescan for renaming
    let (files, _folders) = scan_path(origin)?;

    println!("Renaming files...");
    // Rename!
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

fn rename_file_with_metadata(file: &fs::DirEntry, origin: &Path) -> Result<(), Box<dyn Error>> {
    let tag = metaflac::Tag::read_from_path(&file.path())?;
    let windows_special_chars = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'][..];
    let artist = tag
        .get_vorbis("artist")
        .ok_or("failed to get artist name")?[0]
        .replace(windows_special_chars, "");
    let title = tag.get_vorbis("title").ok_or("failed to get song title")?[0]
        .replace(windows_special_chars, "");
    let file_path = file.path();
    let ext = file_path
        .extension()
        .ok_or("failed to get file extension")?;

    // Format final name and directory
    let mut destination = origin.join(PathBuf::from(format!("{} - {}", title, artist)));
    destination.set_extension(ext);
    fs::rename(file_path, destination)?;
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
