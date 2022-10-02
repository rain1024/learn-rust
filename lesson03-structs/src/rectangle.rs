pub struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let rect1 = Rectangle {
          width: 10,
          height: 20
        };
        assert_eq!(rect1.width, 10);
    }

    #[test]
    fn test_2() {
        let rect1 = Rectangle {
          width: 10,
          height: 20
        };
        let area: u32 = rect1.area();
        assert_eq!(area, 200);
    }
}