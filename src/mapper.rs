#![allow(unused)]

use std::{
    fs::{DirEntry, ReadDir},
    io::Error,
    iter::Peekable,
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

#[derive(Debug)]
pub struct Choice {
    name: String,
    path: PathBuf,
    subchoices: Option<Vec<Choice>>,
}

impl Choice {
    fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("Invalid option")
            .strip_suffix(".sh")
            .unwrap_or("Invalid option")
            .to_string();

        let subchoices: Option<Vec<Choice>> =
            if path.with_file_name(&name).exists() && path.with_file_name(&name).is_dir() {
                if let Result::Ok(entries) = path.with_file_name(&name).read_dir() {
                    Self::read_tree(entries).ok()
                } else {
                    None
                }
            } else {
                None
            };

        Self {
            name,
            path,
            subchoices,
        }
    }

    pub fn read_tree(files: ReadDir) -> Result<Vec<Choice>, Error> {
        let mut output: Vec<Choice> = Vec::new();
        let mut files_filtered = files
            .filter(|file| {
                matches!(file,
                Ok(entry)
                if entry.path().is_file()
                && entry.path().extension().unwrap_or_default().eq("sh")
                && entry.path().file_name().unwrap_or_default().as_bytes()[0] != b'.'
                )
            })
            .peekable();
        if files_filtered.peek().is_none() {
            Err(Error::other("Empty Directory"))
        } else {
            for script in files_filtered {
                output.push(Choice::new(script?.path()));
            }

            Ok(output)
        }
    }
}
