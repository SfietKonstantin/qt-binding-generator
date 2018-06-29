use def::{de, VariableType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AccessKind {
    Read,
    ReadWrite,
}

/*
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PresenceKind {
    Mandatory,
    Optional,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RustAssignmentKind {
    Default,
    ByValue,
}
*/

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Property {
    name: String,
    variable_type: VariableType,
    access: AccessKind,
    // presence: PresenceKind,
    // rust_assignment: RustAssignmentKind,
}

impl Property {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn variable_type(&self) -> &VariableType {
        &self.variable_type
    }

    pub(crate) fn access(&self) -> &AccessKind {
        &self.access
    }

    /*
    pub(crate) fn presence(&self) -> &PresenceKind {
        &self.presence
    }

    pub(crate) fn rust_assignment(&self) -> &RustAssignmentKind {
        &self.rust_assignment
    }
    */
}

impl From<(String, de::Property)> for Property {
    fn from((name, property): (String, de::Property)) -> Self {
        let access = if property.write {
            AccessKind::ReadWrite
        } else {
            AccessKind::Read
        };

        /*
        let presence = if property.optional {
            PresenceKind::Optional
        } else {
            PresenceKind::Mandatory
        };

        let rust_assignment = if property.rust_by_value {
            RustAssignmentKind::ByValue
        } else {
            RustAssignmentKind::Default
        };
        */

        Property {
            name,
            variable_type: VariableType::from(property.variable_type),
            access,
            // presence,
            // rust_assignment,
        }
    }
}
