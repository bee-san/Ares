// build.rs
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let dictionary = include_str!("./dictionaries/words.txt");

    let mut builder = phf_codegen::Set::new();

    dictionary.split(' ').for_each(|word| {
        builder.entry(word);
    });

    let hashset_as_string = format!("{}" , builder.build());

    writeln!(
        &mut file,
         "static STORAGE: phf::Map<&'static str, phf::Set<&'static str>> = \n{};\n",
         phf_codegen::Map::new()
             .entry("Dictionaries", &hashset_as_string)
             .build()
    ).unwrap();
}