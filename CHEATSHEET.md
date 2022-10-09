# Rust Cheatsheet

## Patern Matching

```rs
let x = 5;

match x {
  // matching literals
  1 => println!("one"),
  // matching multiple patterns 
  2|3 => println!("two or three"),
  // matching rangs
  4..=9 => println!("within range"),
  // matching named variables
  x => println!("{}", x),
  // default case (ignores value)
  _ => println!("default case")
}
```

# Functional Programming 

```rs
/* Map */
let a : Vec<i32> = vec![1, 2, 3, 4];
a.into_iter().map(|x| x * 2).collect::<Vec<i32>>()
// [2, 4, 6, 8]

/* Filter */
let a : Vec<i32> = vec![1, 2, 3, 4];
a.into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>()
// [2, 4]

/* Fold */
let a : Vec<i32> = vec![1, 2, 3, 4];
a.into_iter().fold(0, |s, x| s + x)
// 10

/* Windows */ 
let a = vec![1, 2, 3, 4];
for item in a.windows(3) {
  println!("{:?}", item);    
}
// [1, 2, 3]
// [2, 3, 4]
```