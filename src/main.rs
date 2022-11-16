use colored::*;
use std::env;

const INDENTATION_SPACES: u32 = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut depth: u32 = 2;

    if args.len() == 2 {
        if args[1] == "help" {
            println!("{} {}", "Usage:".green(), "command [depth]".yellow());
            println!("{} {}", "Example:".green(), "command 2 outputs the tree with a depth of 2.".yellow());
            return
        } else {
            depth = args[1].parse::<u32>().expect("Depth must be a number.");
        }
    }

    print_directory("./", 0, depth);
}


fn print_directory(path: &str, indentation: u32, depth: u32) {
    let paths;

    match std::fs::read_dir(path) {
        Ok(p) => paths = p,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    for path in paths.into_iter() {
        let file_name;
        let file_path;

        match path {
            Ok(p) => {
                file_name = p.file_name();
                file_path = p.path();
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        for i in 0..indentation {
            if i % INDENTATION_SPACES == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }

        let fname: String;

        match file_name.to_str() {
                Some(p) => {
                    fname = p.to_string();
                }
                None => {
                    println!("Error: file name is not valid UTF-8");
                    continue;
                }
            }

        if file_path.is_dir() {
            let mut dir_name = String::from("˅ ");
            dir_name.push_str(&fname);

            if (indentation / 4) % 3 == 0 {
                println!("{}", dir_name);
            } else if (indentation / 4) % 3 == 1 {
                println!("{}", dir_name);
            } else if (indentation / 4) % 3 == 2 {
                println!("{}", dir_name);
            }

            match file_path.to_str() {
                Some(p) => {
                    if (indentation / INDENTATION_SPACES) < depth || depth == 0 {
                        print_directory(p, indentation + INDENTATION_SPACES, depth);
                    }
                }
                None => {
                    println!("Error: file path is not valid UTF-8");
                    continue;
                }
            }
        } else {
            println!("{}", fname);
        }
    }
}
