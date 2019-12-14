mod folded_vec;
mod folded_test;

use std::collections::{VecDeque};

pub trait FoldedCollection {
    type Collection: IntoIterator;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn folded_collection(self) -> VecDeque<Self::Collection>;
    fn flatten(self) -> Self::Collection;
    
    fn front(&self) -> Option<&Self::Collection>;
    fn pop_front(&mut self) -> Option<Self::Collection>;
    fn back(&self) -> Option<&Self::Collection>;
    fn pop_back(&mut self) -> Option<Self::Collection>;

    fn clear(&mut self);
}

pub trait FoldableCollection {
    type Collection: FoldedCollection;

    fn fold(self, unit_len: usize) -> Self::Collection;
}
