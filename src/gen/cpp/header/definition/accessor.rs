use def::{AccessKind, ObjectDefinition, Property};
use gen::core::{apply, Generate, ListGenerator};
use gen::cpp::utils::{camel_to_pascal, object_properties, return_type, setter_type};
use std::io::{Result, Write};

pub(super) fn gen_accessors<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    let properties = object_properties(definition);
    gen_accessors_from_list(write, properties)
}

fn gen_accessors_from_list<W>(write: &mut W, properties: &[Property]) -> Result<()>
where
    W: Write,
{
    properties
        .iter()
        .map(|property| apply(gen_accessor, property))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_accessor<W>(write: &mut W, definition: &Property) -> Result<()>
where
    W: Write,
{
    write!(write, "    {}", return_type(definition.variable_type()))?;
    write!(write, "{}() const;", definition.name())?;
    writeln!(write)?;

    gen_setter(write, definition)?;

    Ok(())
}

fn gen_setter<W>(write: &mut W, definition: &Property) -> Result<()>
where
    W: Write,
{
    match definition.access() {
        AccessKind::ReadWrite => {
            write!(write, "    void set{}(", camel_to_pascal(definition.name()))?;
            write!(write, "{}", setter_type(definition.variable_type()))?;
            write!(write, "{});", definition.name())?;
            writeln!(write)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
