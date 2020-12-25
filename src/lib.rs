pub use field_count_derive::FieldCount;
pub use field_count_derive::RecursiveFieldCount;
pub use field_count_derive::FieldCountByType;

macro_rules! field_count_for_primitives {
  ($($x: ty),+) => {
    $(
      impl FieldCount for $x {
        fn field_count(&self) -> usize {1}
      }
    )+
  }
}

macro_rules! recursive_field_count_for_primitives {
  ($($x: ty),+) => {
    $(
      impl RecursiveFieldCount for $x {
        fn recursive_field_count(&self) -> usize {1}
      }
    )+
  }
}

macro_rules! implement {
  ($($x: ty),+) => {
    $(
      field_count_for_primitives!($x);
      recursive_field_count_for_primitives!($x);
    )+
  }
}

pub trait FieldCount {
  fn field_count(&self) -> usize;
}

pub trait RecursiveFieldCount {
  fn recursive_field_count(&self) -> usize;
}

pub trait FieldCountByType<T> {
  fn field_count_by_type(&self) -> usize;
}

pub struct Generic;

implement!(i8, i16, i32, i64, i128, isize);
implement!(u8, u16, u32, u64, u128, usize);
implement!(f32, f64);
implement!(char, bool, ());
implement!(String);

impl<T: FieldCount> FieldCount for Option<T> {
  fn field_count(&self) -> usize {
    match self {
      Some(t) => t.field_count(),
      None => 1,
    }
  }
}

impl<T: RecursiveFieldCount> RecursiveFieldCount for Option<T> {
  fn recursive_field_count(&self) -> usize {
    match self {
      Some(t) => t.recursive_field_count(),
      None => 1,
    }
  }
}
