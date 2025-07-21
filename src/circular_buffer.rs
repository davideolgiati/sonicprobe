pub struct CircularBuffer<T: Clone + Default> {
        buffer: Vec<T>,
        size: usize,
        slots_used: usize
}

impl<T: Clone + Default> CircularBuffer<T> {
        pub fn new(size: usize, default_value: T) -> CircularBuffer<T> {
                CircularBuffer { 
                        buffer: vec![default_value; size],
                        size,
                        slots_used: 0
                }
        }

        pub fn push(&mut self, value: T) {
                if self.slots_used < self.size {
                        self.slots_used += 1
                }

                self.buffer.rotate_left(1);
                self.buffer[self.size - 1] = value;
        }

        pub fn collect(&self) -> &Vec<T> {
                &self.buffer
        }

        pub fn len(&self) -> usize {
                self.slots_used
        }
}