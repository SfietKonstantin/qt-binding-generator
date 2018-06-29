use def::{de, Property};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Tree {
    properties: Vec<Property>,
}

impl Tree {
    pub(crate) fn properties(&self) -> &[Property] {
        &self.properties
    }
}

impl From<de::Tree> for Tree {
    fn from(object: de::Tree) -> Self {
        Tree {
            properties: object.properties.into_iter().map(Property::from).collect(),
        }
    }
}
