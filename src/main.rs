mod cc;
use cc::*;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        cc::log_error("No argument / file given!");     
        std::process::exit(1);
    }

    let file_path = &args[1];
    if !cc::check_extension(file_path) {
        cc::log_error("Unknown file extension! Expected '.cc' extension!");
        std::process::exit(1);
    }

    let contents: String = match std::fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(_) => String::new() 
    };

    cc::compile(&contents);
}
