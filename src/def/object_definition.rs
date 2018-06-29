use def::{de, List, Object, ObjectKind, Tree};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ObjectDefinition {
    name: String,
    kind: ObjectKind,
}

impl ObjectDefinition {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn kind(&self) -> &ObjectKind {
        &self.kind
    }
}

impl From<(String, de::ObjectKind)> for ObjectDefinition {
    fn from((name, object): (String, de::ObjectKind)) -> Self {
        let kind = match object {
            de::ObjectKind::Object(object) => ObjectKind::Object(Object::from(object)),
            de::ObjectKind::List(list) => ObjectKind::List(List::from(list)),
            de::ObjectKind::Tree(tree) => ObjectKind::Tree(Tree::from(tree)),
        };

        ObjectDefinition { name, kind }
    }
}
