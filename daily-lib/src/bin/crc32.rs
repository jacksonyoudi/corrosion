use crc32fast::Hasher;

fn main() {
    let mut hasher = Hasher::new();
    hasher.update(b"foo bar baz");
    let checksum = hasher.finalize();
    println!("{}", checksum);
}