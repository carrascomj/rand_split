extern crate rand_split;

use rand_split::TTVSplit;

fn main() {
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
