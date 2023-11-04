use regex::Regex;
use std::io::{self, BufRead};

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

fn main() {
    let mut file_path = String::new();

    println!("Enter the file name (including the .txt extension):");
    io::stdin().lock().read_line(&mut file_path).expect("Failed to read line");

    let file_path = file_path.trim();

    let file_content = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("File not found or cannot be opened.");
            return;
        }
    };

    let prog: Vec<&str> = file_content.lines().collect();

    // Set debug mode if flag is provided
    let mut debug = false;
    if file_path.contains("-D") {
        debug = true;
    }

    let result = interpret_program(prog, debug);
    println!("{}", result);
}
