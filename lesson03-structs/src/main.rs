struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64
}

fn main() {
    let mut user1 = User {
      active: true,
      username: String::from("user1"),
      email: String::from("abc@example.com"),
      sign_in_count: 5
    };
    user1.email = String::from("abc1@example.com");
    println!("{:?}", user1.username);
}
