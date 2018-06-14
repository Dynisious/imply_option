An implementation of [this RFC](https://github.com/rust-lang/rfcs/pull/2180).

Provides a single `trait` which allows the construction of an `Option` based on a bool value.
```
extern crate imply_option;

use imply_option::*;

fn main() {
    let pass = true;

    assert_eq!(pass.then(1), Some(1));
    assert_eq!(pass.then_do(|| 1), Some(1));

    let fail = false;

    assert_eq!(fail.then(1), None);
}
```
