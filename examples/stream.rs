extern crate rand_split;

use rand::prelude::*;
use rand_split::PartsSplit;

fn main() {
    // generate some random data
    let mut rng = rand::thread_rng();
    let mut cont: Vec<_> = (1..1000).collect();
    cont.shuffle(&mut rng);

    // weights for the splits (do not require to sum up to 1)
    let splits = [0.2, 0.7, 1.2, 4., 0.5];

    let mut buckets = vec![Vec::new(); 5];
    cont.iter().split_parts(&splits).for_each(|sp| {
        for (i, bucket) in buckets.iter_mut().enumerate() {
            bucket.push(sp[i])
        }
    });
    println!("{:#?}", buckets);
    println!(
        "Counts per bucket: {:?}",
        buckets
            .iter()
            .map(|el| el.iter().filter(|m| m.is_some()).count())
            .collect::<Vec<usize>>()
    );
}
