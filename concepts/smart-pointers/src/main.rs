// Follow this video tutorial Rust: Smart Pointers, Linked Lists - CS196 SP20
// https://www.youtube.com/watch?v=2q1AzGUwL7M

#[derive(Debug)]
struct Node<T> {
  next: Option<Box<Node<T>>>,
  value: T
}

impl<T> Node<T> {
  fn set_next(&mut self, next: Node<T>){
    self.next = Some(Box::new(next));
  }
}

fn main(){
  let mut head = Node {
    next: None,
    value : 0
  };

  let mut node1 = Node {
    next: None,
    value: 1
  };

  let node2 = Node {
    next: None,
    value: 2
  };

  node1.set_next(node2);
  head.set_next(node1);
  

  println!("{:?}", head);
}