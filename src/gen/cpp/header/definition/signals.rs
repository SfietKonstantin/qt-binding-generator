use def::{ObjectDefinition, Property};
use gen::core::{apply, Generate, ListGenerator};
use gen::cpp::utils::object_properties;
use std::io::{Result, Write};

pub(super) fn gen_signals<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    let properties = object_properties(definition);
    gen_signals_from_list(write, properties)
}

fn gen_signals_from_list<W>(write: &mut W, properties: &[Property]) -> Result<()>
where
    W: Write,
{
    properties
        .iter()
        .map(|property| apply(gen_signal, property))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_signal<W>(write: &mut W, definition: &Property) -> Result<()>
where
    W: Write,
{
    write!(write, "    void {}Changed();", definition.name())?;
    writeln!(write)?;

    Ok(())
}
