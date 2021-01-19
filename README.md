[![Crates.io](https://img.shields.io/crates/v/rand_split.svg)](https://crates.io/crates/rand_split)
[![Documentation](https://docs.rs/rand_split/badge.svg)](https://docs.rs/rand_split/)
[![Build](https://github.com/carrascomj/rand_split/workflows/build/badge.svg)](https://github.com/carrascomj/rand_split/actions)

# rand_split

This crate achieves the functionality of [sklearn's train_test_split](https://scikit-learn.org/stable/modules/generated/sklearn.model_selection.train_test_split.html)
to generate splits of the data (in this case, a slice), generalized for an
arbitrary number of splits. It
both provides functions (see [split_parts](https://docs.rs/rand_split/0.2.0/rand_split/fn.split_parts.html))
that work on slices and iterator traits (see [PartsSplit](https://docs.rs/rand_split/0.2.0/rand_split/trait.PartsSplit.html))
to work with streams of data.

Check out the [examples in the repository](https://github.com/carrascomj/rand_split/tree/trunk/examples)
for more information.

## Example

```rust
use rand_split::split_parts;
use std::convert::TryInto;

let mut data = [1, 2, 3, 4, 5, 6, 8, 9, 10];
let [train, test, valid]: [&mut [u8]; 3] = split_parts(&mut data, &[0.4, 0.2, 0.4])
    .try_into()
    .expect("Works, length (3) comes from arguments.");
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

> README.md is automatically generated on CI using [cargo-readme](https://github.com/livioribeiro/cargo-readme). Please, modify README.tpl or lib.rs instead (check [the github worflow](https://github.com/carrascomj/rust_sbml/blob/trunk/.github/workflows/readme.yml) for more details).
