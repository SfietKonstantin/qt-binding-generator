use def::{de, Property};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct List {
    properties: Vec<Property>,
}

impl List {
    pub(crate) fn properties(&self) -> &[Property] {
        &self.properties
    }
}

impl From<de::List> for List {
    fn from(object: de::List) -> Self {
        List {
            properties: object.properties.into_iter().map(Property::from).collect(),
        }
    }
}
