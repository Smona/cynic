use std::{borrow::Cow, collections::HashMap};

use crate::casings::CasingExt;

use crate::processing::{VariableStruct, VariableStructField};
use crate::schema::TypeSpec;
use crate::ScalarTypeMap;

fn type_spec<'a>(
    field: &'a VariableStructField,
    input_objects_need_lifetime: &HashMap<&str, bool>,
    scalar_types: &ScalarTypeMap,
) -> TypeSpec<'a> {
    match field {
        VariableStructField::Variable(var) => TypeSpec::for_executable_type(
            var.ty(),
            input_objects_need_lifetime
                .get(var.ty().name())
                .copied()
                .unwrap_or(false),
            scalar_types.get(var.ty().name()).map(|s| s.as_str()),
        ),
        VariableStructField::NestedStruct { name } => TypeSpec {
            name: Cow::Borrowed(name),
            contains_lifetime_a: input_objects_need_lifetime
                .get(name.as_str())
                .copied()
                .unwrap_or(false),
        },
    }
}

pub struct VariablesStructForDisplay<'v, 'i, 'q, 's> {
    pub variable_struct: &'v VariableStruct<'q>,
    pub input_objects_need_lifetime: &'i HashMap<&'i str, bool>,
    pub scalar_types: &'s ScalarTypeMap,
}

impl std::fmt::Display for VariablesStructForDisplay<'_, '_, '_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use {super::indented, std::fmt::Write};

        writeln!(f, "#[derive(cynic::QueryVariables, Debug)]")?;
        let type_specs: Vec<_> = self
            .variable_struct
            .fields
            .iter()
            .map(|field| type_spec(field, self.input_objects_need_lifetime, self.scalar_types))
            .collect();
        writeln!(
            f,
            "pub struct {}{} {{",
            self.variable_struct.name,
            TypeSpec::lifetime(&type_specs)
        )?;

        for (field, type_spec) in self.variable_struct.fields.iter().zip(type_specs) {
            let name = field.name().to_snake_case();
            let mut display_field = super::Field::new(&name, &type_spec);

            if name.to_camel_case() != field.name() {
                display_field.add_rename(field.name());
            }

            write!(indented(f, 4), "{display_field}",)?;
        }
        writeln!(f, "}}")
    }
}
