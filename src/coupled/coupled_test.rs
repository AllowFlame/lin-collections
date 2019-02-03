use std::collections::{VecDeque};

use crate::coupled::coupled_vec::CoupledVec;
use crate::coupled::{CoupledCollection, IntoCoupledCollection};

#[test]
fn coupled_vec_init_test() {
    let flat_vec = vec![1, 2, 3, 4, 5];
    let mut coupled_vec = CoupledVec::new(3);
    coupled_vec.init(flat_vec);
}

#[test]
fn coupled_vec_into_test() {
    let flat_vec = vec![1, 2, 3, 4, 5];
    let coupled_vec = flat_vec.into_coupled(3);

    let mut expected = VecDeque::new();
    expected.push_back(vec![1, 2, 3]);
    expected.push_back(vec![4, 5]);

    assert_eq!(coupled_vec.coupled_collection(), expected);
}

#[test]
fn coupled_vec_flatten_test() {
    let flat_vec = vec![1, 2, 3, 4, 5];
    let coupled_vec = flat_vec.into_coupled(3);

    let flattened = coupled_vec.flatten();
    let expected = vec![1, 2, 3, 4, 5];

    assert_eq!(flattened, expected);
}