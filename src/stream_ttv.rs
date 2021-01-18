use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;

use std::collections::VecDeque;

/// An iterator type to generate train/test/validaton splits from a data stream
/// of unknown lenght.
pub struct TTVIterator<T, I>
where
    I: Iterator<Item = T>,
{
    iter: I,
    queue: [VecDeque<T>; 3],
    dist: WeightedIndex<f32>,
    rng: rand::rngs::ThreadRng,
}

impl<T, I: Iterator<Item = T>> TTVIterator<T, I> {
    pub(super) fn new(iter: I, splits: [f32; 3]) -> Self {
        Self {
            iter,
            queue: [VecDeque::new(), VecDeque::new(), VecDeque::new()],
            dist: WeightedIndex::new(&splits).unwrap(),
            rng: rand::thread_rng(),
        }
    }
}

impl<T, I> Iterator for TTVIterator<T, I>
where
    I: Iterator<Item = T>,
{
    type Item = [Option<T>; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = [None, None, None];
        for (i, item) in out.iter_mut().enumerate() {
            if !self.queue[i].is_empty() {
                *item = self.queue[i].pop_front();
            }
        }
        if !out.iter().any(|m| m.is_none()) {
            return Some(out);
        }
        for _ in 0..2 {
            if let Some(point) = self.iter.next() {
                let idx = self.dist.sample(&mut self.rng);
                if out[idx].is_some() {
                    self.queue[idx].push_back(point);
                } else {
                    out[idx] = Some(point);
                }
            }
        }

        if self.queue.iter().all(|v| v.is_empty()) && out.iter().all(|m| m.is_none()) {
            None
        } else {
            Some(out)
        }
    }
}

/// An iterator type to generate train/test/validaton splits from a data stream
/// of unknown lenght.
///
/// It is the same as [`PartsSplit`](./trait.PartsSplit.html) but with a fixed
/// size of 3 (it uses arrays instead of vectors).
///
/// See the [examples](https://github.com/carrascomj/rand_split/tree/trunk/examples)
/// on the repository for both iterator traits.
pub trait TTVSplit<T>: Iterator<Item = T> + Sized {
    /// `splits` contains weights that are not required to sum up to 1.
    fn split_ttv(self, splits: [f32; 3]) -> TTVIterator<T, Self>;
}

impl<'a, T, I> TTVSplit<T> for I
where
    I: Iterator<Item = T>,
{
    fn split_ttv(self, splits: [f32; 3]) -> TTVIterator<T, Self> {
        TTVIterator::new(self, splits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_split_sizes_even_stream() {
        let splits = [0.2, 0.7, 0.1];
        let cont = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (mut train, mut test, mut valid) = (Vec::new(), Vec::new(), Vec::new());
        cont.iter().split_ttv(splits).for_each(|sp| {
            train.push(sp[0]);
            test.push(sp[1]);
            valid.push(sp[2])
        });
        println!(
            "Train: {:#?}, Test: {:#?}, Validation: {:#?}",
            train, test, valid
        );
    }
}
