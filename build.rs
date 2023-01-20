use std::fs::File;
use lzss::{IOSimpleReader, IOSimpleWriter, Lzss, SliceReader, SliceWriter};

fn main() {
    let mut input = File::open("inputs/d01.txt").unwrap();
    let mut output = File::create("inputs/d01c.txt").unwrap();
    type MyLzss = Lzss<10, 4, 0x20, {1 << 10}, {2 << 10}>;
    let result = MyLzss::compress(IOSimpleReader::new(&mut input), IOSimpleWriter::new(&mut output));
    println!("{:?}", result);
    println!("hey");
}
