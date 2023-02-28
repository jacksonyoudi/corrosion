extern crate core;

mod lib;
mod lib_test;

use std::{env, fs};
use std::error::Error;
use std::process;
use lib::Config;
use lib::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}