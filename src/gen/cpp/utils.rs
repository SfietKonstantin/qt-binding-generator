use def::{CppTypeKind, Definition, ObjectDefinition, ObjectKind, Property, VariableType};

const QOBJECT: &str = "QObject";
const QABSTRACTITEMMODEL: &str = "QAbstractItemModel";

pub(super) fn camel_to_pascal(name: &str) -> String {
    let upper = name.chars().take(1).map(|c| c.to_ascii_uppercase());
    let rest = name.chars().skip(1);

    upper.chain(rest).collect()
}

pub(super) fn pascal_to_camel(name: &str) -> String {
    let upper = name.chars().take(1).map(|c| c.to_ascii_lowercase());
    let rest = name.chars().skip(1);

    upper.chain(rest).collect()
}

pub(super) fn base_class(kind: &ObjectKind) -> &'static str {
    match kind {
        ObjectKind::Object(_) => QOBJECT,
        _ => QABSTRACTITEMMODEL,
    }
}

pub(super) fn definition_properties(definition: &Definition) -> Vec<&Property> {
    definition
        .objects()
        .iter()
        .flat_map(object_properties)
        .collect::<Vec<_>>()
}

pub(super) fn object_properties(definition: &ObjectDefinition) -> &[Property] {
    match definition.kind() {
        ObjectKind::Object(object) => object.properties(),
        ObjectKind::List(list) => list.properties(),
        ObjectKind::Tree(tree) => tree.properties(),
    }
}

pub(super) fn property_type(variable_type: &VariableType) -> String {
    match variable_type.cpp_type_kind() {
        CppTypeKind::Primitive => variable_type.cpp_name().to_string(),
        CppTypeKind::Complex(_) => variable_type.cpp_name().to_string(),
        CppTypeKind::Object => format!("{} *", variable_type.cpp_name()),
    }
}

pub(super) fn return_type(variable_type: &VariableType) -> String {
    match variable_type.cpp_type_kind() {
        CppTypeKind::Primitive => format!("{} ", variable_type.cpp_name()),
        CppTypeKind::Complex(_) => format!("{} ", variable_type.cpp_name()),
        CppTypeKind::Object => format!("{} * ", variable_type.cpp_name()),
    }
}

pub(super) fn setter_type(variable_type: &VariableType) -> String {
    match variable_type.cpp_type_kind() {
        CppTypeKind::Primitive => format!("{} ", variable_type.cpp_name()),
        CppTypeKind::Complex(_) => format!("const {} &", variable_type.cpp_name()),
        CppTypeKind::Object => format!("{} *", variable_type.cpp_name()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_pascal() {
        assert_eq!(camel_to_pascal("test"), "Test".to_string());
        assert_eq!(camel_to_pascal("Test"), "Test".to_string());
        assert_eq!(
            camel_to_pascal("testSomething"),
            "TestSomething".to_string()
        );
        assert_eq!(camel_to_pascal(""), "".to_string());
    }

    #[test]
    fn test_pascal_to_camel() {
        assert_eq!(pascal_to_camel("test"), "test".to_string());
        assert_eq!(pascal_to_camel("Test"), "test".to_string());
        assert_eq!(
            pascal_to_camel("TestSomething"),
            "testSomething".to_string()
        );
        assert_eq!(pascal_to_camel(""), "".to_string());
    }
}
