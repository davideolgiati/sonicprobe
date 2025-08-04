use std::sync::Arc;

use crate::dsp::DSPChain;

impl<T: Clone> DSPChain<T> {
    pub const fn new(data: Arc<[T]>) -> Self {
        Self {
            data,
        }
    }

    pub fn window<R, F>(self, size: usize, func: F) -> DSPChain<R>
    where
        F: Fn(&Arc<[T]>, usize, usize) -> R,
        R: Clone,
    {
        let new_data = (0..(self.data.len() - size))
            .map(|index| func(&self.data, index, index + size))
            .collect();
        DSPChain { data: new_data }
    }

    pub fn flat_window<R, I, F>(self, size: usize, func: F) -> DSPChain<R>
    where
        F: Fn(Arc<[T]>, usize) -> I,
        I: Iterator<Item = R>,
    {
        let new_data = (0..(self.data.len() - size))
            .flat_map(|index| func(Arc::clone(&self.data), index))
            .collect();
        DSPChain { data: new_data }
    }

    pub fn collect(self) -> Arc<[T]> {
        self.data
    }
}
