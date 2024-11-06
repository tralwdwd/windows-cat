use std::fs;
use std::path::PathBuf;
use clap::{Arg, Command};

fn main(){
    let matches=Command::new("windows-cat")
    .about("windows-cat: Unix cat command but for windows.")
    .bin_name("cat")
    .arg(
        Arg::new("file")
        .help("File to extract contents from.")
        .required(false)
        .index(1)
    )
    .get_matches();
    let file = PathBuf::from(matches.get_one::<String>("file")
    .unwrap_or_else(|| {
    eprint!("Error: No file specified");
    std::process::exit(1); 
    }));
    if !file.is_dir(){
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
}else{
    eprint!("Error: Path is not a file");
    std::process::exit(1);
}
}