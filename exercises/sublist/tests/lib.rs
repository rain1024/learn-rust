use sublist::*;

#[test]
fn sublist_1() {
  let v1 = &[1, 2, 3];
  let v2 = &[1, 2, 3];
  assert_eq!(sublist(v1, v2), Comparison::Equal);
}
