mod helper;

use def::Definition;
use gen::core::{apply, join, Generate};
use gen::gen_header;
use std::io::{Result, Write};

use self::helper::gen_helper;

pub(crate) fn gen_cpp_source<W>(
    write: &mut W,
    header_filename: &str,
    definition: &Definition,
) -> Result<()>
where
    W: Write,
{
    let include = |write: &mut W| {
        writeln!(write, "#include \"{}\"", header_filename)?;
        writeln!(write)?;
        Ok(())
    };

    join!(gen_header, include, apply(gen_helper, definition)).gen(write)
}
