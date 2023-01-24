use lz4_flex::compress_prepend_size;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    for i in 1..=25 {
        let mut buffer = vec![];
        File::open(format!("inputs/d{i:02}.txt"))
            .unwrap()
            .read_to_end(&mut buffer)
            .unwrap();

        let mut output = File::create(format!("inputs/d{i:02}c.txt")).unwrap();
        output.write_all(&compress_prepend_size(&buffer)).unwrap();
    }
}
