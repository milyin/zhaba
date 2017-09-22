use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub mod form;

pub fn hash<T>(v: &T) -> u64
where
    T: Hash,
{
    let mut s = DefaultHasher::new();
    v.hash(&mut s);
    s.finish()
}

