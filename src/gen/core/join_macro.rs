macro_rules! join {
    ($first:expr, $second:expr) => {
        join($first, $second)
    };
    ($first:expr, $second:expr, $($rest:tt)*) => {
        join!(join($first, $second), $($rest)*)
    };
}

#[cfg(test)]
mod tests {
    use gen::core::{join, Generate};
    use std::io::{Result, Write};

    fn gen_test<W>(write: &mut W) -> Result<()>
    where
        W: Write,
    {
        write!(write, "test")
    }

    #[test]
    fn test_gen_multiple() {
        let mut out = Vec::new();
        join!(gen_test, gen_test, gen_test).gen(&mut out).unwrap();

        let result = String::from_utf8(out).unwrap();
        let expected = "testtesttest";
        assert_eq!(result, expected);
    }
}
