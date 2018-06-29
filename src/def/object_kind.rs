use def::{List, Object, Tree};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ObjectKind {
    Object(Object),
    List(List),
    Tree(Tree),
}

impl ObjectKind {
    pub(crate) fn is_object(&self) -> bool {
        match self {
            ObjectKind::Object(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_list(&self) -> bool {
        match self {
            ObjectKind::List(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_tree(&self) -> bool {
        match self {
            ObjectKind::Tree(_) => true,
            _ => false,
        }
    }
}
