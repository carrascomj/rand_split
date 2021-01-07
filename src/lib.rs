//! This crate achieves the functionality of [sklearn's train_test_split](https://scikit-learn.org/stable/modules/generated/sklearn.model_selection.train_test_split.html)
//! to generate splits of the data (in this case, a slice), generalized for an
//! arbitrary number of splits (see [split_parts](./fn.split_parts.html)). It 
//! both provides functions that work on slices and iterator traits to work
//! with streams of data.
use rand::seq::index::sample;

mod stream;
pub use stream::PartsSplit;

mod stream_ttv;
pub use stream_ttv::TTVSplit;

/// Split the elements of a container in randomized sets which contain a
/// a part (in `splits`) of the input.
///
/// # Errors
///
/// Return an error if the parts in `splits` do not sum up to 1.
///
/// # Example
///
/// ```
/// use rand_split::split_parts;
///
/// println!("{:#?}", split_parts(&[1,2,3,4,5,6,8,9,10], &[0.4, 0.2, 0.4]));
/// ```
pub fn split_parts<T>(cont: &[T], splits: &[f32]) -> Result<Vec<Vec<T>>, &'static str>
where
    T: Clone,
{
    if (splits.iter().sum::<f32>() - 1.).abs() > 0.001 {
        return Err("splits must sum 1!");
    }
    let n = cont.len();
    let shuffled = sample(&mut rand::thread_rng(), n, n).into_vec();
    let splits: Vec<usize> = {
        let mut tmp: Vec<usize> = splits
            .iter()
            .map(|x| (x * n as f32) as usize)
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect();
        // account for rounding errors
        tmp[splits.len() - 1] += n - tmp[tmp.len() - 1];
        tmp
    };
    Ok([0]
        .iter()
        .chain(splits[0..(splits.len() - 1)].iter())
        .zip(splits.iter())
        .map(|(start, end)| {
            shuffled[*start..*end]
                .iter()
                .map(|i| cont[*i].clone())
                .collect::<Vec<T>>()
        })
        .collect())
}

/// Generate train-test splits. Wrapper around [`split_parts`](./split_parts)
/// # Examples
///
/// ```
/// use rand_split::train_test_split;
///
/// let cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// let total_len = cont.len();
/// let result = train_test_split(&cont, 0.8, 0.2).unwrap();
/// assert_eq!(
///     result.iter().map(|inner| inner.len()).sum::<usize>(),
///     total_len
/// );
/// ```
pub fn train_test_split<T>(cont: &[T], train: f32, test: f32) -> Result<Vec<Vec<T>>, &'static str>
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
/// let cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// let total_len = cont.len();
/// let result = ttv_split(&cont, 0.6, 0.2, 0.2).unwrap();
/// assert_eq!(
///     result.iter().map(|inner| inner.len()).sum::<usize>(),
///     total_len
/// );
/// ```
pub fn ttv_split<T>(
    cont: &[T],
    train: f32,
    test: f32,
    validation: f32,
) -> Result<Vec<Vec<T>>, &'static str>
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
        let cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let total_len = cont.len();
        let result = split_parts(&cont, &splits).unwrap();
        assert_eq!(
            result.iter().map(|inner| inner.len()).sum::<usize>(),
            total_len
        );
        assert_eq!(result.len(), splits.len());
    }

    #[test]
    fn correct_split_sizes_odd_number() {
        let splits = [0.2, 0.2, 0.3, 0.3];
        let cont = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let total_len = cont.len();
        let result = split_parts(&cont, &splits).unwrap();
        assert_eq!(
            result.iter().map(|inner| inner.len()).sum::<usize>(),
            total_len
        );
        assert_eq!(result.len(), splits.len());
    }
}
