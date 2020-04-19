//extern crate array_stump;
use std::cmp::Ordering;

use super::helpers::{gen_rand_values_i32, shuffle_clone};

use array_stump::ArrayStump;
use super::alternatives::splay::SplaySet;


fn cmp(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}


#[test]
fn insert_and_remove() {
    let repeats = 8;
    for _ in 0 .. repeats {
        for array_len in 0 .. 64 {
            for cap in 2 .. 64 {
                let values = gen_rand_values_i32(array_len);
                // println!("\nInserting: {:?}", values);

                let mut set_a = ArrayStump::new_explicit(cmp, cap);
                let mut set_b = SplaySet::new(cmp);

                for x in &values {
                    let res_a = set_a.insert(*x);
                    let res_b = set_b.insert(*x);
                    // println!("{} {} {}", x, res_a, res_b);
                    assert_eq!(res_a, res_b);
                    assert_eq!(set_a.len(), set_b.len());
                    assert_eq!(set_a.collect(), set_b.collect());
                }

                let values = shuffle_clone(&values);
                for x in &values {
                    let res_a = set_a.remove(x);
                    let res_b = set_b.remove(x);
                    // println!("{} {} {}", x, res_a, res_b);
                    assert_eq!(res_a, res_b);
                    assert_eq!(set_a.len(), set_b.len());
                    assert_eq!(set_a.collect(), set_b.collect());
                }

            }
        }
    }
}
