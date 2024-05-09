#[derive(Debug, Clone, Copy, Hash)]
pub struct UnorderedPair<T: Ord> {
    pub first: T,
    pub second: T,
}

impl<T: Ord> UnorderedPair<T> {
    pub fn new(first: T, second: T) -> Self {
        if first < second {
            UnorderedPair { first, second }
        } else {
            UnorderedPair {
                first: second,
                second: first,
            }
        }
    }
}

impl<T: Ord> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<T: Ord> Eq for UnorderedPair<T> {}
