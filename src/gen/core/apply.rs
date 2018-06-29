use super::Generate;
use std::io::{Result, Write};
use std::marker::PhantomData;

pub(in gen) struct ApplyGenerator<'a, W, T, F>
where
    T: 'a,
{
    data: &'a T,
    f: F,
    _phantom: PhantomData<W>,
}

impl<'a, W, T, F> Generate<W> for ApplyGenerator<'a, W, T, F>
where
    W: Write,
    T: 'a,
    F: Fn(&mut W, &T) -> Result<()>,
{
    fn gen(&self, write: &mut W) -> Result<()> {
        (self.f)(write, self.data)
    }
}

pub(in gen) fn apply<W, T, F>(f: F, data: &T) -> ApplyGenerator<W, T, F> {
    ApplyGenerator {
        data,
        f,
        _phantom: PhantomData,
    }
}
