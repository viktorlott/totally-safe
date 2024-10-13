use std::mem;

use totally_safe::TotallySafe;

#[derive(PartialEq, Debug)]
struct NonCopy(usize, Box<Option<usize>>);

fn main() {
    let mut value = NonCopy(20, Box::new(Some(100)));

    let copied_value = value.copy();

    let [a, b] = value.as_mut_alias_array();

    assert_eq!(a, b);

    *a.1 = Some(b.0);

    assert_eq!(value, copied_value);

    mem::forget(copied_value);

    let mut m = Box::new(100);
    let n = m.copy();

    println!("{m:?} {n:?}");
}
