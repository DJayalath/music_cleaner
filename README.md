# music_cleaner
A music directory clean up utility.

## What does it do?
- Scans given folder for files and subfolders
- Recursively copies music files (extensions .flac, .mp3, .webm) from subfolders to root directory
- Deletes all subfolders

## Why?
Many music downloads come in folders with artist name and album subfolders. These folders are usually useless since the music file 
itself will contain artist and album metadata read by music players. This program helps to organise your tracks by moving music files out of subfolders and into the root folder so all your tracks are in one place.

## Usage
For building: cargo run <path to directory>