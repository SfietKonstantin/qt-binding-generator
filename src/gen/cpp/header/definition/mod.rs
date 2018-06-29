mod accessor;
mod property;
mod signals;

use self::accessor::gen_accessors;
use self::property::gen_properties;
use self::signals::gen_signals;
use def::{Definition, ObjectDefinition};
use gen::core::{apply, join, Generate, ListGenerator};
use gen::cpp::utils::base_class;
use std::io::{Result, Write};

pub(super) fn gen_definitions<W>(write: &mut W, definition: &Definition) -> Result<()>
where
    W: Write,
{
    definition
        .objects()
        .iter()
        .map(|object| apply(gen_definition, object))
        .collect::<ListGenerator<_>>()
        .gen(write)
}

fn gen_definition<W>(write: &mut W, definition: &ObjectDefinition) -> Result<()>
where
    W: Write,
{
    let name = definition.name();
    let base_class = base_class(definition.kind());

    let class_header = |write: &mut W| {
        writeln!(write, "class {}: public {}", name, base_class)?;
        writeln!(write, "{{")?;
        writeln!(write, "    Q_OBJECT")?;
        Ok(())
    };

    let properties = apply(gen_properties, definition);

    let class_default_methods = |write: &mut W| {
        writeln!(write, "public:")?;
        writeln!(write, "    class Private;")?;
        writeln!(write, "public:")?;
        writeln!(write, "    explicit {}(QObject *parent = nullptr);", name)?;
        writeln!(write, "    ~{}();", name)?;
        Ok(())
    };

    let accessors = apply(gen_accessors, definition);

    let class_signals = |write: &mut W| {
        writeln!(write, "signals:")?;
        Ok(())
    };

    let signals = apply(gen_signals, definition);

    let class_footer = |write: &mut W| {
        writeln!(write, "private:")?;
        writeln!(write, "    Private *m_d;")?;
        writeln!(write, "}};")?;
        Ok(())
    };

    join!(
        class_header,
        properties,
        class_default_methods,
        accessors,
        class_signals,
        signals,
        class_footer
    ).gen(write)
}
