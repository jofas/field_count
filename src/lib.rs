pub use field_count_derive::FieldCount;

pub trait FieldCount {
  fn field_count() -> usize;
}
