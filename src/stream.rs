use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;

/// An iterator type to generate splits from a data stream of unknown lenght.
pub struct PartsIterator<T, I>
where
    I: Iterator<Item = T>,
{
    iter: I,
    queue: Vec<Vec<T>>,
    dist: WeightedIndex<f32>,
    rng: rand::rngs::ThreadRng,
    n: usize,
}

impl<T, I: Iterator<Item = T>> PartsIterator<T, I> {
    pub(super) fn new(iter: I, splits: &[f32]) -> Self {
        let n = splits.len();
        let mut queue = Vec::with_capacity(n);
        queue.resize_with(n, Vec::new);
        Self {
            iter,
            queue,
            dist: WeightedIndex::new(splits).unwrap(),
            rng: rand::thread_rng(),
            n,
        }
    }
}

impl<T, I> Iterator for PartsIterator<T, I>
where
    I: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    type Item = Vec<Option<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = Vec::with_capacity(self.n);
        out.resize_with(self.n, || None::<T>);
        for (i, item) in out.iter_mut().enumerate() {
            if !self.queue[i].is_empty() {
                *item = Some(self.queue[i].remove(0));
            }
        }
        if !out.iter().any(|m| m.is_none()) {
            return Some(out);
        }
        for _ in 0..self.n {
            if let Some(point) = self.iter.next() {
                let idx = self.dist.sample(&mut self.rng);
                if out[idx].is_some() {
                    self.queue[idx].push(point);
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

/// An iterator trait to generate splits from a data stream of unknown lenght.
///
/// # Example
///
/// ```
/// use rand_split::PartsSplit;
///
/// let splits = [4., 7., 2.5];
///
/// let cont = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// let (mut train, mut test, mut valid) = (Vec::new(), Vec::new(), Vec::new());
/// cont.iter().split_parts(&splits).for_each(|sp| {
///     train.push(sp[0]);
///     test.push(sp[1]);
///     valid.push(sp[2])
/// });
/// println!(
///     "Train: {:#?}, Test: {:#?}, Validation: {:#?}",
///     train, test, valid
/// );
/// ```
pub trait PartsSplit<T>: Iterator<Item = T> + Sized {
    /// `splits` contains weights that are not required to sum up to 1.
    fn split_parts(self, splits: &[f32]) -> PartsIterator<T, Self>;
}

impl<'a, T, I> PartsSplit<T> for I
where
    I: Iterator<Item = T>,
{
    fn split_parts(self, splits: &[f32]) -> PartsIterator<T, Self> {
        PartsIterator::new(self, splits)
    }
}
