use crate::{concurrent_wrapper::ConcurrentContainer, runner::Runner};
use std::thread;

impl<K, V, I> Runner<K, V, I>
where
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + PartialEq + std::fmt::Debug + 'static,
    I: ConcurrentContainer<K, V> + Clone + 'static,
{
    pub fn stress_puts_with(&self, threads: usize, ops: usize, pairs: Vec<(K, V)>) {
        assert_eq!(pairs.len(), threads);
        let handles: Vec<_> = pairs
            .iter()
            .map(|(k, v)| {
                let c = self.container.clone();
                let k = k.clone();
                let v = v.clone();
                thread::spawn(move || {
                    for _ in 0..ops {
                        c.put(k.clone(), v.clone());
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        for (k, v) in &pairs {
            assert_eq!(self.container.get(k), Some(v.clone()));
        }
    }

    pub fn stress_put_remove_with(&self, threads: usize, ops: usize, pairs: Vec<(K, V)>) {
        assert_eq!(pairs.len(), threads);
        let handles: Vec<_> = pairs
            .iter()
            .map(|(k, v)| {
                let c = self.container.clone();
                let k = k.clone();
                let v = v.clone();
                thread::spawn(move || {
                    for _ in 0..ops {
                        c.put(k.clone(), v.clone());
                        c.remove(&k);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        for (k, _) in &pairs {
            assert_eq!(self.container.get(k), None);
        }
    }

    pub fn stress_contention_with(&self, threads: usize, ops: usize, key: K, value: V) {
        let handles: Vec<_> = (0..threads)
            .map(|i| {
                let c = self.container.clone();
                let k = key.clone();
                let v = value.clone();
                thread::spawn(move || {
                    for j in 0..ops {
                        if (i + j) % 2 == 0 {
                            c.put(k.clone(), v.clone());
                        } else {
                            c.remove(&k);
                        }
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
    }
}

impl<I> Runner<u64, u64, I>
where
    I: ConcurrentContainer<u64, u64> + Clone + 'static,
{
    pub fn run_stress_u64(&self, threads: usize, ops: usize) {
        let pairs: Vec<(u64, u64)> = (0..threads as u64).map(|i| (i, i)).collect();
        self.stress_puts_with(threads, ops, pairs.clone());
        self.stress_put_remove_with(threads, ops, pairs);
        self.stress_contention_with(threads, ops, 0u64, 0u64);
    }
}
