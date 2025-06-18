use std::ops::{Index, IndexMut};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use tap::Tap;

/// A vector that is sorted from the greatest to lowest element
#[derive(Derivative, Serialize, Deserialize, Debug)]
#[derivative(Default(bound = ""))]
pub struct ReversedSortedVec<T: Ord>(Vec<T>);
impl<T: Ord> ReversedSortedVec<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        vec.sort();
        vec.reverse();
        Self(vec)
    }

    /// Inserts the value and returns the index
    pub fn push(&mut self, value: T) -> usize {
        let index = self
            .0
            .iter()
            .position(|item| value > *item)
            .unwrap_or_else(|| self.len());
        self.0.insert(index, value);
        index
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.0
    }

    pub fn inner_owned(self) -> Vec<T> {
        self.0
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    /// Calls the closure to the element at the index and return its new position
    pub fn map_item_at<F: FnOnce(T) -> T>(&mut self, index: usize, closure: F) -> usize {
        let item = self.remove(index);
        self.push(closure(item))
    }
}

impl<T: Ord> FromIterator<T> for ReversedSortedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(
            iter.into_iter()
                .collect::<Vec<T>>()
                .tap_mut(|vec| vec.sort()),
        )
    }
}

impl<T: Ord> Index<usize> for ReversedSortedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner().get(index).unwrap()
    }
}

impl<T: Ord> IndexMut<usize> for ReversedSortedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}
