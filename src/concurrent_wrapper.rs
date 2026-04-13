use std::sync::Arc;

pub trait ConcurrentContainer<K, V>: Send + Sync {
    fn put(&self, key: K, value: V);
    fn get(&self, key: &K) -> Option<V>;
    fn remove(&self, key: &K) -> Option<V>;
}

impl<K, V, I: ConcurrentContainer<K, V>> ConcurrentContainer<K, V> for Arc<I> {
    fn put(&self, key: K, value: V) {
        self.as_ref().put(key, value)
    }
    fn get(&self, key: &K) -> Option<V> {
        self.as_ref().get(key)
    }
    fn remove(&self, key: &K) -> Option<V> {
        self.as_ref().remove(key)
    }
}
