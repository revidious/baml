use std::sync::{Arc, Mutex};
use std::fmt;

use baml_types::{BamlValue, FieldType};
use indexmap::IndexMap;

use crate::runtime_context::{PropertyAttributes, RuntimeClassOverride, RuntimeEnumOverride};

type MetaData = Arc<Mutex<IndexMap<String, BamlValue>>>;

trait Meta {
    fn meta(&self) -> MetaData;
}

pub trait WithMeta {
    fn with_meta(&self, key: &str, value: BamlValue) -> &Self;
}

macro_rules! impl_meta {
    ($type:ty) => {
        impl Meta for $type {
            fn meta(&self) -> MetaData {
                self.meta.clone()
            }
        }
    };
}

impl<T> WithMeta for T
where
    T: Meta,
{
    fn with_meta(&self, key: &str, value: BamlValue) -> &T {
        let meta = self.meta();
        let mut meta = meta.lock().unwrap();
        meta.insert(key.to_string(), value);
        self
    }
}

impl<T: Meta> From<&Arc<Mutex<T>>> for PropertyAttributes {
    fn from(value: &Arc<Mutex<T>>) -> Self {
        let value = value.lock().unwrap();
        let meta = value.meta();
        let meta = meta.lock().unwrap();
        let properties = meta.clone();
        let alias = properties.get("alias").cloned();
        let skip = properties.get("skip").and_then(|v| v.as_bool());

        Self {
            alias,
            skip,
            meta: properties,
        }
    }
}

pub struct ClassBuilder {
    properties: Arc<Mutex<IndexMap<String, Arc<Mutex<ClassPropertyBuilder>>>>>,
    meta: MetaData,
}
impl_meta!(ClassBuilder);

pub struct ClassPropertyBuilder {
    r#type: Arc<Mutex<Option<FieldType>>>,
    meta: MetaData,
}
impl_meta!(ClassPropertyBuilder);

impl ClassPropertyBuilder {
    pub fn r#type(&self, r#type: FieldType) -> &Self {
        *self.r#type.lock().unwrap() = Some(r#type);
        self
    }
}

impl Default for ClassBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassBuilder {
    pub fn new() -> Self {
        Self {
            properties: Default::default(),
            meta: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn property(&self, name: &str) -> Arc<Mutex<ClassPropertyBuilder>> {
        let mut properties = self.properties.lock().unwrap();
        Arc::clone(properties.entry(name.to_string()).or_insert_with(|| {
            Arc::new(Mutex::new(ClassPropertyBuilder {
                r#type: Default::default(),
                meta: Default::default(),
            }))
        }))
    }
}

pub struct EnumBuilder {
    values: Arc<Mutex<IndexMap<String, Arc<Mutex<EnumValueBuilder>>>>>,
    meta: MetaData,
}
impl_meta!(EnumBuilder);

pub struct EnumValueBuilder {
    meta: MetaData,
}
impl_meta!(EnumValueBuilder);

impl Default for EnumBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EnumBuilder {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
            meta: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn value(&self, name: &str) -> Arc<Mutex<EnumValueBuilder>> {
        let mut values = self.values.lock().unwrap();
        Arc::clone(values.entry(name.to_string()).or_insert_with(|| {
            Arc::new(Mutex::new(EnumValueBuilder {
                meta: Default::default(),
            }))
        }))
    }
}

impl fmt::Display for ClassPropertyBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meta = self.meta.lock().unwrap();
        let alias = meta.get("alias").and_then(|v| v.as_string());
        let desc = meta.get("description").and_then(|v| v.as_string());

        write!(f, "{}", self.r#type.lock().unwrap().as_ref().map_or("unset", |_| "set"))?;
        if let Some(alias) = alias {
            write!(f, " (alias='{}'", alias)?;
            if let Some(desc) = desc {
                write!(f, ", desc='{}'", desc)?;
            }
            write!(f, ")")?;
        } else if let Some(desc) = desc {
            write!(f, " (desc='{}')", desc)?;
        }
        Ok(())
    }
}

impl fmt::Display for ClassBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let properties = self.properties.lock().unwrap();
        write!(f, "{{")?;
        if !properties.is_empty() {
            write!(f, " ")?;
            for (i, (name, prop)) in properties.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} {}", name, prop.lock().unwrap())?;
            }
            write!(f, " ")?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for EnumValueBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meta = self.meta.lock().unwrap();
        let alias = meta.get("alias").and_then(|v| v.as_string());
        let desc = meta.get("description").and_then(|v| v.as_string());

        if let Some(alias) = alias {
            write!(f, " (alias='{}'", alias)?;
            if let Some(desc) = desc {
                write!(f, ", desc='{}'", desc)?;
            }
            write!(f, ")")?;
        } else if let Some(desc) = desc {
            write!(f, " (desc='{}')", desc)?;
        }
        Ok(())
    }
}

impl fmt::Display for EnumBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = self.values.lock().unwrap();
        write!(f, "{{")?;
        if !values.is_empty() {
            write!(f, " ")?;
            for (i, (name, value)) in values.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}{}", name, value.lock().unwrap())?;
            }
            write!(f, " ")?;
        }
        write!(f, "}}")
    }
}

/// implements a string representation for typebuilder.
///
/// this implementation provides a clear, hierarchical view of the typebuilder's structure,
/// making it easy to understand the defined types and their metadata at a glance.
///
/// # Format
/// ```text
/// TypeBuilder(
///   Classes: [
///     ClassName {
///       property_name type (alias='custom_name', desc='property description'),
///       another_property type (desc='another description'),
///       simple_property type
///     },
///     EmptyClass { }
///   ],
///   Enums: [
///     EnumName {
///       VALUE (alias='custom_value', desc='value description'),
///       ANOTHER_VALUE (alias='custom'),
///       SIMPLE_VALUE
///     },
///     EmptyEnum { }
///   ]
/// )
/// ```
///
/// # properties shown
/// - class and property names
/// - property types (set/unset)
/// - property metadata (aliases, descriptions)
/// - enum values and their metadata
/// - empty classes and enums
impl fmt::Display for TypeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let classes = self.classes.lock().unwrap();
        let enums = self.enums.lock().unwrap();

        write!(f, "TypeBuilder(")?;

        if !classes.is_empty() {
            write!(f, "Classes: [")?;
            for (i, (name, cls)) in classes.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} {}", name, cls.lock().unwrap())?;
            }
            write!(f, "]")?;
        }

        if !enums.is_empty() {
            if !classes.is_empty() {
                write!(f, ", ")?;
            }
            write!(f, "Enums: [")?;
            for (i, (name, e)) in enums.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} {}", name, e.lock().unwrap())?;
            }
            write!(f, "]")?;
        }

        write!(f, ")")
    }
}

#[derive(Clone)]
pub struct TypeBuilder {
    classes: Arc<Mutex<IndexMap<String, Arc<Mutex<ClassBuilder>>>>>,
    enums: Arc<Mutex<IndexMap<String, Arc<Mutex<EnumBuilder>>>>>,
}

impl Default for TypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeBuilder {
    pub fn new() -> Self {
        Self {
            classes: Default::default(),
            enums: Default::default(),
        }
    }

    pub fn class(&self, name: &str) -> Arc<Mutex<ClassBuilder>> {
        Arc::clone(
            self.classes
                .lock()
                .unwrap()
                .entry(name.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(ClassBuilder::new()))),
        )
    }

    pub fn r#enum(&self, name: &str) -> Arc<Mutex<EnumBuilder>> {
        Arc::clone(
            self.enums
                .lock()
                .unwrap()
                .entry(name.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(EnumBuilder::new()))),
        )
    }

    pub fn to_overrides(
        &self,
    ) -> (
        IndexMap<String, RuntimeClassOverride>,
        IndexMap<String, RuntimeEnumOverride>,
    ) {
        log::debug!("Converting types to overrides");
        let cls = self
            .classes
            .lock()
            .unwrap()
            .iter()
            .map(|(name, cls)| {
                log::debug!("Converting class: {}", name);
                let mut overrides = RuntimeClassOverride {
                    alias: None,
                    new_fields: Default::default(),
                    update_fields: Default::default(),
                };

                cls.lock()
                    .unwrap()
                    .properties
                    .lock()
                    .unwrap()
                    .iter()
                    .for_each(|(property_name, f)| {
                        let attrs = PropertyAttributes::from(f);
                        let t = {
                            let property = f.lock().unwrap();
                            let t = property.r#type.lock().unwrap();
                            t.clone()
                        };
                        match t.as_ref() {
                            Some(r#type) => {
                                overrides
                                    .new_fields
                                    .insert(property_name.to_string(), (r#type.clone(), attrs));
                            }
                            None => {
                                overrides
                                    .update_fields
                                    .insert(property_name.to_string(), attrs);
                            }
                        }
                    });
                (name.clone(), overrides)
            })
            .collect();

        let enm = self
            .enums
            .lock()
            .unwrap()
            .iter()
            .map(|(name, enm)| {
                let attributes = PropertyAttributes::from(enm);
                let values = enm
                    .lock()
                    .unwrap()
                    .values
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(value_name, value)| {
                        (value_name.clone(), PropertyAttributes::from(value))
                    })
                    .collect();
                (
                    name.clone(),
                    RuntimeEnumOverride {
                        values,
                        alias: attributes.alias,
                    },
                )
            })
            .collect();
        log::debug!(
            "Dynamic types: \n {:#?} \n Dynamic enums\n {:#?} enums",
            cls,
            enm
        );
        (cls, enm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_builder() {
        let builder = TypeBuilder::new();

        // Add a class with properties and metadata
        let cls = builder.class("User");
        {
            let cls = cls.lock().unwrap();
            // Add name property with alias and description
            cls.property("name")
                .lock()
                .unwrap()
                .r#type(FieldType::string())
                .with_meta("alias", BamlValue::String("username".to_string()))
                .with_meta("description", BamlValue::String("The user's full name".to_string()));

            // Add age property with description only
            cls.property("age")
                .lock()
                .unwrap()
                .r#type(FieldType::int())
                .with_meta("description", BamlValue::String("User's age in years".to_string()));

            // Add email property with no metadata
            cls.property("email")
                .lock()
                .unwrap()
                .r#type(FieldType::string());
        }

        // Add an enum with values and metadata
        let enm = builder.r#enum("Status");
        {
            let enm = enm.lock().unwrap();
            // Add ACTIVE value with alias and description
            enm.value("ACTIVE")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("active".to_string()))
                .with_meta("description", BamlValue::String("User is active".to_string()));

            // Add INACTIVE value with alias only
            enm.value("INACTIVE")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("inactive".to_string()));

            // Add PENDING value with no metadata
            enm.value("PENDING");
        }

        // Convert to string and verify the format
        let output = builder.to_string();
        assert_eq!(
            output,
            "TypeBuilder(Classes: [User { name set (alias='username', desc='The user\'s full name'), age set (desc='User\'s age in years'), email set }], Enums: [Status { ACTIVE (alias='active', desc='User is active'), INACTIVE (alias='inactive'), PENDING }])"
        );
    }


// my paranoia kicked in, so tis  test is to ensure that the string representation is correct
// and that the to_overrides method is working as expected

    #[test]
    fn test_type_builder_advanced() {
        let builder = TypeBuilder::new();

        // 1. Complex class with nested types and all field types
        let address = builder.class("Address");
        {
            let address = address.lock().unwrap();
            // String with all metadata
            address.property("street")
                .lock()
                .unwrap()
                .r#type(FieldType::string())
                .with_meta("alias", BamlValue::String("streetAddress".to_string()))
                .with_meta("description", BamlValue::String("Street address including number".to_string()));

            // Optional int with description
            address.property("unit")
                .lock()
                .unwrap()
                .r#type(FieldType::int().as_optional())
                .with_meta("description", BamlValue::String("Apartment/unit number if applicable".to_string()));

            // List of strings with alias
            address.property("tags")
                .lock()
                .unwrap()
                .r#type(FieldType::string().as_list())
                .with_meta("alias", BamlValue::String("labels".to_string()));

            // Boolean with no metadata
            address.property("is_primary")
                .lock()
                .unwrap()
                .r#type(FieldType::bool());

            // Float with skip metadata
            address.property("coordinates")
                .lock()
                .unwrap()
                .r#type(FieldType::float())
                .with_meta("skip", BamlValue::Bool(true));
        }

        // 2. Empty class
        builder.class("EmptyClass");

        // 3. Complex enum with various metadata combinations
        let priority = builder.r#enum("Priority");
        {
            let priority = priority.lock().unwrap();
            // All metadata
            priority.value("HIGH")
                .lock()
                .unwrap()
                .with_meta("alias", BamlValue::String("urgent".to_string()))
                .with_meta("description", BamlValue::String("Needs immediate attention".to_string()))
                .with_meta("skip", BamlValue::Bool(false));

            // Only description
            priority.value("MEDIUM")
                .lock()
                .unwrap()
                .with_meta("description", BamlValue::String("Standard priority".to_string()));

            // Only skip
            priority.value("LOW")
                .lock()
                .unwrap()
                .with_meta("skip", BamlValue::Bool(true));

            // No metadata
            priority.value("NONE");
        }

        // 4. Empty enum
        builder.r#enum("EmptyEnum");

        // Test string representation
        let output = builder.to_string();
        assert_eq!(
            output,
            "TypeBuilder(Classes: [Address { street set (alias='streetAddress', desc='Street address including number'), unit set (desc='Apartment/unit number if applicable'), tags set (alias='labels'), is_primary set, coordinates set }, EmptyClass {}], Enums: [Priority { HIGH (alias='urgent', desc='Needs immediate attention'), MEDIUM (desc='Standard priority'), LOW, NONE }, EmptyEnum {}])"
        );

        // Test to_overrides()
        let (classes, enums) = builder.to_overrides();

        // Verify class overrides
        assert_eq!(classes.len(), 2);
        let address_override = classes.get("Address").unwrap();
        assert_eq!(address_override.new_fields.len(), 5); // All fields are new
        assert!(address_override.new_fields.get("street").unwrap().1.alias.is_some());
        assert!(address_override.new_fields.get("coordinates").unwrap().1.skip.unwrap());

        // Verify enum overrides
        assert_eq!(enums.len(), 2);
        let priority_override = enums.get("Priority").unwrap();
        assert_eq!(priority_override.values.len(), 4);
        assert!(priority_override.values.get("HIGH").unwrap().alias.is_some());
        assert!(priority_override.values.get("LOW").unwrap().skip.unwrap());
    }
}
