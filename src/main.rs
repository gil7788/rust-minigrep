use std::env;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

   let result = minigrep::run(config);
    if let Err(e) = result {
         eprintln!("Application error: {}", e);
         std::process::exit(1);
    }
}
