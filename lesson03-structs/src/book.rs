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
    fn test_book_1() {
      let book1 = Book {
        isbn: 10,
        format: Paperback
      };
      assert_eq!(book.isbn, 10);
    }
}