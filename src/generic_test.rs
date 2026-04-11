use crate::{runner::Runner, wrapper::Container};

impl<K, V, I: Container<K, V>> Runner<K, V, I>
where
    K: Clone,
    V: Clone + PartialEq + std::fmt::Debug,
{
    pub fn test_put_get(&mut self, key: K, value: V) {
        self.container.put(key.clone(), value.clone());
        assert_eq!(self.container.get(&key), Some(&value));
    }

    pub fn test_get_missing(&self, key: &K) {
        assert_eq!(self.container.get(key), None);
    }

    pub fn test_remove(&mut self, key: K, value: V) {
        self.container.put(key.clone(), value.clone());
        assert_eq!(self.container.remove(&key), Some(value));
        assert_eq!(self.container.get(&key), None);
    }

    pub fn test_overwrite(&mut self, key: K, v1: V, v2: V) {
        self.container.put(key.clone(), v1);
        self.container.put(key.clone(), v2.clone());
        assert_eq!(self.container.get(&key), Some(&v2));
    }

    pub fn run_generic(&mut self, k1: K, k2: K, v1: V, v2: V) {
        self.test_put_get(k1.clone(), v1.clone());
        self.test_remove(k2.clone(), v2.clone());
        self.test_overwrite(k1, v1, v2);
    }
}
