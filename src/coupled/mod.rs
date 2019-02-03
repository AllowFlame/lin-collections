pub mod coupled_vec;

mod coupled_test;

pub trait CoupledCollection {
    type Collection: IntoIterator;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    
    fn front(&self) -> Option<&Self::Collection>;
    fn pop_front(&mut self) -> Option<Self::Collection>;
    fn back(&self) -> Option<&Self::Collection>;
    fn pop_back(&mut self) -> Option<Self::Collection>;

    fn clear(&mut self);
}

pub trait IntoCoupledCollection {
    type Collection: CoupledCollection;

    fn into_coupled(self, unit_len: usize) -> Self::Collection;
}