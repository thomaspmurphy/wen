use std::fs;
use std::env;
mod tokeniser;
use tokeniser::tokenise as t;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: wen <command> <file_path>");
        return;
    }
    let command = &args[1];
    let file_path = &args[2];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    match command.as_str() {
        "t" => {
            let tokens = t(contents);
            println!("Tokens: {:?}", tokens);
        }
        _ => println!("Unknown command"),
    }
}
