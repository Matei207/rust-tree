use std::{env, fs, io, path::Path, path::PathBuf};

use fs::DirEntry;


/*
    * Day 3: make sure we can get the count of files
    make sure we can travel our dir
    * Day 4: add args, fix todos
*/

pub fn run(args: env::Args) -> io::Result<()> {
    let cfg = Config::new(args);
    
    let path = if cfg.path == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&cfg.path)
    }; 

    println!("{}", cfg.path);
    let mut file_counter = Count::new();
    walk(&path, &String::from(""), 0, &mut file_counter, &cfg)?;

    file_counter.display();
    Ok(())
}

//TODO: get rid of depth and add a count
fn walk(dir: &Path, sym_format: &str, depth: u32, file_counter: &mut Count, cfg: &Config) -> io::Result<()> {
    if depth > 0 && depth == cfg.depth_lvl {
        return Ok(())
    }

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
                walk(&entry.path(), &new_sym_format, depth + 1,  file_counter, cfg)?;
            }
        }
    }

    Ok(())
}

fn display_child(entry: &DirEntry, prefix: &str, is_last: bool) {
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

/*TODO:
    * path (. // actual path)
    * -l, -a
    * --help
    * -L -- depth
*/
#[derive(Debug)]
struct Config {
    path: String,
    dir_only: bool,
    depth_lvl: u32,
    show_all: bool,
}


impl Config {
    fn new(mut args: env::Args) -> Self {
        args.next();
        let path = match args.next()  {
            Some(path_as_string) => path_as_string,
            None => String::from("."),
        };

        let show_all: bool = match args.next() {
            Some(arg) => {
                if arg == "-a" {
                    true
                } else {
                    false
                }

            }

            None => false
        };

        let dir_only = match args.next() {
            Some(arg) => {
                if arg == "-d" {
                    true
                } else {
                    false
                }

            }

            _ => false
        };

        let depth_lvl: u32 = match args.next()  {
            Some(arg) => {
                if arg != "-L" {
                    0
                } else {
                    let depth = args.next().unwrap().parse::<u32>().unwrap();
                    depth
                }
            }

            _ => 0
        };

        return Config{path, show_all, dir_only, depth_lvl};

    }
}

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
