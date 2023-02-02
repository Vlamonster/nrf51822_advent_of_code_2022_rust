use core::slice;

pub struct BitArray2D<'a> {
    width: usize,
    data: &'a mut [u8],
}

impl BitArray2D<'_> {
    pub fn new(start: *mut u8, offset: usize, width: usize, height: usize) -> Self {
        let data =
            unsafe { slice::from_raw_parts_mut(start.add(offset), (width * height - 1) / 8 + 1) };
        data.fill(0);

        BitArray2D { width, data }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let bit_index = y * self.width + x;
        let byte_index = bit_index / 8;
        let inner_index = bit_index % 8;

        self.data[byte_index] |= 1 << inner_index;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let bit_index = y * self.width + x;
        let byte_index = bit_index / 8;
        let inner_index = bit_index % 8;

        self.data[byte_index] & 1 << inner_index != 0
    }

    pub fn count_ones(&self) -> u32 {
        self.data.iter().map(|&byte| byte.count_ones()).sum::<u32>()
    }
}
