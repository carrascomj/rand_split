extern crate rand;
extern crate rand_distr;
extern crate rand_split;

use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use rand_split::TTVSplit;

/// Data augmentation with iterators
fn main() {
    // some dummy data
    let data = 0i32..1000000i32;
    // synthetic points to add per train instance
    let k = 3;
    // gaussian noise
    let normal = Normal::new(12.0, 6.0).unwrap();
    // splits will be stored here
    let (mut train, mut test, mut valid) = (Vec::new(), Vec::new(), Vec::new());
    let mut rng = thread_rng();

    data.split_ttv([0.8, 0.1, 0.1]).for_each(|[tr, te, va]| {
        // if a value is placed in a stream, add it to its bucket
        if let Some(point) = tr {
            // if train split, add the value and some augmented data
            train.push(point);
            train.extend(
                normal
                    .sample_iter(&mut rng)
                    .take(k)
                    .map(|noise: f32| point + noise as i32),
            );
        }
        if let Some(point) = te {
            test.push(point);
        }
        if let Some(point) = va {
            valid.push(point);
        }
    });

    // Check the result
    let total = (train.len() + test.len() + valid.len()) as f32;
    println!(
        "train: {:.2}%, test: {:.2}%, validation: {:.2}%",
        train.len() as f32 / total * 100.,
        test.len() as f32 / total * 100.,
        valid.len() as f32 / total * 100.,
    )
}
