# music_cleaner
A music directory clean up utility.

## Usage
```
usage: music_cleaner.exe directory [options]
  options:
      -e, --extract   Moves files from subfolder to root and deletes subfolders
      -r, --rename    Renames files in root folder using format Title - Artist
      -er, --both     Moves and renames files
```

## What does it do?
### Extraction
- Scans given folder for files and subfolders
- Recursively copies music files (extensions .flac, .mp3, .webm) from subfolders to root folder
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