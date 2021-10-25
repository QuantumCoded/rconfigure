use far::{far, Errors};
use std::path::{Path, PathBuf};
use std::{collections::hash_map::RandomState, collections::HashMap, fs, iter::FromIterator};

/// Parses a template file and generates tries to generate the completed config file from it
pub fn generate_config<P: AsRef<Path>>(
    path: P,
    map: HashMap<String, String>,
) -> Result<(PathBuf, String), Errors> {
    let data = fs::read_to_string(path.as_ref()).unwrap();
    let mut lines = data.lines();
    let mut template = String::new();

    // the first line of a template is always a path and must be included
    let header = match lines.next().map(|s| Path::new(s)) {
        Some(path) => path,
        None => {
            println!(
                "failed to parse template {:?}, missing header",
                path.as_ref()
            );
            std::process::exit(1);
        }
    };

    // skip all the empty lines following the header
    let first_line = loop {
        if let Some(line) = lines.next() {
            if line.trim() != "" {
                break Some(line);
            }
        } else {
            break None;
        }
    };

    // collect all of the actual template data
    if let Some(first_line) = first_line {
        template.push_str(first_line);

        for line in lines {
            let mut line =  String::from(line);
            line.push_str("\n");

            template.push_str(line.as_str());
        }
    }

    Ok((
        header.to_owned(),
        far(
            template,
            &HashMap::<&str, &str, RandomState>::from_iter(
                map.iter().map(|(k, v)| (k.as_str(), v.as_str())),
            ),
        )?,
    ))
}
