use std::marker::PhantomData;

/// Currently has generic tests for read/write/delete functions
/// and stress tests
pub struct Runner<K, V, I> {
    pub container: I,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V, I> Runner<K, V, I> {
    pub fn new(container: I) -> Self {
        Runner {
            container,
            _phantom: PhantomData,
        }
    }
}
