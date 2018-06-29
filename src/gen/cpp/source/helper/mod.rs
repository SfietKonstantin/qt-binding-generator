mod complex_type_helper;
mod emitter;

use self::complex_type_helper::{gen_qbytearray_helper, gen_qstring_helper};
use self::emitter::gen_emitters;
use def::Definition;
use gen::core::{apply, join, Generate};
use gen::cpp::utils::definition_properties;
use std::io::{Result, Write};

pub(super) fn gen_helper<W>(write: &mut W, definition: &Definition) -> Result<()>
where
    W: Write,
{
    let properties = definition_properties(definition);

    let helper_begin = |write: &mut W| {
        writeln!(write, "namespace {{")?;
        Ok(())
    };

    let helper_end = |write: &mut W| {
        writeln!(write, "}} // namespace")?;
        Ok(())
    };

    join!(
        helper_begin,
        apply(gen_qstring_helper, &properties),
        apply(gen_qbytearray_helper, &properties),
        apply(gen_emitters, definition),
        helper_end
    ).gen(write)
}
