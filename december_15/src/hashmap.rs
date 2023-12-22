use indexmap::IndexMap;
use std::hash::Hash;

pub trait CustomHash {
    fn compute_hash(&self) -> usize;
}

#[derive(Debug)]
pub struct LinkedHashMap<K, V> {
    data: Vec<IndexMap<K, V>>,
}

impl<K, V> LinkedHashMap<K, V>
where
    K: CustomHash + Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(256);
        for _ in 0..256 {
            data.push(IndexMap::new());
        }
        LinkedHashMap { data }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash_index = key.compute_hash();
        let bucket = &mut self.data[hash_index];
        bucket.insert(key, value)
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let hash_index = key.compute_hash();
        let bucket = &mut self.data[hash_index];
        bucket.shift_remove(&key)
    }

    pub fn data(&self) -> &Vec<IndexMap<K, V>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_insert() {
        let mut map: LinkedHashMap<String, i32> = LinkedHashMap::new();
        map.insert(String::from("rn"), 1);
        map.remove(String::from("cm"));
        map.insert(String::from("qp"), 3);
        map.insert(String::from("cm"), 2);
        map.remove(String::from("qp"));
        map.insert(String::from("pc"), 4);
        map.insert(String::from("ot"), 9);
        map.insert(String::from("ab"), 5);
        map.remove(String::from("pc"));
        map.insert(String::from("pc"), 6);
        map.insert(String::from("ot"), 7);

        assert_eq!(
            map.data()[0]
                .iter()
                .map(|(&ref k, &v)| (k.clone(), v))
                .collect::<Vec<(String, i32)>>(),
            vec![(String::from("rn"), 1), (String::from("cm"), 2)]
        );
        assert_eq!(
            map.data()[3]
                .iter()
                .map(|(&ref k, &v)| (k.clone(), v))
                .collect::<Vec<(String, i32)>>(),
            vec![(String::from("ot"), 7), (String::from("ab"), 5), (String::from("pc"), 6)]
        );

        for i in 0..256 {
            if i != 0 || i != 3 {
                continue;
            }
            assert!(map.data()[i].is_empty());
        }
    }
}
