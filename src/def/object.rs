use def::{de, /*Function,*/ Property};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Object {
    properties: Vec<Property>,
    // functions: Vec<Function>,
}

impl Object {
    pub(crate) fn properties(&self) -> &[Property] {
        &self.properties
    }

    /*
    pub(crate) fn functions(&self) -> &[Function] {
        &self.functions
    }
    */
}

impl From<de::Object> for Object {
    fn from(object: de::Object) -> Self {
        Object {
            properties: object.properties.into_iter().map(Property::from).collect(),
            // functions: Vec::default(),
        }
    }
}
