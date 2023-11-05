use regex::Regex;
use std::fs;
use std::env;
use std::io::{self, BufRead};

#[allow(unused_assignments)]
fn interpret_program(prog: Vec<&str>, debug: bool) -> String {
    let mut buffer = String::new();

    // Set buffer to start
    let cur_state: Vec<&str> = prog[0].split('|').collect();

    // If starting buffer is empty, read from stdin
    if cur_state.len() == 1 {
        println!("Enter starting buffer:");
        let mut temp = String::new();
        io::stdin().lock().read_line(&mut temp).expect("Failed to read line");
        buffer = temp.trim().to_string();
    } else {
        buffer = cur_state[1].to_string();
    }

    // Set statement to after start
    let mut state_num = 2;
    while state_num <= prog.len() {
        // Get the current state
        let cur_state: Vec<&str> = prog[state_num - 1].split('|').collect();

        // Check for Halt
        if cur_state[1].to_lowercase() == "halt" {
            break;
        } else {
            let needle = Regex::new(&regex::escape(cur_state[1])).unwrap();
            if needle.is_match(&buffer) {
                if debug {
                    print!(
                        "{}: {} ---> {} == ",
                        state_num,
                        cur_state[1],
                        cur_state[2]
                    );
                }
                // Replace needle with replacement string
                buffer = needle.replace_all(&buffer, cur_state[2]).to_string();
                state_num = cur_state[3].parse().unwrap();
            } else {
                if debug {
                    print!("{}: {} not found. == ", state_num, cur_state[1]);
                }
                state_num = cur_state[4].parse().unwrap();
            }
        }

        // If debug is turned on, print buffer every time
        if debug {
            println!("{}", buffer);
        }
    }

    buffer
}

fn run_for_single_file() {
    let mut file_path = String::new();

    println!("Enter the file name (including the .txt extension):");
    io::stdin().read_line(&mut file_path)
        .expect("Failed to read line");

    let file_path = file_path.trim();

    let file_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("File not found or cannot be opened.");
            return;
        }
    };

    let prog: Vec<&str> = file_content.lines().collect();
    let debug = false; // Set debug mode if needed

    let result = interpret_program(prog, debug);
    println!("{}", result);
}

fn run_for_all_files() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let mut files: Vec<_> = fs::read_dir(current_dir)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|dir_entry| dir_entry.path())
        .filter(|path| {
            if let Some(extension) = path.extension() {
                if let Some(ext) = extension.to_str() {
                    return ext.to_lowercase() == "txt";
                }
            }
            false
        })
        .collect();

    files.sort();

    for file in files {
        let file_content = fs::read_to_string(&file).expect("Failed to read file");
        let prog: Vec<&str> = file_content.lines().collect();
        let debug = false; // Set debug mode if needed

        let result = interpret_program(prog, debug);
        println!("Result for {}: {}", file.display(), result);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "all" {
        run_for_all_files();
    } else {
        run_for_single_file();
    }
}
