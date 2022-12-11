use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use crate::contents::convert::contents::ParseContentsError;
use crate::contents::convert::lines::ParseLineError;
use crate::contents::convert::sections::ParseSectionError;

struct FileErrorWithLine {
    line: usize,
    error_description: String,
}

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

pub enum FileParseResult<T> {
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

impl From<FileErrorCollection> for ParseContentsError {
    fn from(collection: FileErrorCollection) -> Self {
        let FileErrorCollection(collection) = collection;
        let line_errors = collection
            .into_iter()
            .map(
                |FileErrorWithLine {
                     line,
                     error_description,
                 }| { ParseLineError::new(line, error_description) },
            )
            .collect::<Vec<ParseLineError>>();
        let section_error = ParseSectionError::new(0, 0, line_errors, None);
        ParseContentsError::new(vec![section_error])
    }
}
