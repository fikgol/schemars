use crate::gen::{BoolSchemas, SchemaGenerator};
use crate::schema::*;
use crate::{JsonSchema, Result};

macro_rules! map_impl {
    ($($desc:tt)+) => {
        impl $($desc)+
        where
            K: Into<String>,
            V: JsonSchema,
        {
            no_ref_schema!();

            fn schema_name() -> String {
                format!("Map_Of_{}", V::schema_name())
            }

            fn json_schema(gen: &mut SchemaGenerator) -> Result {
                let subschema = gen.subschema_for::<V>()?;
                let json_schema_bool = gen.settings().bool_schemas == BoolSchemas::AdditionalPropertiesOnly
                    && subschema == gen.schema_for_any();
                let additional_properties =
                    if json_schema_bool {
                        true.into()
                    } else {
                        subschema.into()
                    };
                Ok(SchemaObject {
                    instance_type: Some(InstanceType::Object.into()),
                    object: Some(Box::new(ObjectValidation {
                        additional_properties: Some(Box::new(additional_properties)),
                        ..Default::default()
                    })),
                    ..Default::default()
                }.into())
            }
        }
    };
}

map_impl!(<K: Ord, V> JsonSchema for std::collections::BTreeMap<K, V>);
map_impl!(<K: Eq + core::hash::Hash, V, H: core::hash::BuildHasher> JsonSchema for std::collections::HashMap<K, V, H>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen::*;
    use crate::tests::{custom_schema_object_for, schema_for};
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn schema_for_map_any_value() {
        for bool_schemas in &[BoolSchemas::Enable, BoolSchemas::AdditionalPropertiesOnly] {
            let settings = SchemaSettings {
                bool_schemas: *bool_schemas,
                ..Default::default()
            };
            let schema = custom_schema_object_for::<BTreeMap<String, serde_json::Value>>(settings);
            assert_eq!(
                schema.instance_type,
                Some(SingleOrVec::from(InstanceType::Object))
            );
            let additional_properties = schema.object
                .unwrap()
                .additional_properties
                .expect("additionalProperties field present");
            assert_eq!(*additional_properties, Schema::Bool(true));
        }
    }

    #[test]
    fn schema_for_map_any_value_no_bool_schema() {
        let settings = SchemaSettings {
            bool_schemas: BoolSchemas::Disable,
            ..Default::default()
        };
        let schema = custom_schema_object_for::<BTreeMap<String, serde_json::Value>>(settings);
        assert_eq!(
            schema.instance_type,
            Some(SingleOrVec::from(InstanceType::Object))
        );
        let additional_properties = schema.object
            .unwrap()
            .additional_properties
            .expect("additionalProperties field present");
        assert_eq!(*additional_properties, Schema::Object(Default::default()));
    }

    #[test]
    fn schema_for_map_int_value() {
        for bool_schemas in &[
            BoolSchemas::Enable,
            BoolSchemas::Disable,
            BoolSchemas::AdditionalPropertiesOnly,
        ] {
            let settings = SchemaSettings {
                bool_schemas: *bool_schemas,
                ..Default::default()
            };
            let schema = custom_schema_object_for::<BTreeMap<String, i32>>(settings);
            assert_eq!(
                schema.instance_type,
                Some(SingleOrVec::from(InstanceType::Object))
            );
            let additional_properties = schema.object
                .unwrap()
                .additional_properties
                .expect("additionalProperties field present");
            assert_eq!(*additional_properties, schema_for::<i32>());
        }
    }
}
