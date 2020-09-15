use std::{fs, env};
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    walk(&current_dir)?;
    

    Ok(())
}

fn walk(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Err(err) => println!("error {}", err),
            Ok(paths) => for path in paths {
                let entry = path.unwrap();
                match entry.file_name().to_str() {
                    Some(name) => println!("-- {}", name),
                    _ => {}
                }
                
                let path = entry.path();
                if path.is_dir() {
                    if !entry.path().starts_with(".") {
                       walk(&path)?;
                    }
                }

            }
        }
    }
  
    Ok(())
}
/*
    * '?' is shorthand for an entire match statement, if it's an error
    it returns from the function, if it is an ok it can unwraps
*/