mod header;
mod source;

pub(in gen::cpp) mod utils;

pub(crate) use self::header::gen_cpp_header;
pub(crate) use self::source::gen_cpp_source;
