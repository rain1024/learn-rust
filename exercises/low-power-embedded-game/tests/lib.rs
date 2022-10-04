use low_power_embedded_game::*;

#[test]
fn divmod_1() {
  assert_eq!(divmod(10, 3), (3, 1));
}

#[test]
fn divmod_2() {
  assert_eq!(divmod(5, 2), (2, 1));
}

#[test]
fn evens_1() {
  let mut even_ints = evens(0_u8..);
  assert_eq!(even_ints.next(), Some(0));
  assert_eq!(even_ints.next(), Some(2));
  assert_eq!(even_ints.next(), Some(4));
}

#[test]
fn position_1() {
  assert_eq!(Position(3, 4).manhattan(), 7);
}