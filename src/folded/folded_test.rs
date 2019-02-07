#[test]
fn coupled_vec_init_test() {
    use crate::folded::folded_vec::FoldedVec;

    let flat_vec = vec![1, 2, 3, 4, 5];
    let mut coupled_vec = FoldedVec::new(3);
    coupled_vec.init(flat_vec);
}

#[test]
fn coupled_vec_into_test() {
    use std::collections::VecDeque;
    use crate::folded::{FoldedCollection, FoldableCollection};

    let flat_vec = vec![1, 2, 3, 4, 5];
    let coupled_vec = flat_vec.fold(3);

    let mut expected = VecDeque::new();
    expected.push_back(vec![1, 2, 3]);
    expected.push_back(vec![4, 5]);

    assert_eq!(coupled_vec.folded_collection(), expected);
}

#[test]
fn coupled_vec_flatten_test() {
    use crate::folded::{FoldedCollection, FoldableCollection};

    let flat_vec = vec![1, 2, 3, 4, 5];
    let coupled_vec = flat_vec.fold(3);

    let flattened = coupled_vec.flatten();
    let expected = vec![1, 2, 3, 4, 5];

    assert_eq!(flattened, expected);
}