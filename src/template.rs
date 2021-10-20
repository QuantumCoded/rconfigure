use std::{collections::HashMap, hash::BuildHasher, path::Path, fs};
use far::{far, Errors};

pub fn generate_config<P, H>(path: P, map: &HashMap<&str, &str, H>) -> Result<String, Errors>
where
    P: AsRef<Path>,
    H: BuildHasher,
{
    let path = path.as_ref();
    let template = fs::read_to_string(path).unwrap();

    // TODO: parse the template header and through an error if it's not found

    far(template, map)
}
