# music_cleaner
A music directory clean up utility.

## Download
See releases [here](https://github.com/armytricks/music_cleaner/releases)

## Usage
- Open a powershell instance in the folder containing `music_cleaner.exe`
```
USAGE:
    music_cleaner <directory> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <directory>    

SUBCOMMANDS:
    both       Extract AND rename files
    extract    Move files from subfolder to root and delete subfolders
    help       Prints this message or the help of the given subcommand(s)
    rename     Rename files in format Title - Artist
```
Example usage:
`./music_cleaner.exe "/Documents/Music/" both flac,mp3`

## What does it do?
### Extraction
- Scans given folder for files and subfolders
- Recursively copies music files with specified extensions from subfolders to root folder
- Deletes all subfolders
### Renaming
- Collects track title and artist name from file metadata
- Renames file in format "Title - Artist"

## Why?
Many music downloads come in folders with artist name and album subfolders. These folders are usually useless since the music file 
itself will contain artist and album metadata read by music players. This program helps to organise your tracks by moving music files out of subfolders and into the root folder so all your tracks are in one place.

The renaming option exists to maintain consistency amongst track names.

## Extract example
Before:
```
Music
│   myfavouritetrack.mp3
│   absoluteclassic.flac  
│
└───ACDC
│   │   albumcover.png
│   │   data.id2
|   |   Highway to hell.flac
│   │
│   └───High Voltage
│       │   It's a long way to the top.flac
│       │   ...
│   
└───Blue Swede
    │   Hooked on a feeling.webm
    │   ...
```
After:
```
Music
│   myfavouritetrack.mp3
│   absoluteclassic.flac  
|   Highway to hell.flac
│   It's a long way to the top.flac
|   Hooked on a feeling.webm
```

## Rename example
Before:
```
07_Toto_Africa_Africa.flac
CarryOnMyWaywardSon.flac
```
After:
```
Africa - Toto.flac
Carry on Wayward Son - Kansas.flac
```

## Notes, warnings and disclaimers
- Do not put `music_cleaner.exe` in your music folder. Always use it from outside and specify a path to the music folder.
- File renaming currently only works for `.flac` files. It will ignore all others.
- Always make a backup of your directory and be VERY careful with the directory path you specify.
