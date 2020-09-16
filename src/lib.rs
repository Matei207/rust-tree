use std::{env, fs, io, path::Path};

use fs::DirEntry;


/*
    * Day 3: make sure we can get the count of files
    make sure we can travel our dir
*/
struct Count {
    file_number: u32,
    dir_number: u32,
}

impl Count {
    fn new() -> Self {
        Count {
            file_number: 0,
            dir_number: 0,
        }
    }

    fn add_file(&mut self) {
        self.file_number += 1;
    }

    fn add_dir(&mut self) {
        self.dir_number += 1;
    }

    fn display(&self) {
        println!("\n{} directories, {} files in total", self.dir_number, self.file_number);
    }
}

pub fn run() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("{}", current_dir.file_name().unwrap().to_str().unwrap());
    let mut file_counter = Count::new();
    walk(&current_dir, &String::from(""), &mut file_counter)?;

    file_counter.display();
    Ok(())
}

//TODO: get rid of depth and add a count
fn walk(dir: &Path, sym_format: &String, file_counter: &mut Count) -> io::Result<()> {
    if dir.is_dir() {
        let entries: Vec<_> = fs::read_dir(dir)?
            .filter_map(|e| {
                let entry = e.unwrap();
                match entry.file_name().to_str() {
                    Some(name) => {
                        if name.starts_with(".") || name == "target" {
                            return None;
                        }
                        Some(entry)
                    }

                    None => None,
                }
            })
            .collect();

        //println!("{:?}", entries);
        for (index, entry) in entries.iter().enumerate() {
            if entry.file_type().unwrap().is_dir() {
                file_counter.add_dir();
            } else {
                file_counter.add_file();
            }

            
            let is_last = if index == entries.len() - 1 {
                true
            } else {
                false
            };
            
            display_child(&entry, sym_format, is_last);
            
            //TODO: instead of taking it case by case just change space to be prefix and then based on conditions change the prefix
            if entry.file_type().unwrap().is_dir() {
                let space: String = if is_last {
                    String::from("    ")
                } else {
                    String::from("|    ")
                };
                
                let new_sym_format = format!("{}{}", sym_format, space);
                walk(&entry.path(), &new_sym_format, file_counter)?;
            }
        }
    }

    Ok(())
}

fn display_child(entry: &DirEntry, prefix: &String, is_last: bool) {
    let file_name = entry.file_name();
    if is_last {
        println!(
            "{}└── {}",
            prefix,
            file_name.to_str().unwrap(),
            
        );
    } else {
        println!(
            "{}├── {}",
            prefix,
            file_name.to_str().unwrap(),
        );
    }
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
