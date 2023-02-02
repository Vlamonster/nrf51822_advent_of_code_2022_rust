use core::slice;

pub struct RingBuffer<'a, T> {
    read_ptr: usize,
    write_ptr: usize,
    capacity: usize,
    data: &'a mut [T],
}

impl<T: Copy> RingBuffer<'_, T> {
    pub fn new(start: *mut T, capacity: usize) -> Self {
        assert!(
            capacity.is_power_of_two(),
            "Capacity must be a power of two."
        );

        let data = unsafe { slice::from_raw_parts_mut(start, capacity) };

        RingBuffer {
            read_ptr: 0,
            write_ptr: 0,
            capacity,
            data,
        }
    }

    pub fn queue(&mut self, value: T) {
        self.data[self.write_ptr] = value;
        self.write_ptr += 1;
        self.write_ptr &= !self.capacity;
    }

    pub fn dequeue(&mut self) -> T {
        let value = self.data[self.read_ptr];
        self.read_ptr += 1;
        self.read_ptr &= !self.capacity;
        value
    }
}
