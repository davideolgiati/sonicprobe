pub struct CircularBuffer<T: Clone + Default> {
        buffer: Vec<T>,
        start: usize,
        end: usize,
        size: usize,
        full: bool
}

impl<T: Clone + Default> CircularBuffer<T> {
        pub fn new(size: usize, default_value: T) -> CircularBuffer<T> {
                CircularBuffer { 
                        buffer: vec![default_value; size], 
                        start: 0,
                        end: 0,
                        size,
                        full: false
                }
        }

        pub fn push(&mut self, value: T) {
                self.end = (self.end + 1) % self.size;
                self.buffer[self.end] = value;

                if self.end == self.start {
                        self.start = (self.start + 1) % self.size
                }

                if !self.full && self.end == self.size - 1{
                        self.full = true
                }
        }

        pub fn at(&self, pos: usize) -> &T {
                if pos >= self.size {
                        panic!("position {} is out of range for buffer of size {}", pos, self.size)
                }

                let actual_index = (self.start + pos) % self.size;

                &self.buffer[actual_index]
        }

        pub fn len(&self) -> usize {
                if self.full {
                        self.size
                } else {
                        self.end + 1
                }
        }
}