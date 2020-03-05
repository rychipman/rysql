use std::{process, io::{self, Write}};

fn main() {
	repl()
}

fn repl() {
	loop {
		print_prompt();
		match read_input().as_str() {
			".exit" => process::exit(0),
			input => {
				println!("unknown command '{}'", input);
			},
		};
	}
}

fn read_input() -> String {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {},
		Err(e) => panic!(e),
	};
	input.trim().to_string()
}

fn print_prompt() {
	print!("> ");
	io::stdout().flush().expect("failed to flush stdout");
}
