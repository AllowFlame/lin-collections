use std::collections::VecDeque;

pub trait ToVec<T> {
    fn to_vec(&self) -> Vec<T>;
}

impl ToVec<u8> for [u8] {
    fn to_vec(&self) -> Vec<u8> {
        let mut vector = Vec::new();
        vector.extend_from_slice(&self);
        vector
    }
}

pub trait ToVecDeque<T> {
    fn to_vec_deque(&self) -> VecDeque<T>;
}

impl ToVecDeque<u8> for [u8] {
    fn to_vec_deque(&self) -> VecDeque<u8> {
        let mut deque = VecDeque::new();
        for item in self {
            deque.push_back(*item);
        }
        deque
    }
}