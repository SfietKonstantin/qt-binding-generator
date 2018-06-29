use super::Generate;
use std::io::{Result, Write};
use std::iter::FromIterator;

pub(in gen) struct ListGenerator<G> {
    generators: Vec<G>,
}

impl<W, G> Generate<W> for ListGenerator<G>
where
    W: Write,
    G: Generate<W>,
{
    fn gen(&self, write: &mut W) -> Result<()> {
        for generator in &self.generators {
            generator.gen(write)?;
        }
        Ok(())
    }
}

impl<G> FromIterator<G> for ListGenerator<G> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = G>,
    {
        ListGenerator {
            generators: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gen::core::apply;

    fn gen_string<W>(write: &mut W, item: &&str) -> Result<()>
    where
        W: Write,
    {
        write.write(item.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_gen_list() {
        let data = vec!["first\n", "second\n", "third\n"];
        let generator = data.iter()
            .map(|data| apply(gen_string, data))
            .collect::<ListGenerator<_>>();

        let mut out = Vec::new();
        generator.gen(&mut out).unwrap();

        let result = String::from_utf8(out).unwrap();
        let expected = "first\nsecond\nthird\n";
        assert_eq!(result, expected);
    }
}
