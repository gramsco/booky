use std::env;

pub fn read_args() -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let joined = args[1..].join(" ");
        println!(" Looking for : {} ", joined);
        Ok(joined)
    } else {
        panic!("No arguments were provided.")
    }
}
