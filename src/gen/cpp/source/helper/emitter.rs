use def::{Definition, ObjectDefinition, Property};
use gen::core::{apply, Generate, ListGenerator};
use gen::cpp::utils::{camel_to_pascal, object_properties, pascal_to_camel};
use std::io::{Result, Write};

pub(super) fn gen_emitters<W>(write: &mut W, definition: &Definition) -> Result<()>
where
    W: Write,
{
    definition
        .objects()
        .iter()
        .map(|object| apply(gen_object_emitter, object))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_object_emitter<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    let properties = object_properties(definition);

    let gen_emitter = |write: &mut W, property: &Property| {
        let object_name = pascal_to_camel(definition.name());
        let property_name = camel_to_pascal(property.name());
        write!(write, "    void ")?;
        write!(write, "{}{}Changed", object_name, property_name)?;
        write!(write, "({} *object)", definition.name())?;
        writeln!(write, " {{")?;
        writeln!(write, "        emit object->{}Changed();", property.name())?;
        writeln!(write, "    }}")?;
        Ok(())
    };

    properties
        .iter()
        .map(|property| apply(gen_emitter, property))
        .collect::<ListGenerator<_>>()
        .gen(write)
}
