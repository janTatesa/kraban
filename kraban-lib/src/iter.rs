// If the iterator is empty, return an iterator of the default value
pub struct DefaultIter<I: Iterator> {
    default: Option<I::Item>,
    iter: I,
}

impl<T, I> Iterator for DefaultIter<I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => {
                self.default = None;
                Some(item)
            }
            None => self.default.take(),
        }
    }
}

pub trait IterExt: Iterator {
    fn default(self, default: Self::Item) -> DefaultIter<Self>
    where
        Self: Sized,
    {
        DefaultIter {
            default: Some(default),
            iter: self,
        }
    }
}

impl<T: Iterator> IterExt for T {}
