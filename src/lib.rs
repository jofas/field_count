pub use field_count_derive::FieldCount;

macro_rules! field_count_for_primitives {
  ($($x: ty),+) => {
    $(
      impl FieldCount for $x {
        fn field_count() -> usize {1}
      }
    )+
  }
}

pub trait FieldCount {
  fn field_count() -> usize;
}

field_count_for_primitives!(i8, i16, i32, i64, i128, isize);
field_count_for_primitives!(u8, u16, u32, u64, u128, usize);
field_count_for_primitives!(f32, f64);
field_count_for_primitives!(char, bool, ());
field_count_for_primitives!(String);
