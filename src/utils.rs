use rand::seq::SliceRandom;
use std::collections::BTreeSet;

pub fn vec2btree<T: Ord>(v: Vec<T>) -> BTreeSet<T> {
    v.into_iter().collect()
}

pub fn pick_random_iter<S>(s: &S, n: usize) -> Vec<S::Item>
where
    S: IntoIterator + Clone,
{
    let mut rand = rand::thread_rng();
    let mut v = s.clone().into_iter().collect::<Vec<_>>();
    v.shuffle(&mut rand);
    v.into_iter().take(n).collect()
}
