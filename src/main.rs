use mia::{Config, Instance};
use std::process;

fn main() {
    let config = Config::new("package.json", "yarn").unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
    let instance = Instance::new(config).unwrap_or_else(|err| {
        eprintln!("There was a problem parsing the json: {}", err);
        process::exit(1);
    });
    let mut process = instance.run("start").unwrap_or_else(|err| {
        eprintln!("There was a problem starting the process: {}", err);
        process::exit(1);
    });
    if let Err(err) = process.wait() {
        eprintln!("There was a problem waiting: {}", err);
        process::exit(1);
    }
}
