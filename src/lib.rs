//! This crate achieves the functionality of [sklearn's train_test_split](https://scikit-learn.org/stable/modules/generated/sklearn.model_selection.train_test_split.html)
//! to generate splits of the data (in this case, a slice), generalized for an
//! arbitrary number of splits. It
//! both provides functions (see [split_parts](https://docs.rs/rand_split/0.2.0/rand_split/fn.split_parts.html)) that work on slices and iterator traits (see [PartsSplit](https://docs.rs/rand_split/0.2.0/rand_split/trait.PartsSplit.html))to work
//! with streams of data.
//!
//! Check out the [examples in the repository](https://github.com/carrascomj/rand_split/tree/trunk/examples)
//! for more information.
use rand::prelude::*;

mod stream;
pub use stream::PartsSplit;

mod stream_ttv;
pub use stream_ttv::TTVSplit;

/// Split the elements of a container in randomized sets which contain a
/// a part (in `splits`) of the input.
///
/// # Example
///
/// ```
/// use rand_split::split_parts;
///
/// println!("{:#?}", split_parts(&mut [1,2,3,4,5,6,8,9,10], &[0.4, 0.2, 0.4]));
/// ```
pub fn split_parts<'a, T>(cont: &'a mut [T], splits: &[f32]) -> Vec<&'a mut [T]>
where
    T: Clone,
{
    let n = cont.len();
    let n_weights = splits.len();
    let total_weights = splits.iter().sum::<f32>();

    let mut out = Vec::with_capacity(n);
    let mut left = cont;
    left.shuffle(&mut rand::thread_rng());
    for sp in splits
        .iter()
        .map(|w| (w * (n as f32) / total_weights) as usize)
        .take(n_weights - 1)
    {
        let (right, l) = left.split_at_mut(sp);
        out.push(right);
        left = l;
    }
    out.push(left);

    out
}

/// Generate train-test splits. Wrapper around [`split_parts`](./split_parts)
/// # Examples
///
/// ```
/// use rand_split::train_test_split;
///
/// let mut cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// let total_len = cont.len();
/// let result = train_test_split(&mut cont, 0.8, 0.2);
/// assert_eq!(
///     result.iter().map(|inner| inner.len()).sum::<usize>(),
///     total_len
/// );
/// ```
pub fn train_test_split<T>(cont: &mut [T], train: f32, test: f32) -> Vec<&mut [T]>
where
    T: Clone,
{
    split_parts(cont, &[train, test])
}

/// Generate train-test-validation splits. Wrapper around [`split_parts`](./split_parts)
/// # Examples
///
/// ```
/// use rand_split::ttv_split;
///
/// let mut cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// let total_len = cont.len();
/// let result = ttv_split(&mut cont, 0.6, 0.2, 0.2);
/// assert_eq!(
///     result.iter().map(|inner| inner.len()).sum::<usize>(),
///     total_len
/// );
/// ```
pub fn ttv_split<T>(cont: &mut [T], train: f32, test: f32, validation: f32) -> Vec<&mut [T]>
where
    T: Clone,
{
    split_parts(cont, &[train, test, validation])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_split_sizes_even_number() {
        let splits = [0.2, 0.7, 0.1];
        let mut cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let total_len = cont.len();
        let result = split_parts(&mut cont, &splits);
        assert_eq!(
            result.iter().map(|inner| inner.len()).sum::<usize>(),
            total_len
        );
        assert_eq!(result.len(), splits.len());
    }

    #[test]
    fn correct_split_sizes_odd_number() {
        let splits = [0.2, 0.2, 0.3, 0.3];
        let mut cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let total_len = cont.len();
        let result = split_parts(&mut cont, &splits);
        assert_eq!(
            result.iter().map(|inner| inner.len()).sum::<usize>(),
            total_len
        );
        assert_eq!(result.len(), splits.len());
    }

    #[test]
    fn unnormalized_weights_is_approx_correct() {
        let splits = [2., 2., 3., 10.];
        let mut cont = (0..1999).collect::<Vec<usize>>();
        let result = split_parts(&mut cont, &splits);
        assert!(result[3].len() > result[0..2].iter().map(|sp| sp.len()).sum());
        assert!(result[1].len() < result[2].len());
    }

    #[test]
    fn unnormalized_weights_preserve_data() {
        let splits = [2., 2., 3., 6.];
        let mut cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let total_len = cont.len();
        let result = split_parts(&mut cont, &splits);
        assert_eq!(
            result.iter().map(|inner| inner.len()).sum::<usize>(),
            total_len
        );
        assert_eq!(result.len(), splits.len());
    }
}
