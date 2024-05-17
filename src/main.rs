#![allow(unuser)]
use std::cell::Cell;
fn main() {

}


fn two_refs<'big: 'small, 'small>(
    // NOTE: these two lines changed
    big: Cell<std::ptr::NonNull<&'big u32>>,
    small: Cell<std::ptr::NonNull<&'big u32>>,
) {
    take_two(big, small);
}

fn take_two<T>(_val1: T, _val2: T) { }