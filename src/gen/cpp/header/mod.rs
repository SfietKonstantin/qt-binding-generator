mod definition;
mod fwd_declaration;
mod guard;
mod includes;

use self::definition::gen_definitions;
use self::fwd_declaration::gen_fwd_declarations;
use self::guard::gen_guard;
use self::includes::gen_includes;
use def::Definition;
use gen::core::{apply, join, Generate, ScopedGenerate};
use gen::gen_header;
use gen::utils::new_line;
use std::io::{Result, Write};

pub(crate) fn gen_cpp_header<W>(
    write: &mut W,
    filename: &str,
    definition: &Definition,
) -> Result<()>
where
    W: Write,
{
    let content = join!(
        apply(gen_includes, definition),
        new_line,
        apply(gen_fwd_declarations, definition),
        new_line,
        apply(gen_definitions, definition)
    );
    let generator = gen_guard(filename).nest(content);

    join!(gen_header, generator).gen(write)
}
