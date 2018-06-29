use std::collections::BTreeMap;

#[derive(Deserialize)]
pub(super) struct Definition {
    pub(super) objects: BTreeMap<String, ObjectKind>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(super) enum ObjectKind {
    Object(Object),
    List(List),
    Tree(Tree),
}

#[derive(Deserialize)]
pub(super) struct Object {
    pub(super) properties: BTreeMap<String, Property>,
}

#[derive(Deserialize)]
pub(super) struct List {
    pub(super) properties: BTreeMap<String, Property>,
}

#[derive(Deserialize)]
pub(super) struct Tree {
    pub(super) properties: BTreeMap<String, Property>,
}

#[derive(Deserialize)]
pub(super) struct Property {
    #[serde(rename = "type")]
    pub(super) variable_type: String,
    #[serde(default)]
    pub(super) write: bool,
    // #[serde(default)]
    // pub(in super) optional: bool,
    // #[serde(rename = "rustByValue")]
    // #[serde(default)]
    // pub(in super) rust_by_value: bool,
}
