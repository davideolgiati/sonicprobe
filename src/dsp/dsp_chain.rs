use crate::dsp::DSPChain;

impl<T: Clone> DSPChain<T> {
    pub fn new(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn window<R, F>(self, size: usize, func: F) -> DSPChain<R>
    where
        F: Fn(&[T]) -> Vec<R>,
        R: Clone,
    {
        let new_data= self.data
                .windows(size)
                .flat_map(func)
                .collect();
        DSPChain { data: new_data }
    }

    pub fn collect(self) -> Vec<T> {
        self.data
    }
}
