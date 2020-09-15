use std::{env, fs, io, path::Path};

/*
    * Day 3: make sure we can get the count of files
    make sure we can travel our dir
*/

pub fn run() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("{}", current_dir.file_name().unwrap().to_str().unwrap());
    walk(&current_dir, &mut String::from(""))?;

    Ok(())
}
fn walk(dir: &Path, space: &mut String) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| {
            let entry = e.unwrap();
            match entry.file_name().to_str() {
                Some(name) => {
                    if name.starts_with(".") || name == "target" {
                        return None
                    }
                    Some(entry)
                }

                None => None
            }
        })
        .collect();
    
        for (index, entry) in entries.iter().enumerate() {
            let file_name = entry.file_name();
            if index == entries.len() - 1 {
                println!("{}└── {}", space, file_name.to_str().unwrap());
            } else {
                println!("{}├── {}", space, file_name.to_str().unwrap());
            }

            if entry.file_type().unwrap().is_dir() {
                space.push_str("    ");
                walk(&entry.path(), space)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_current_directory() {}
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/* Rules for imports:
 * for function, import parent module
 * for import structs do
*/
