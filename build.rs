use lzss::{IOSimpleReader, IOSimpleWriter, Lzss};
use std::fs::File;
use std::io::BufReader;

fn main() {
    for i in 1..=25 {
        let input = File::open(format!("inputs/d{:02}.txt", i)).unwrap();
        let mut output = File::create(format!("inputs/d{:02}c.txt", i)).unwrap();
        Lzss::<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>::compress(
            IOSimpleReader::new(&mut BufReader::new(input)),
            IOSimpleWriter::new(&mut output),
        )
        .unwrap();
    }
}
