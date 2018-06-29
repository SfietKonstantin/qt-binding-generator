#[macro_use]
mod core;
mod cpp;
mod header;
mod utils;

pub(in gen) use self::header::gen_header;

pub(crate) use self::cpp::{gen_cpp_header, gen_cpp_source};
