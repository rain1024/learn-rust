## Health Statistics

> ⚠️ This exercise is fully copied from 
> https://exercism.org/tracks/rust/exercises/health-statistics/

You're working on implementing a health-monitoring system. As part of that, you need to keep track of users' health statistics.

You'll start with some stubbed functions in an impl block as well as the following struct definition:

```rs
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}
```

Your goal is to implement the stubbed out methods on the User struct defined in the impl block.

For example, the new method should return an instance of the User struct with the specified name, age, and weight values.

```rs
let mut bob = User::new(String::from("Bob"), 32, 155.2);
// Returns: a User with name "Bob", age 32, and weight 155.2
```


The weight method should return the weight of the User.

```rs
bob.weight();
// Returns: 155.2
```

The set_age method should set the age of the User.

```rs
bob.set_age(33);
// Updates Bob's age to 33; happy birthday Bob!
```

Have fun!