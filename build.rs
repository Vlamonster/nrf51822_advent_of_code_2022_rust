use lzss::{IOSimpleReader, IOSimpleWriter, Lzss};
use std::fs::File;
use std::io::BufReader;

fn main() {
    for i in 1..=25 {
        let input = File::open(format!("inputs/d{i:02}.txt")).unwrap();
        let mut output = File::create(format!("inputs/d{i:02}c.txt")).unwrap();
        Lzss::<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>::compress(
            IOSimpleReader::new(&mut BufReader::new(input)),
            IOSimpleWriter::new(&mut output),
        )
        .unwrap();
    }
}
