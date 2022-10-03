enum BookFormat {
  Paperback,
  Hardback,
  Ebook
}

struct Book {
  isbn: i32,
  format: BookFormat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
      let book1 = Book {
        isbn: 10,
        format: BookFormat::Paperback
      };
      assert_eq!(book1.isbn, 10);
    }
}