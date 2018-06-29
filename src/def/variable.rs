#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ComplexCppTypeKind {
    QString,
    QByteArray,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CppTypeKind {
    Primitive,
    Complex(ComplexCppTypeKind),
    Object,
}

impl CppTypeKind {
    pub(crate) fn is_qstring(&self) -> bool {
        match self {
            CppTypeKind::Complex(complex_type_kind) => match complex_type_kind {
                ComplexCppTypeKind::QString => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub(crate) fn is_qbytearray(&self) -> bool {
        match self {
            CppTypeKind::Complex(complex_type_kind) => match complex_type_kind {
                ComplexCppTypeKind::QByteArray => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VariableType {
    cpp_type_kind: CppTypeKind,
    cpp_name: String,
    // rust_name: String,
    // rust_init: String,
}

impl VariableType {
    pub(crate) fn cpp_type_kind(&self) -> &CppTypeKind {
        &self.cpp_type_kind
    }

    pub(crate) fn cpp_name(&self) -> &str {
        &self.cpp_name
    }

    /*
    pub(crate) fn rust_name(&self) -> &str {
        &self.rust_name
    }

    pub(crate) fn rust_init(&self) -> &str {
        &self.rust_init
    }
    */
}

impl From<String> for VariableType {
    fn from(name: String) -> Self {
        match name.as_ref() {
            "bool" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "bool".to_string(),
                // rust_name: "bool".to_string(),
                // rust_init: "true".to_string(),
            },
            "qint8" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "qint8".to_string(),
                // rust_name: "i8".to_string(),
                // rust_init: "0".to_string(),
            },
            "quint8" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "quint8".to_string(),
                // rust_name: "u8".to_string(),
                // rust_init: "0".to_string(),
            },
            "qint16" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "qint16".to_string(),
                // rust_name: "i16".to_string(),
                // rust_init: "0".to_string(),
            },
            "quint16" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "quint16".to_string(),
                // rust_name: "u16".to_string(),
                // rust_init: "0".to_string(),
            },
            "qint32" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "qint32".to_string(),
                // rust_name: "i32".to_string(),
                // rust_init: "0".to_string(),
            },
            "quint32" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "quint32".to_string(),
                // rust_name: "u32".to_string(),
                // rust_init: "0".to_string(),
            },
            "qint64" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "qint64".to_string(),
                // rust_name: "i64".to_string(),
                // rust_init: "0".to_string(),
            },
            "quint64" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "quint64".to_string(),
                // rust_name: "u64".to_string(),
                // rust_init: "0".to_string(),
            },
            "float" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "float".to_string(),
                // rust_name: "f32".to_string(),
                // rust_init: "0.".to_string(),
            },
            "double" => VariableType {
                cpp_type_kind: CppTypeKind::Primitive,
                cpp_name: "double".to_string(),
                // rust_name: "f64".to_string(),
                // rust_init: "0.".to_string(),
            },
            "QString" => VariableType {
                cpp_type_kind: CppTypeKind::Complex(ComplexCppTypeKind::QString),
                cpp_name: "QString".to_string(),
                // rust_name: "String".to_string(),
                // rust_init: "String::new()".to_string(),
            },
            "QByteArray" => VariableType {
                cpp_type_kind: CppTypeKind::Complex(ComplexCppTypeKind::QByteArray),
                cpp_name: "QByteArray".to_string(),
                // rust_name: "Vec<u8>".to_string(),
                // rust_init: "Vec::new()".to_string(),
            },
            _ => VariableType {
                cpp_type_kind: CppTypeKind::Object,
                cpp_name: name.to_string(),
                // rust_name: name.to_string(),
                // rust_init: "".to_string(),
            },
        }
    }
}
