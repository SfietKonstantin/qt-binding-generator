use std::io::{Result, Write};

pub(in gen) fn new_line<W>(write: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(write)
}
