use std::ops::{Deref, Index};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// A vector that is sorted from the greatest to lowest element
#[derive(Derivative, Serialize, Deserialize, Debug)]
#[derivative(Default(bound = ""))]
pub struct ReversedSortedVec<T: Ord>(Vec<T>);

impl<T: Ord> Deref for ReversedSortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: Ord> ReversedSortedVec<T> {
    /// Inserts the value and returns the index
    pub fn push(&mut self, value: T) -> usize {
        let idx = self
            .iter()
            .position(|item| value > *item)
            .unwrap_or(self.0.len());
        self.0.insert(idx, value);
        idx
    }

    pub fn remove(&mut self, idx: usize) -> T { self.0.remove(idx) }
    pub fn modify_item_at<U>(&mut self, idx: usize, f: impl FnOnce(&mut T) -> U) -> U {
        let mut item = self.remove(idx);
        let out = f(&mut item);
        self.push(item);
        out
    }
}

impl<T: Ord> FromIterator<T> for ReversedSortedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec: Vec<T> = iter.into_iter().collect();
        vec.sort();
        vec.reverse();
        Self(vec)
    }
}

impl<T: Ord> Index<usize> for ReversedSortedVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output { &self.0[idx] }
}
