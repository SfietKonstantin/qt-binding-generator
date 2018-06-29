use def::Definition;
use gen::cpp::utils::base_class;
use std::io::{Result, Write};

pub(super) fn gen_includes<W>(write: &mut W, definition: &Definition) -> Result<()>
where
    W: Write,
{
    let object_definitions = definition.objects();

    let object = object_definitions
        .iter()
        .find(|definition| definition.kind().is_object());

    if let Some(object) = object {
        writeln!(write, "#include <{}>", base_class(object.kind()))?;
    }

    let model = object_definitions
        .iter()
        .find(|definition| definition.kind().is_list() || definition.kind().is_tree());

    if let Some(model) = model {
        writeln!(write, "#include <{}>", base_class(model.kind()))?;
    }

    Ok(())
}
