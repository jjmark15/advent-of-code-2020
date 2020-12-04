use std::fmt::{Display, Formatter};

pub struct Answer<T: Display> {
    value: T,
}

impl<T: Display> Answer<T> {
    pub fn new(value: T) -> Self {
        Answer { value }
    }
}

impl<T: Display> Display for Answer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Answer: {}", self.value)
    }
}
