use crate::dsp::DSPChain;

impl<T: Clone> DSPChain<T> {
    pub fn new(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn window<R, F>(self, size: usize, func: F) -> DSPChain<R>
    where
        F: Fn(&[T], usize, usize) -> Vec<R>,
        R: Clone,
    {
        let new_data = (0..(self.data.len() - size))
            .flat_map(|index| func(&self.data, index, index + size))
            .collect();
        DSPChain { data: new_data }
    }

    pub fn collect(self) -> Vec<T> {
        self.data
    }
}
