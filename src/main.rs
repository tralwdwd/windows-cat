use std::fs;
use std::env;

fn main(){
    let file = env::args().nth(1).unwrap_or_else(|| {
    eprint!("Error: No file specified");
    std::process::exit(1); 
    });
    match fs::read_to_string(file) {
        Ok(contents) => {
            // Successfully read the file
            print!("{}", contents);
        }
        Err(err) => {
            // Handle the error
            eprint!("Error reading file: {}", err);
            std::process::exit(1);
        }
    }
}