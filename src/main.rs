use std::env;
use std::process::exit;
use std::fs;

fn main() {
   let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("Usage: json_parser <file_path>");
        exit(1);
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if is_valid_json(&content) {
                println!("Valid JSON");
                exit(0);
            }else{
                eprintln!("Invalid JSON");
                exit(1);
            }
        }

        Err(_) => {
            eprintln!("Unable to read file");
            exit(1);
        }
    }

}

fn is_valid_json(content: &str)-> bool{
    content.trim() == "{}"
}