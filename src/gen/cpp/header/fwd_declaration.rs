use def::{Definition, ObjectDefinition};
use gen::core::{apply, Generate, ListGenerator};
use std::io::{Result, Write};

pub(super) fn gen_fwd_declarations<W>(write: &mut W, definition: &Definition) -> Result<()>
where
    W: Write,
{
    definition
        .objects()
        .iter()
        .map(|object| apply(gen_fwd_declaration, object))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_fwd_declaration<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    writeln!(write, "class {};", definition.name())?;
    Ok(())
}
