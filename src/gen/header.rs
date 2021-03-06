use std::io::{Result, Write};

pub(super) fn gen_header<W>(write: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(write, "/* generated by qt_binding_generator */")?;
    Ok(())
}
