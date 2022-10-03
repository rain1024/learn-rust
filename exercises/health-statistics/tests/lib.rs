use health_statistics::*;

#[test]
fn test_age() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(32, bob.age());
}

#[test]
fn test_weight() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(155.2, bob.weight());
}

