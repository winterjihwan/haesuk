use std::{collections::HashMap, fmt::Debug, hash::Hash, sync::Arc};

#[derive(Default, Debug)]
pub struct Bimap<K, V> {
    forward: HashMap<Arc<K>, Arc<V>>,
    backward: HashMap<Arc<V>, Arc<K>>,
}

impl<K, V> Bimap<K, V>
where
    K: Eq + Hash,
    V: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) {
        let k_arc = Arc::new(k);
        let v_arc = Arc::new(v);
        self.forward.insert(k_arc.clone(), v_arc.clone());
        self.backward.insert(v_arc, k_arc);
    }

    pub fn extract_val(&self, k: &K) -> &V {
        self.forward.get(k).unwrap()
    }

    pub fn extract_key(&self, v: &V) -> &K {
        self.backward.get(v).unwrap()
    }
}
