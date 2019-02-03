use std::collections::{VecDeque};

use crate::coupled::{CoupledCollection, IntoCoupledCollection};

pub struct CoupledVec<T> {
    unit_len: usize,
    coupled_vec: VecDeque<Vec<T>>,
}

impl<T> CoupledVec<T> {
    pub fn new(unit_size: usize) -> CoupledVec<T> {
        CoupledVec {
            unit_len: unit_size,
            coupled_vec: VecDeque::new(),
        }
    }

    pub fn init(&mut self, flat_vec: Vec<T>) {
        use std::borrow::BorrowMut;

        let unit_len = self.unit_len;
        let mut coupled_vec = self.coupled_vec.borrow_mut();
        let mut flat_index: usize = 0;
        for item in flat_vec {
            let remainder = flat_index % unit_len;
            let mut inner = match remainder {
                0 => {
                    let inner_vec: Vec<T> = Vec::new();
                    coupled_vec.push_back(inner_vec);
                    coupled_vec.back_mut().unwrap()
                },
                _ => {
                    coupled_vec.back_mut().unwrap()
                }
            };
            inner.push(item);
            flat_index += 1;
        }
    }
}

impl<T> IntoCoupledCollection for Vec<T> {
    type Collection = CoupledVec<T>;

    fn into_coupled(self, unit_len: usize) -> CoupledVec<T> {
        let mut coupled_vec = CoupledVec::new(unit_len);
        coupled_vec.init(self);
        coupled_vec
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

impl<T> IntoIterator for CoupledVec<T> {
    type Item = Vec<T>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(mut self) -> IntoIter<Self::Item> {
        IntoIter {
            inner: self.coupled_vec,
        }
    }
}

impl<T> CoupledCollection for CoupledVec<T> {
    type Collection = Vec<T>;

    fn is_empty(&self) -> bool {
        self.coupled_vec.is_empty()
    }

    fn len(&self) -> usize {
        let outer_len = self.coupled_vec.len();
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

    fn front(&self) -> Option<&Vec<T>> {
        self.coupled_vec.front()
    }

    fn pop_front(&mut self) -> Option<Vec<T>> {
        self.coupled_vec.pop_front()
    }

    fn back(&self) -> Option<&Vec<T>> {
        self.coupled_vec.back()
    }

    fn pop_back(&mut self) -> Option<Vec<T>> {
        self.coupled_vec.pop_back()
    }

    fn clear(&mut self) {
        self.unit_len = 0;
        self.coupled_vec.clear();
    }
}