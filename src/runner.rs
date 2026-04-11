use std::marker::PhantomData;

use crate::wrapper::Container;

/// Currently has generic tests for read/write/delete functions
/// and stress tests
/// TODO: add tests for concurrent data structures
pub struct Runner<K, V, I: Container<K, V>> {
    pub container: I,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V, I: Container<K, V>> Runner<K, V, I> {
    pub fn new(container: I) -> Self {
        Runner {
            container,
            _phantom: PhantomData,
        }
    }
}
