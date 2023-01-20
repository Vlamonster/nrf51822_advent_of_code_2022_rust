use lzss::{IOSimpleReader, IOSimpleWriter, Lzss};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

struct ConcatFiles<'a> {
    files: Vec<File>,
    idx: usize,
    delimiter: &'a str,
}

impl<'a> ConcatFiles<'a> {
    pub fn new(paths: &[&'a str], delimiter: &'a str) -> io::Result<Self> {
        Ok(Self {
            files: paths
                .iter()
                .map(|i| File::open(i))
                .collect::<Result<_, _>>()?,
            idx: 0,
            delimiter,
        })
    }
}

impl Read for ConcatFiles<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < self.delimiter.as_bytes().len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "buffer smaller than delimiter length",
            ));
        }

        if self.idx >= self.files.len() {
            return Ok(0);
        }

        let curr: &mut File = &mut self.files[self.idx];
        match curr.read(buf) {
            // if we didn't read anything, go to the next file
            Ok(0) => {
                // go to the next file
                self.idx += 1;
                // if we are at the end, return 0
                if self.idx >= self.files.len() {
                    return Ok(0);
                }

                // write the delimiter if we're not at the end
                // but if the length of the delimiter is 0 (ie there is none)
                // just recursively read again, the next file so we fill the buffer with something.
                if self.delimiter.len() == 0 {
                    self.read(buf)
                } else {
                    for (&src, dst) in self.delimiter.as_bytes().iter().zip(buf) {
                        *dst = src;
                    }
                    Ok(self.delimiter.as_bytes().len())
                }
            }
            i => i,
        }
    }
}

fn main() {
    let input = ConcatFiles::new(
        &[
            "inputs/d01.txt",
            "inputs/d02.txt",
            "inputs/d03.txt",
            "inputs/d04.txt",
            "inputs/d05.txt",
            "inputs/d06.txt",
            "inputs/d07.txt",
            "inputs/d08.txt",
            "inputs/d09.txt",
            "inputs/d10.txt",
            "inputs/d11.txt",
            "inputs/d12.txt",
            "inputs/d13.txt",
            "inputs/d14.txt",
            "inputs/d15.txt",
            "inputs/d16.txt",
            "inputs/d17.txt",
            "inputs/d18.txt",
            "inputs/d19.txt",
            "inputs/d20.txt",
            "inputs/d21.txt",
            "inputs/d22.txt",
            "inputs/d23.txt",
            "inputs/d24.txt",
            "inputs/d25.txt",
        ],
        "!!!\n",
    )
    .unwrap();

    // let mut input = File::open("inputs/d01.txt").unwrap();
    let mut output = File::create("inputs/compressed_input.txt").unwrap();
    Lzss::<10, 4, 0x20, { 1 << 10 }, { 2 << 10 }>::compress(
        IOSimpleReader::new(&mut BufReader::new(input)),
        IOSimpleWriter::new(&mut output),
    )
    .unwrap();
}
