use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Default, Debug)]
struct MyConfiguration {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool,
}


fn main() {
    let mut conf = MyConfiguration::default();

    // do something with conf here
    conf.check = true;

    println!("conf = {:#?}", conf)
}


