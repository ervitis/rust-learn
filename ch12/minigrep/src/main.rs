use std::{env, process, fs};
use std::error::Error;
use std::intrinsics::rustc_peek;

#[derive(Debug)]
struct Args {
    query: String,
    filename: String,
}

impl Args {
    fn parse(str_args: &Vec<String>) -> Result<Args, &str> {
        if str_args.len() < 3 {
            return Err("not enough arguments")
        }

        Ok(Args {
            filename: String::from(&str_args[2]),
            query: String::from(&str_args[1]),
        })
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;

    for line in search(&args.query, &contents) {
        println!("{}", line)
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

fn main() {
    let args = Args::parse(&env::args().collect()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", args);

    if let Err(e) = run(args) {
        println!("app error: {}", e);
        process::exit(1);
    }
}
