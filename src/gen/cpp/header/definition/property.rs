use def::{AccessKind, ObjectDefinition, Property};
use gen::core::{apply, Generate, ListGenerator};
use gen::cpp::utils::{camel_to_pascal, object_properties, property_type};
use std::io::{Result, Write};

pub(super) fn gen_properties<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    let properties = object_properties(definition);
    gen_properties_from_list(write, properties)
}

fn gen_properties_from_list<W>(write: &mut W, properties: &[Property]) -> Result<()>
where
    W: Write,
{
    properties
        .iter()
        .map(|property| apply(gen_property, property))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_property<W>(write: &mut W, definition: &Property) -> Result<()>
where
    W: Write,
{
    write!(write, "    Q_PROPERTY(")?;
    write!(write, "{} ", property_type(definition.variable_type()))?;
    write!(write, "READ {} ", definition.name())?;
    gen_write_property(write, definition)?;
    write!(write, "NOTIFY {}Changed ", definition.name())?;
    write!(write, "FINAL)")?;
    writeln!(write)?;

    Ok(())
}

fn gen_write_property<W>(write: &mut W, definition: &Property) -> Result<()>
where
    W: Write,
{
    match definition.access() {
        AccessKind::ReadWrite => write!(write, "WRITE set{} ", camel_to_pascal(definition.name())),
        _ => Ok(()),
    }
}
