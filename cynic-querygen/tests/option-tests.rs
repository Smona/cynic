use insta::assert_snapshot;

#[test]
fn test_field_overrides() {
    let schema = include_str!("../../schemas/test_cases.graphql");
    let query = r#"
      query MyQuery($input: OneOfObject!) {
        clashes {
          str
          bool
          i32
          u32
        }
      }
    "#;

    let generator = cynic_querygen::Generator::new(schema)
        .expect("schema parse failed")
        .with_override(
            "FieldNameClashes.str",
            "std::collections::HashMap<String, String>",
        )
        .unwrap();

    assert_snapshot!(generator.generate(query).expect("QueryGen Failed"))
}

#[test]
fn test_scalar_type_overrides() {
    let schema = include_str!("../../schemas/test_cases.graphql");
    let query = r#"
      query MyQuery($id: UUID!) {
        bar(id: $id) {
          id
          name
        }
      }
    "#;

    let generator = cynic_querygen::Generator::new(schema)
        .expect("schema parse failed")
        .with_scalar_type("UUID", "my_crate::Uuid");

    assert_snapshot!(generator.generate(query).expect("QueryGen Failed"))
}

#[test]
fn test_field_override_beats_scalar_type_override() {
    // Field-level overrides take precedence over scalar-wide defaults.
    let schema = include_str!("../../schemas/test_cases.graphql");
    let query = r#"
      query MyQuery {
        clashes {
          str
          bool
        }
      }
    "#;

    let generator = cynic_querygen::Generator::new(schema)
        .expect("schema parse failed")
        .with_scalar_type("String", "my_crate::MyString")
        .with_override(
            "FieldNameClashes.str",
            "std::collections::HashMap<String, String>",
        )
        .unwrap();

    assert_snapshot!(generator.generate(query).expect("QueryGen Failed"))
}

#[test]
fn test_scalar_type_overrides_in_input_objects() {
    // Overrides for built-in scalars propagate into input objects and variable structs too.
    let schema = include_str!("../../schemas/test_cases.graphql");
    let query = r#"
      query MyQuery($input: Baz!) {
        clashes {
          str
        }
      }
    "#;

    let generator = cynic_querygen::Generator::new(schema)
        .expect("schema parse failed")
        .with_scalar_types([("String", "my_crate::MyString")]);

    assert_snapshot!(generator.generate(query).expect("QueryGen Failed"))
}
