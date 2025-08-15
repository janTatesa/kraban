use std::ops::Deref;

#[derive(Debug, Default, Clone, Copy)]
pub struct WrappingUsize {
    value: usize,
    max: usize
}

impl Deref for WrappingUsize {
    type Target = usize;

    fn deref(&self) -> &Self::Target { &self.value }
}

impl WrappingUsize {
    pub const fn new(max: usize) -> Self { Self { value: 0, max } }
    pub const fn new_with_value(max: usize, value: usize) -> Self { Self { value, max } }
    pub const fn increment(&mut self) { self.value = (self.value + 1) % (self.max + 1) }
    pub const fn decrement(&mut self) {
        self.value = match self.value {
            0 => self.max,
            _ => self.value - 1
        }
    }
}
