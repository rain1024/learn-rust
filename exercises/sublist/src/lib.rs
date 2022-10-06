use std::hash::Hash;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Hash + Eq + PartialEq>(_v1: &[T], _v2: &[T]) -> Comparison {
  if _v1.len() > _v2.len() {
    return sublist(_v2, _v1);
  }
  let m1 = _v1.into_iter().counts();
  let m2 = _v2.into_iter().counts();
  for (k, f1) in m1.iter() {
      match m2.get(k) {
          Some(f2) => if f1 > f2 {
              return -1;
          } else {
              return 0;
          }
          None => {
              return -2;
          }
      }
  }

  // let count = [1, 1, 1, 3, 3, 5].into_iter().counts();
  // let d1 = s1.difference(&s2).count();
  // let d2 = s2.difference(&s1).count();
  println!("{:?}", m1);
  return Comparison::Unequal;
}
