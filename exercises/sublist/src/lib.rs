use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn count_freqs<T:Eq+Hash>(x: &[T]) -> HashMap<&T, usize> {
  x.iter().fold(HashMap::new(), |mut m, val| {
      m.entry(val).and_modify(|freq| { *freq += 1 }).or_insert(1);
      m
  })
}

pub fn sublist<T: Hash + Eq + PartialEq>(_v1: &[T], _v2: &[T]) -> Comparison {
  if _v1.len() > _v2.len() {
    let result = sublist(_v2, _v1);
    if result == Comparison::Sublist {
      return Comparison::Superlist;
    } else {
      return result;
    }
  }
  let m1 = count_freqs(&_v1);
  let m2 = count_freqs(&_v2);
  for (k, f1) in m1.iter() {
      match m2.get(k) {
          Some(f2) => if f1 > f2 {
              return Comparison::Unequal;
          }
          None => {
              return Comparison::Unequal;
          }
      }
  }
  if _v1.len() == _v2.len() {
    return Comparison::Equal;
  } else {
    return Comparison::Sublist;
  }
}
