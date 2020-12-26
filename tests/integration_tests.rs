use field_count::{FieldCount, RecursiveFieldCount, FieldCountByType,
  Generic};

// TODO: test pointers

#[derive(FieldCount, RecursiveFieldCount, Default, FieldCountByType)]
struct ExampleNamedStruct {
  field1: i64,
  _field2: Option<bool>,
  _field3: ::std::string::String,
  _field4: Option<i64>,
  _field5: MyResult<String, ()>,
  _field6: MyResult<i64, i64>,
}

enum MyResult<T, U> {
  Ok(T),
  _Err(U),
}

impl<T: Default, U> Default for MyResult<T, U> {
  fn default() -> Self { MyResult::Ok(T::default()) }
}

impl<T, U> RecursiveFieldCount for MyResult<T, U> {
  fn recursive_field_count(&self) -> usize {1}
}

#[derive(FieldCount, RecursiveFieldCount, Default)]
struct ExampleNestedStruct {
  _example_named_struct: ExampleNamedStruct,
  _field2: bool,
  _field3: String,
  _field4: Option<String>,
}

#[derive(FieldCount, Default)]
struct ExampleUnnamedStruct(i64, bool, String, u32, String);

#[derive(FieldCount, Default)]
struct ExampleUnitStruct;

#[test]
fn test_example_named_struct() {
  assert_eq!(ExampleNamedStruct::default().field_count(), 6);
}

#[test]
fn test_example_named_struct_recursive() {
  assert_eq!(ExampleNamedStruct::default().recursive_field_count(), 6);
}

#[test]
fn test_named_struct_by_type() {
  let def = ExampleNamedStruct::default();

  assert_eq!(FieldCountByType::<i64>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<Option<bool>>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<String>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<Option<Generic>>::field_count_by_type(&def), 2);
  assert_eq!(FieldCountByType::<MyResult<String, ()>>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<MyResult<String, Generic>>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<MyResult<Generic, ()>>::field_count_by_type(&def), 1);
  assert_eq!(FieldCountByType::<MyResult<Generic, Generic>>::field_count_by_type(&def), 2);
}

#[test]
fn test_example_nested_struct() {
  assert_eq!(ExampleNestedStruct::default().field_count(), 4);
}

#[test]
fn test_example_nested_struct_recursive() {
  assert_eq!(ExampleNestedStruct::default().recursive_field_count(), 9);
}

#[test]
fn test_example_unnamed_struct() {
  assert_eq!(ExampleUnnamedStruct::default().field_count(), 5);
}

#[test]
fn test_example_unit_struct() {
  assert_eq!(ExampleUnitStruct::default().field_count(), 0);
}

#[test]
fn test_primitive1() {
  assert_eq!(i64::default().field_count(), 1);
  assert_eq!(1.field_count(), 1);
}

#[test]
fn test_primitive2() {
  assert_eq!(().field_count(), 1);
}

#[test]
fn test_primitive3() {
  assert_eq!(String::default().field_count(), 1);
}
