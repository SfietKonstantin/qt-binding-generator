mod de;
// mod function;
mod list;
mod object;
mod object_definition;
mod object_kind;
mod property;
mod tree;
mod variable;

use errors::Result;
use serde_json::from_reader;
use std::io::Read;

// pub(crate) use self::function::Function;
pub(crate) use self::list::List;
pub(crate) use self::object::Object;
pub(crate) use self::object_definition::ObjectDefinition;
pub(crate) use self::object_kind::ObjectKind;
pub(crate) use self::property::{AccessKind, /* PresenceKind,*/ Property /*RustAssignmentKind*/,};
pub(crate) use self::tree::Tree;
pub(crate) use self::variable::{CppTypeKind, VariableType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Definition {
    objects: Vec<ObjectDefinition>,
}

impl Definition {
    pub(crate) fn objects(&self) -> &[ObjectDefinition] {
        &self.objects
    }
}

impl Definition {
    pub(crate) fn from_json_reader<R>(reader: R) -> Result<Self>
    where
        R: Read,
    {
        let definition: de::Definition = from_reader(reader)?;
        Ok(Self::from_definition(definition))
    }

    fn from_definition(definition: de::Definition) -> Self {
        let objects = definition
            .objects
            .into_iter()
            .map(ObjectDefinition::from)
            .collect();

        Definition { objects }
    }
}
