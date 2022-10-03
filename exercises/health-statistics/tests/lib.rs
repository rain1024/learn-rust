use health_statistics::*;

#[test]
fn test_age() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(32, bob.age());
}

#[test]
fn test_name() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(String::from("Bob"), bob.name());
}

#[test]
fn test_weight() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  assert_eq!(155.2, bob.weight());
}

#[test]
fn test_set_age() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  bob.set_age(33);
  assert_eq!(33, bob.age());
}

#[test]
fn test_set_weight() {
  let mut bob = User::new(String::from("Bob"), 32, 155.2);
  bob.set_weight(160.0);
  assert_eq!(160.0, bob.weight());
}

