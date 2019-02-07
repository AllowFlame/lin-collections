use std::collections::{VecDeque};

use crate::folded::{FoldedCollection, FoldableCollection};

pub struct FoldedVec<T> {
    unit_len: usize,
    folded_vec: VecDeque<Vec<T>>,
}

impl<T> FoldedVec<T> {
    pub fn new(unit_size: usize) -> FoldedVec<T> {
        FoldedVec {
            unit_len: unit_size,
            folded_vec: VecDeque::new(),
        }
    }

    pub fn init(&mut self, flat_vec: Vec<T>) {
        use std::borrow::BorrowMut;

        let unit_len = self.unit_len;
        let folded_vec = self.folded_vec.borrow_mut();
        let mut flat_index: usize = 0;
        for item in flat_vec {
            let remainder = flat_index % unit_len;
            let inner = match remainder {
                0 => {
                    let inner_vec: Vec<T> = Vec::new();
                    folded_vec.push_back(inner_vec);
                    folded_vec.back_mut().unwrap()
                },
                _ => {
                    folded_vec.back_mut().unwrap()
                }
            };
            inner.push(item);
            flat_index += 1;
        }
    }
}

impl<T> FoldableCollection for Vec<T> {
    type Collection = FoldedVec<T>;

    fn fold(self, unit_len: usize) -> FoldedVec<T> {
        let mut folded_vec = FoldedVec::new(unit_len);
        folded_vec.init(self);
        folded_vec
    }
}

#[derive(Clone)]
pub struct IntoIter<T> {
    inner: VecDeque<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<T> IntoIterator for FoldedVec<T> {
    type Item = Vec<T>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> IntoIter<Self::Item> {
        IntoIter {
            inner: self.folded_vec,
        }
    }
}

impl<T> FoldedCollection for FoldedVec<T> {
    type Collection = Vec<T>;

    fn is_empty(&self) -> bool {
        self.folded_vec.is_empty()
    }

    fn len(&self) -> usize {
        let outer_len = self.folded_vec.len();
        if outer_len == 0 {
            return 0;
        }

        let last_collection_len = match self.back() {
            Some(vec) => {
                vec.len()
            },
            None => 0,
        };

        (self.unit_len * outer_len) - (self.unit_len - last_collection_len)
    }

    fn folded_collection(self) -> VecDeque<Vec<T>> {
        self.folded_vec
    }

    fn flatten(self) -> Vec<T> {
        self.into_iter().flatten().collect::<Vec<T>>()
    }

    fn front(&self) -> Option<&Vec<T>> {
        self.folded_vec.front()
    }

    fn pop_front(&mut self) -> Option<Vec<T>> {
        self.folded_vec.pop_front()
    }

    fn back(&self) -> Option<&Vec<T>> {
        self.folded_vec.back()
    }

    fn pop_back(&mut self) -> Option<Vec<T>> {
        self.folded_vec.pop_back()
    }

    fn clear(&mut self) {
        self.unit_len = 0;
        self.folded_vec.clear();
    }
}