use field_count::FieldCount;

#[derive(FieldCount)]
struct ExampleNamedStruct {
  _field1: i64,
  _field2: bool,
  _field3: String,
  _field4: u32,
  _field5: String,
}

#[derive(FieldCount)]
struct ExampleUnnamedStruct(i64, bool, String, u32, String);

#[derive(FieldCount)]
struct ExampleUnitStruct;

#[derive(FieldCount)]
enum ExampleEnum {
  _Variant1,
  _Variant2,
  _Variant3,
}

#[derive(FieldCount)]
union ExampleUnion {
  _field1: i64,
  _field2: u32,
  _field3: bool,
}

#[test]
fn test_example_named_struct() {
  assert_eq!(ExampleNamedStruct::field_count(), 5);
}

#[test]
fn test_example_unnamed_struct() {
  assert_eq!(ExampleUnnamedStruct::field_count(), 5);
}

#[test]
fn test_example_unit_struct() {
  assert_eq!(ExampleUnitStruct::field_count(), 0);
}

#[test]
fn test_example_enum() {
  assert_eq!(ExampleEnum::field_count(), 3);
}

#[test]
fn test_example_union() {
  assert_eq!(ExampleUnion::field_count(), 1);
}
