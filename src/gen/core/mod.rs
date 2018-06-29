mod apply;
mod join;
mod map;
mod scope;

#[macro_use]
mod join_macro;

use std::io::{Result, Write};

pub(in gen) use self::apply::apply;
pub(in gen) use self::join::join;
pub(in gen) use self::map::ListGenerator;
pub(in gen) use self::scope::ScopedGenerate;

pub(in gen) trait Generate<W>
where
    W: Write,
{
    fn gen(&self, write: &mut W) -> Result<()>;
}

impl<W, F> Generate<W> for F
where
    W: Write,
    F: Fn(&mut W) -> Result<()>,
{
    fn gen(&self, write: &mut W) -> Result<()> {
        (self)(write)
    }
}
