#[derive(Debug, Default, Clone, Copy)]
pub struct WrappingUsize {
    value: usize,
    max: usize,
}

impl WrappingUsize {
    pub const fn new(max: usize) -> Self {
        Self { value: 0, max }
    }

    pub const fn with_value(value: usize, max: usize) -> Self {
        Self {
            value: value % (max + 1),
            max,
        }
    }

    pub const fn value(self) -> usize {
        self.value
    }

    #[must_use = "method returns a new value"]
    pub const fn increment(self) -> Self {
        Self {
            value: (self.value + 1) % (self.max + 1),
            ..self
        }
    }

    #[must_use = "method returns a new value"]
    pub const fn decrement(self) -> Self {
        Self {
            value: match self.value {
                0 => self.max,
                _ => self.value - 1,
            },
            ..self
        }
    }

    pub const fn set_max(self, max: usize) -> Self {
        Self::with_value(self.value, max)
    }

    pub const fn set_value(self, value: usize) -> Self {
        Self::with_value(value, self.max)
    }
}
