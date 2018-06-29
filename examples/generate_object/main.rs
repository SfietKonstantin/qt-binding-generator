extern crate qt_binding_generator;
use std::path::Path;

fn main() {
    let out_dir = Path::new("examples/generate_object");
    qt_binding_generator::generate(
        Path::new("examples/generate_object/test_object.json"),
        &out_dir,
    ).unwrap();
}
