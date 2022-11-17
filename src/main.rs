use colored::*;
use std::env;

mod utility;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut depth: u32 = 2;

    if args.len() == 2 {
        if args[1] == "help" {
            println!("{} {}", "Usage:".cyan(), "ft [depth]. Depth is optional.");
            println!("{} {}", "Example:".cyan(), "\"ft 2\" outputs the tree with a depth of 2.");
            return
        } else {
            depth = args[1].parse::<u32>().expect("Depth must be a number.");
        }
    }

    println!("{}", ".".blue());
    print_directory("./", "", 0, depth);
}


fn print_directory(path: &str, start: &str, indentation: u32, depth: u32) {
    let paths: Vec<_>;

    match std::fs::read_dir(path) {
        Ok(p) => paths = p.collect(),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    let num_paths = paths.len();
    let mut index = 0;

    for path in paths {

        let file_name;
        let file_path;

        match path {
            Ok(p) => {
                file_name = p.file_name();
                file_path = p.path();
                if utility::is_hidden(file_name.clone()) {
                    index+=1;
                    continue;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        let prepend: ColoredString;
        print!("{}", start);

        index += 1;
        //println!("Index: {}, Num Paths: {}", index, num_paths);
        if index == num_paths {
            match indentation % 3 {
                0=>{
                    print!("{}", "└── ".blue());
                    prepend = "    ".blue();
                },
                1=>{
                    print!("{}", "└── ".cyan());
                    prepend = "    ".cyan();
                },
                2=>{
                    print!("{}", "└── ".green());
                    prepend = "    ".green();
                },
                _=>panic!("Indentation is not a multiple of 3."),
            }

        } else {
            match indentation % 3 {
                0=>{
                    print!("{}", "├── ".blue());
                    prepend = "│   ".blue();
                },
                1=>{
                    print!("{}", "├── ".cyan());
                    prepend = "│   ".cyan();
                },
                2=>{
                    print!("{}", "├── ".green());
                    prepend = "│   ".green();
                },
                _=>panic!("Indentation is not a multiple of 3."),
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
            let mut dir_name = String::from("./");
            dir_name.push_str(&fname);

            match (indentation + 1) % 3 {
                0=>{
                    println!("{}", dir_name.bold().blue());
                },
                1=>{
                    println!("{}", dir_name.bold().cyan());
                },
                2=>{
                    println!("{}", dir_name.bold().green());
                },
                _=>panic!("Indentation is not a multiple of 3."),
            }

            match file_path.to_str() {
                Some(p) => {
                    if indentation < depth {
                        print_directory(p, format!("{}{}", start, prepend).as_str(), indentation + 1, depth);
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


