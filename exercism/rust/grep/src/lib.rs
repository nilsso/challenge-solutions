#![feature(stmt_expr_attributes)]
use std::fs::File;
use std::io::Read;

use failure::Error;

#[derive(Default, Debug)]
#[rustfmt::skip]
pub struct Flags {
    pub line_numbers:     bool,
    pub file_names:       bool,
    pub case_insensitive: bool,
    pub inverse_match:    bool,
    pub entire_line:      bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut res: Self = Default::default();

        for &flag in flags {
            #[rustfmt::skip]
            match flag {
                "-n" => res.line_numbers     = true,
                "-l" => res.file_names       = true,
                "-i" => res.case_insensitive = true,
                "-v" => res.inverse_match    = true,
                "-x" => res.entire_line      = true,
                _ => panic!("Unknown flag '{}'", flag),
            }
        }

        res
    }
}

pub fn grep(pattern: &str, flags: &Flags, file_names: &[&str]) -> Result<Vec<String>, Error> {
    let mut files: Vec<(&str, File)> = Vec::new();
    for file_name in file_names {
        files.push((file_name, File::open(file_name)?));
    }

    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_owned()
    };

    let multiple_files = files.len() > 1;
    let mut res = vec![];
    let mut buf = String::new();

    for (name, mut handle) in files {
        buf.clear();
        handle.read_to_string(&mut buf)?;

        for (i, line) in buf
            .split('\n')
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
        {
            let found = {
                let found = if flags.case_insensitive {
                    line.to_lowercase().find(&pattern)
                } else {
                    line.find(&pattern)
                }
                .is_some()
                    && (!flags.entire_line || line.len() == pattern.len());

                found ^ flags.inverse_match
            };

            if found {
                #[rustfmt::skip]
                let res_line = match (flags.file_names, multiple_files, flags.line_numbers) {
                    (true,    _,    _) => name.to_owned(),
                    (   _, true, true) => format!("{}:{}:{}", name, i + 1, line),
                    (   _, true,    _) => format!("{}:{}", name, line),
                    (   _,    _, true) => format!("{}:{}", i + 1, line),
                    (   _,    _,    _) => line.to_owned(),
                };
                res.push(res_line);

                if flags.file_names {
                    break;
                }
            }
        }
    }

    Ok(res)
}
