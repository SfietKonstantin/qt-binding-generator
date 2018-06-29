use super::Generate;
use std::io::{Result, Write};
use std::marker::PhantomData;

pub(in gen) trait ScopedGenerate<W>
where
    W: Write,
{
    fn gen_begin(&self, write: &mut W) -> Result<()>;
    fn gen_end(&self, write: &mut W) -> Result<()>;

    fn nest<G>(self, generator: G) -> Nest<W, Self, G>
    where
        Self: Sized,
        G: Generate<W>,
    {
        Nest {
            scoped_generator: self,
            generator,
            _phantom: PhantomData,
        }
    }
}

pub(in gen) struct Nest<W, S, G>
where
    W: Write,
    S: ScopedGenerate<W>,
    G: Generate<W>,
{
    scoped_generator: S,
    generator: G,
    _phantom: PhantomData<W>,
}

impl<W, S, G> Generate<W> for Nest<W, S, G>
where
    W: Write,
    S: ScopedGenerate<W>,
    G: Generate<W>,
{
    fn gen(&self, write: &mut W) -> Result<()> {
        self.scoped_generator.gen_begin(write)?;
        self.generator.gen(write)?;
        self.scoped_generator.gen_end(write)?;
        Ok(())
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

    struct TagGenerator {
        tag: String,
    }

    impl TagGenerator {
        fn new<S>(tag: S) -> Self
        where
            S: ToString,
        {
            TagGenerator {
                tag: tag.to_string(),
            }
        }
    }

    impl<W> ScopedGenerate<W> for TagGenerator
    where
        W: Write,
    {
        fn gen_begin(&self, write: &mut W) -> Result<()> {
            write!(write, "<{}>", &self.tag)
        }
        fn gen_end(&self, write: &mut W) -> Result<()> {
            write!(write, "</{}>", &self.tag)
        }
    }

    #[test]
    fn test_gen_nested() {
        let mut out = Vec::new();
        TagGenerator::new("test")
            .nest(gen_test)
            .gen(&mut out)
            .unwrap();

        let result = String::from_utf8(out).unwrap();
        let expected = "<test>test</test>";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_gen_more_nested() {
        let mut out = Vec::new();
        TagGenerator::new("test")
            .nest(TagGenerator::new("other").nest(gen_test))
            .gen(&mut out)
            .unwrap();

        let result = String::from_utf8(out).unwrap();
        let expected = "<test><other>test</other></test>";
        assert_eq!(result, expected);
    }
}
