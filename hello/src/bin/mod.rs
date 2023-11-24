use std::path::PathBuf;

use prost::{
    length_delimiter_len
};

fn main() {
    let path = PathBuf::from("/path/to/xixi/");
    let file_name = path.file_name().unwrap();
    println!("{}", std::mem::size_of::<u8>() + length_delimiter_len(std::u32::MAX as usize) * 2);
    println!("{}", std::mem::size_of::<u32>());
}