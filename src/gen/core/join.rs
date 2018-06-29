use super::Generate;
use std::io::{Result, Write};
use std::marker::PhantomData;

pub(in gen) struct Join<W, G1, G2>
where
    W: Write,
    G1: Generate<W>,
    G2: Generate<W>,
{
    first: G1,
    second: G2,
    _phantom: PhantomData<W>,
}

impl<W, G1, G2> Generate<W> for Join<W, G1, G2>
where
    W: Write,
    G1: Generate<W>,
    G2: Generate<W>,
{
    fn gen(&self, write: &mut W) -> Result<()> {
        self.first.gen(write)?;
        self.second.gen(write)?;
        Ok(())
    }
}

pub(in gen) fn join<W, G1, G2>(first: G1, second: G2) -> Join<W, G1, G2>
where
    W: Write,
    G1: Generate<W>,
    G2: Generate<W>,
{
    Join {
        first,
        second,
        _phantom: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test<W>(write: &mut W) -> Result<()>
    where
        W: Write,
    {
        write!(write, "test")
    }

    #[test]
    fn test_gen_pair() {
        let mut out = Vec::new();
        join(gen_test, gen_test).gen(&mut out).unwrap();

        let result = String::from_utf8(out).unwrap();
        let expected = "testtest";
        assert_eq!(result, expected);
    }
}
