use std::env;
use std::io::{self, Write};

fn get_input() -> String {
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

struct Line {
	key: i32,
	name: String,
	email: String
}

impl Line {
	fn new(key: i32, name: String, email: String) -> Line {
		Line {
			key: key,
			name: name,
			email: email
		}
	}
}

fn handle_input(input: &str, lines: &mut Vec<Line>) {
	let parts: Vec<&str> = input.split_whitespace().collect();
	if parts.len() != 4 {
		print!("Invalid insert command. Use: insert <key> <name> <email>");
		return;
	}
	let key: i32 = match parts[1].parse() {
		Ok(num) => num,
		Err(_) => {
			print!("Invalid key. It should be an integer");
			return;
		}
	};

	let name = parts[2].to_string();
	let email = parts[3].to_string();
	let new_line = Line::new(key, name, email);
	lines.push(new_line);
}
fn handle_select(input: &str, lines: &Vec<Line>) {
	let parts: Vec<&str> = input.split_whitespace().collect();
	if parts.len() != 2 {
		println!("Invalid select command. Use select <key>");
		return;
	}

	let key: i32 = match parts[1].parse() {
	    Ok(num) => num,
	    Err(_) => {
	    	println!("Invalid key. It should be an integer.");
	    	return;
	    }
	};

	for line in lines {
		if line.key == key {
			println!("Found: {} {} {}",line.key, line.name, line.email);
			return;
		}
	}

	println!("No entry found with key: {}", key);
}

fn process_input(input: &str, lines: &mut Vec<Line>) -> bool {
	let trimmed_input = input.trim().to_lowercase();

	if trimmed_input == "exit" {
		println!("Exiting Program!");
		return true;
	} else if trimmed_input.starts_with("insert") {
		println!("Performing insert command!");
		handle_input(input, lines);
	} else if trimmed_input.starts_with("select") {
		println!("Performing select command!");
		handle_select(input, lines);
	} else {
		println!("Unrecognied command");
	}
	false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for (i, arg) in args.iter().enumerate() {
            if i == 0 {
                continue;
            }
            println!("Argument: {}", arg);
        }
    } else {
        println!("No arguments given");
    }

    let mut lines: Vec<Line> = Vec::new();

    loop {
        print!("db > ");
        let input: String = get_input();
        if process_input(&input, &mut lines) {
        	break;
        }
    }
}