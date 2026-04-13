use std::thread;

use crate::concurrent_wrapper::ConcurrentContainer;
use crate::runner::Runner;

impl<K, V, I> Runner<K, V, I>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + PartialEq + std::fmt::Debug + 'static,
    I: ConcurrentContainer<K, V> + Clone + 'static,
{
    /// Two threads write the same key, value must be present after.
    pub fn test_concurrent_put_get(&self, key: K, value: V) {
        let h1 = {
            let c = self.container.clone();
            let (k, v) = (key.clone(), value.clone());
            thread::spawn(move || c.put(k, v))
        };

        let h2 = {
            let c = self.container.clone();
            let (k, v) = (key.clone(), value.clone());
            thread::spawn(move || c.put(k, v))
        };

        h1.join().unwrap();
        h2.join().unwrap();
        assert_eq!(self.container.get(&key), Some(value));
    }

    /// Two threads write distinct keys,  neither write should be lost.
    pub fn test_concurrent_distinct_puts(&self, k1: K, v1: V, k2: K, v2: V) {
        let h1 = {
            let c = self.container.clone();
            let (k, v) = (k1.clone(), v1.clone());
            thread::spawn(move || c.put(k, v))
        };
        let h2 = {
            let c = self.container.clone();
            let (k, v) = (k2.clone(), v2.clone());
            thread::spawn(move || c.put(k, v))
        };
        h1.join().unwrap();
        h2.join().unwrap();
        assert_eq!(self.container.get(&k1), Some(v1));
        assert_eq!(self.container.get(&k2), Some(v2));
    }

    /// Two threads both remove the same key, key must be absent after
    pub fn test_concurrent_remove(&self, key: K, value: V) {
        self.container.put(key.clone(), value);
        let h1 = {
            let c = self.container.clone();
            let k = key.clone();
            thread::spawn(move || {
                c.remove(&k);
            })
        };
        let h2 = {
            let c = self.container.clone();
            let k = key.clone();
            thread::spawn(move || {
                c.remove(&k);
            })
        };
        h1.join().unwrap();
        h2.join().unwrap();
        assert_eq!(self.container.get(&key), None);
    }

    pub fn run_concurrent_generic(&self, k1: K, k2: K, v1: V, v2: V) {
        self.test_concurrent_put_get(k1.clone(), v1.clone());
        self.test_concurrent_distinct_puts(k1.clone(), v1.clone(), k2.clone(), v2.clone());
        self.test_concurrent_remove(k1, v1);
    }
}
