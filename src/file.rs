use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

struct FileErrorWithLine {
    line: usize,
    error_description: String,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct FileErrorCollection(Vec<FileErrorWithLine>);

impl Display for FileErrorCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Errors:")?;
        let FileErrorCollection(file_errors) = self;
        for file_error in file_errors {
            writeln!(
                f,
                "Line {}: {}",
                file_error.line + 1,
                file_error.error_description
            )?;
        }

        Ok(())
    }
}

impl Debug for FileErrorCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <FileErrorCollection as Display>::fmt(self, f)
    }
}

impl Error for FileErrorCollection {}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl FileErrorCollection {
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.to_string()
    }
}

enum FileParseResult<T> {
    FileOk(Vec<T>),
    FileErr(FileErrorCollection),
}

impl<T, E> FromIterator<Result<T, E>> for FileParseResult<T>
where
    E: Display,
{
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        let mut errors: Vec<FileErrorWithLine> = Vec::new();
        let mut result: Vec<T> = Vec::new();
        for (line, item) in iter.into_iter().enumerate() {
            match item {
                Ok(value) => result.push(value),
                Err(error) => errors.push(FileErrorWithLine {
                    line,
                    error_description: error.to_string(),
                }),
            }
        }

        if errors.is_empty() {
            FileParseResult::FileOk(result)
        } else {
            FileParseResult::FileErr(FileErrorCollection(errors))
        }
    }
}

impl<T> From<FileParseResult<T>> for Result<Vec<T>, FileErrorCollection> {
    fn from(value: FileParseResult<T>) -> Self {
        match value {
            FileParseResult::FileOk(result) => Ok(result),
            FileParseResult::FileErr(errors) => Err(errors),
        }
    }
}

pub fn parse_lines<T>(file_contents: String) -> Result<Vec<T>, FileErrorCollection>
where
    T: FromStr,
    T::Err: Display,
{
    file_contents
        .lines()
        .map(|line| -> Result<T, <T as FromStr>::Err> { line.parse::<T>() })
        .collect::<FileParseResult<T>>()
        .into()
}

pub fn parse_optional_lines<T>(file_contents: String) -> Result<Vec<Option<T>>, FileErrorCollection>
where
    T: FromStr,
    T::Err: Display,
{
    file_contents
        .lines()
        .map(|line| -> Result<Option<T>, <T as FromStr>::Err> {
            if line.is_empty() {
                Ok(None)
            } else {
                line.parse::<T>().map(|value| Some(value))
            }
        })
        .collect::<FileParseResult<Option<T>>>()
        .into()
}
