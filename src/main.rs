#![allow(unuser)]
use std::cell::Cell;
fn main() {
    let s = "sdf".to_string();
    
    let s_ref = &s;
    
    let s2 = *s_ref;
}


