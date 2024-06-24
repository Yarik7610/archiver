use std::{env, process};

fn main() {
    let filename = archiver::get_filename(env::args()).unwrap_or_else(|msg| {
        eprintln!("{msg}");
        process::exit(1);
    });
    archiver::run(&filename).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
