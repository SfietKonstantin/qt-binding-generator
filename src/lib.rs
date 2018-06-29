#![allow(renamed_and_removed_lints)] // Disable for error_chain

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod def;
mod errors;
mod gen;

use self::def::Definition;
use self::gen::{gen_cpp_header, gen_cpp_source};
use errors::Result;
use std::fs::File;
use std::path::Path;

const HEADER_NAME: &str = "bindings.hpp";
const SOURCE_NAME: &str = "bindings.cpp";

pub fn generate(input_file: &Path, output_dir: &Path) -> Result<()> {
    let file = File::open(input_file)?;
    let definition = Definition::from_json_reader(file)?;

    gen_header(&definition, output_dir)?;
    gen_source(&definition, output_dir)?;

    Ok(())
}

fn gen_header(definition: &Definition, output_dir: &Path) -> Result<()> {
    let cpp_header_path = output_dir.join(HEADER_NAME);
    let mut cpp_header = File::create(cpp_header_path)?;

    gen_cpp_header(&mut cpp_header, HEADER_NAME, definition)?;

    Ok(())
}

fn gen_source(definition: &Definition, output_dir: &Path) -> Result<()> {
    let cpp_source_path = output_dir.join(SOURCE_NAME);
    let mut cpp_source = File::create(cpp_source_path)?;

    gen_cpp_source(&mut cpp_source, HEADER_NAME, definition)?;

    Ok(())
}
