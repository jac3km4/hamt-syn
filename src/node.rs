use std::hash::Hash;

pub trait Node
where
    Self: Sized,
{
    type Key: Hash + PartialEq;
    type Value;

    fn insert(&self, k: Self::Key, v: Self::Value) -> (Self, bool);
    fn delete(&self, k: &Self::Key) -> Option<Self>;
    fn find(&self, k: &Self::Key) -> Option<&Self::Value>;
    fn first_rest(&self) -> Option<(&Self::Key, &Self::Value, Self)>;
    fn is_singleton(&self) -> bool; // for normalization
    fn size(&self) -> usize; // for debugging
}
