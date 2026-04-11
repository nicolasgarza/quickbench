use crate::{runner::Runner, wrapper::Container};

impl<K, V, I: Container<K, V>> Runner<K, V, I>
where
    K: Clone,
    V: Clone + PartialEq + std::fmt::Debug,
{
    pub fn run_stress(&mut self, keys: Vec<K>, values: Vec<V>)
    where
        K: Clone,
        V: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(keys.len(), values.len());

        for (k, v) in keys.iter().zip(values.iter()) {
            self.container.put(k.clone(), v.clone());
        }

        for (k, v) in keys.iter().zip(values.iter()) {
            assert_eq!(self.container.get(k), Some(v));
        }

        for k in keys.iter() {
            assert!(self.container.remove(k).is_some());
            assert_eq!(self.container.get(k), None);
        }
    }
}
