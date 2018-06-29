use gen::core::ScopedGenerate;
use std::io::{Result, Write};

pub(super) struct GenerateGuard {
    guard: String,
}

pub(super) fn gen_guard(filename: &str) -> GenerateGuard {
    let guard = filename.replace(".", "_").to_uppercase();
    GenerateGuard { guard }
}

impl<W> ScopedGenerate<W> for GenerateGuard
where
    W: Write,
{
    fn gen_begin(&self, write: &mut W) -> Result<()> {
        writeln!(write, "#ifndef {}", self.guard)?;
        writeln!(write, "#define {}", self.guard)?;
        writeln!(write)?;
        Ok(())
    }

    fn gen_end(&self, write: &mut W) -> Result<()> {
        writeln!(write)?;
        writeln!(write, "#endif // {}", self.guard)?;
        Ok(())
    }
}
