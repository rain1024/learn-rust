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