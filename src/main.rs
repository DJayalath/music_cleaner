use std::fs;

fn main() {
    read_path();
}

fn read_path() {
    let paths = fs::read_dir("C:/Users/dhjay/Music/Music").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}