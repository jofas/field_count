pub use field_count_derive::FieldCount;

pub trait FieldCount {
  fn field_count() -> usize;
}

// TODO: write tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
